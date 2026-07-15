use crate::dynamics::FlightState;

use nalgebra::{Vector2, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct FlightObservation {
    initial_state: FlightState,
    state: FlightState,
    wind: Vector3<f64>
}

impl FlightObservation {
    pub fn from_state(initial_state: FlightState, state: FlightState, wind: Vector3<f64>) -> Self {
         Self {
             initial_state,
             state,
             wind
         }
     }

    // sensed directly
    pub fn altitude(&self) -> f64 {
        // sensor: barometric altimeter
        return self.state.position.z;
    }

    pub fn ground_speed(&self) -> f64 {
        Vector2::new(self.state.velocity.x, self.state.velocity.y).norm()
    }

    pub fn airspeed(&self) -> f64 {
        (self.state.velocity - self.wind).norm()

    }

    // derived
    pub fn gamma(&self) -> f64 {
        self.state.velocity.z.atan2(self.ground_speed())
    }

    pub fn distance(&self) -> f64 {
        (Vector2::new(self.state.position.x, self.state.position.y) - Vector2::new(self.initial_state.position.x, self.initial_state.position.y)).norm()
    }
}


#[derive(Debug, Clone, Copy)]
pub struct FlightAction {
    pub alpha: f64, // rad, angle of attack
}

impl FlightAction {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct MinimumSinkRateController {
    target_airspeed: f64,
    max_alpha: f64,
    min_alpha: f64,
    k_wind: f64,
    k_airspeed: f64,
}

impl MinimumSinkRateController {
    pub fn new(optimal_airspeed: f64) -> Self {
        Self {
            target_airspeed: optimal_airspeed,
            max_alpha: 0.26,  // rad
            min_alpha: -0.09, // rad
            k_wind: 0.4,
            k_airspeed: 0.02,
        }
    }

    pub fn optimal_alpha_min_sink(&self, airspeed: f64, wind_z: f64) -> f64 {
        let w = self.target_airspeed - self.k_wind * wind_z;
        let a = -self.k_airspeed * (airspeed - w);

        a.min(self.max_alpha).max(self.min_alpha)
    }
}
 
