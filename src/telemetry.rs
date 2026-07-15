use std::sync::atomic::AtomicUsize;

use crate::atmosphere::Atmosphere;
use crate::dynamics::{FlightDynamics, FlightState};
use polars::prelude::*;

pub trait Frameable {
    fn to_dataframe(data: &[Self]) -> DataFrame
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
pub struct FlightTelemetry {
    time: f64,
    state: FlightState,
}

impl FlightTelemetry {
    pub fn from(state: FlightState) -> Self {
        Self {
            time: state.time,
            state: state,
        }
    }
}

impl Frameable for FlightTelemetry {
    fn to_dataframe(data: &[Self]) -> DataFrame {
        let time: Vec<f64> = data.iter().map(|d| d.time).collect();
        let gamma: Vec<f64> = data.iter().map(|d| d.state.gamma()).collect();
        let x: Vec<f64> = data.iter().map(|d| d.state.position.x).collect();
        let y: Vec<f64> = data.iter().map(|d| d.state.position.y).collect();
        let z: Vec<f64> = data.iter().map(|d| d.state.position.z).collect();
        let v_x: Vec<f64> = data.iter().map(|d| d.state.velocity.x).collect();
        let v_y: Vec<f64> = data.iter().map(|d| d.state.velocity.y).collect();
        let v_z: Vec<f64> = data.iter().map(|d| d.state.velocity.z).collect();

        df! {
            "time" => time,
            "gamma" => gamma,
            "x" => x,
            "y" => y,
            "z" => z,
            "v_x" => v_x,
            "v_y" => v_y,
            "v_z" => v_z
        }
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsTelemetry {
    time: f64,
    pe: f64, // potential energy, PE
    ke: f64, // kinetic energy, KE
    ma: f64, // mach number, Ma
}

impl PhysicsTelemetry {
    pub fn from(state: FlightState, dynamics: &FlightDynamics) -> Self {
        Self {
            time: state.time,
            pe: dynamics.potential_energy(state.altitude()),
            ke: dynamics.kinetic_energy(state.velocity.magnitude()),
            ma: dynamics.mach(state.velocity.magnitude()),
        }
    }
}

impl Frameable for PhysicsTelemetry {
    fn to_dataframe(data: &[Self]) -> DataFrame {
        let time: Vec<f64> = data.iter().map(|d| d.time).collect();
        let pe: Vec<f64> = data.iter().map(|d| d.pe).collect();
        let ke: Vec<f64> = data.iter().map(|d| d.ke).collect();
        let ma: Vec<f64> = data.iter().map(|d| d.ma).collect();

        df! {
            "time" => time,
            "pe" => pe,
            "ke" => ke,
            "ma" => ma
        }
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct WeatherTelemetry {
    time: f64,
    air_temp: f64,
}

impl WeatherTelemetry {
    pub fn from(state: FlightState, atmosphere: &Atmosphere) -> Self {
        Self {
            time: state.time,
            air_temp: atmosphere.air_temperature(state.altitude()),
        }
    }
}

impl Frameable for WeatherTelemetry {
    fn to_dataframe(data: &[Self]) -> DataFrame {
        let time: Vec<f64> = data.iter().map(|d| d.time).collect();
        let air_temp: Vec<f64> = data.iter().map(|d| d.air_temp).collect();

        df! {
            "time" => time,
            "air_temp" => air_temp,
        }
        .unwrap()
    }
}

// channel

pub struct Channel<T> {
    tlm_queue: Vec<T>,
    batch_size: usize,
    batch_index: AtomicUsize,
    base_url: String,
}

impl<T> Channel<T> {
    pub fn new(base_url: String, batch_size: usize) -> Self {
        Self {
            tlm_queue: Vec::with_capacity(batch_size),
            batch_size,
            batch_index: AtomicUsize::new(0),
            base_url,
        }
    }

    pub fn write(&mut self, tlm: T)
    where
        T: Frameable,
    {
        self.tlm_queue.push(tlm);

        if self.tlm_queue.len() >= self.batch_size {
            self.dispatch();
        }
    }

    fn dispatch(&mut self)
    where
        T: Frameable,
    {
        if self.tlm_queue.is_empty() {
            return;
        }

        let df = T::to_dataframe(&self.tlm_queue);

        let batch_num = self
            .batch_index
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let url = format!("{}_{:06}.parquet", self.base_url, batch_num);

        let mut file = std::fs::File::create(&url).unwrap();
        ParquetWriter::new(&mut file)
            .finish(&mut df.clone())
            .unwrap();

        self.tlm_queue.clear();
    }

    pub fn close(&mut self)
    where
        T: Frameable,
    {
        self.dispatch();
    }
}
