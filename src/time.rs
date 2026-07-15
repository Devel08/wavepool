#[derive(Clone)]
pub struct Timestamp {
    pub elapsed_sols: f64,
    pub local_solar_time: f64,
}

pub struct Clock {
    sol: f64,
    h_per_sol: f64,
}

// sol clock

impl Clock {
    pub fn new(h_per_sol: f64) -> Self {
        Self {
            sol: 0.0,
            h_per_sol,
        }
    }

    pub fn elapsed_sols(&self) -> f64 {
        self.sol
    }

    pub fn local_solar_time(&self) -> f64 {
        (self.sol % 1.0) * self.h_per_sol
    }

    pub fn increment(&mut self, dt: f64) {
        self.sol += dt / (self.h_per_sol * 60.0 * 60.0);
    }

    pub fn print(&self) {
        println!(
            "Sol {:.0} is {:.2}% completed.",
            self.sol.floor(),
            self.sol % 1.0 * 100.0
        );
    }

    pub fn now(&self) -> Timestamp {
        Timestamp {
            elapsed_sols: self.sol,
            local_solar_time: self.local_solar_time(),
        }
    }
}

// benchmark

use std::time::{Duration,Instant};

pub struct Benchmark {
    t_s: Instant,
}

impl Benchmark {

    pub fn start() -> Self {
        Self {
            t_s: Instant::now(),
        }
    }

    pub fn stop(self, sim_duration: Duration) {
        let t_f = Instant::now();
        println!("Simulated {:.1} minutes in {:.1} minutes.", sim_duration.as_secs_f64() / 60.0, t_f.duration_since(self.t_s).as_secs_f64() / 60.0);
    }
}
