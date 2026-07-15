use nalgebra::{Vector2, Vector3};

use crate::aircraft::Aircraft;
use crate::atmosphere::Planet;

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct FlightState {
    // SI unit, symbol or state
    pub time: f64,              // s, t
    pub position: Vector3<f64>, // m, (distance, ?, altitude)
    pub velocity: Vector3<f64>, // m/s, ground speed
}

impl FlightState {
    pub fn altitude(&self) -> f64 {
        self.position.z
    }

    pub fn distance(&self) -> f64 {
        Vector2::new(self.position.x, self.position.y).norm()
    }

    pub fn gamma(&self) -> f64 {
        self.velocity.z.atan2(self.ground_speed())
    }

    pub fn ground_speed(&self) -> f64 {
        Vector2::new(self.velocity.x, self.velocity.y).norm()
    }
}

#[derive(Debug, Clone, Copy)]
struct Numerics {
    dp_dt: Vector3<f64>, // change in position
    dv_dt: Vector3<f64>, // change in velocity
    dt_dt: f64,          // change in time
}

fn apply_derivatives(derivatives: Numerics, state: FlightState, dt: f64) -> FlightState {
    FlightState {
        position: state.position + dt * derivatives.dp_dt,
        velocity: state.velocity + dt * derivatives.dv_dt,
        time: state.time + dt * derivatives.dt_dt,
    }
}

pub struct FlightDynamics {
    aircraft: Aircraft,
    target: Planet,
}

impl FlightDynamics {
    pub fn new(aircraft: &Aircraft, planet: &Planet) -> Self {
        Self {
            aircraft: aircraft.clone(),
            target: planet.clone(),
        }
    }

    fn orthogonal_lift_dir(&self, v_air: Vector3<f64>) -> Vector3<f64> {
        let up = Vector3::new(0.0, 0.0, 1.0);

        if v_air.norm_squared() < 1.0e-12 {
            return up;
        }

        let lift_raw = v_air.cross(&up).cross(&v_air);

        if lift_raw.norm_squared() < 1.0e-12 {
            up
        } else {
            lift_raw.normalize()
        }
    }

    fn compute_derivatives(
        &mut self,
        state: FlightState,
        wind_velocity: Vector3<f64>,
        alpha: f64,
    ) -> Numerics {
        // alpha is angle of attack and the control input

        if state.altitude() <= 0.0 {
            return Numerics {
                dp_dt: Vector3::new(0.0, 0.0, 0.0),
                dv_dt: Vector3::new(0.0, 0.0, 0.0),
                dt_dt: 1.0,
            };
        }

        let v_air = state.velocity - wind_velocity;
        let v_air_norm = v_air.norm();

        // lift and drag
        let (lift, drag) = if v_air_norm > 1.0e-6 {
            self.aircraft
                .lift_and_drag(state.altitude(), v_air_norm, self.target.atmosphere, alpha)
        } else {
            (0.0, 0.0) // no lift or drag
        };

        // drag vector
        let v_drag = if v_air_norm > 1.0e-6 {
            -v_air.normalize() * drag
        } else {
            Vector3::zeros()
        };

        let lift_dir = self.orthogonal_lift_dir(v_air);
        let v_lift = lift_dir * lift;

        // load factor
        let weight = self.aircraft.mass * self.target.g;
        let n = v_lift.norm() / weight;
        let n_limited = self.aircraft.limit_load_factor(n);
        let lift_limited = n_limited * weight;

        let lift_magnitude = v_lift.norm();
        let lift_scale = if lift_magnitude > 1e-6 {
            lift_limited / lift_magnitude
        } else {
            0.0
        };
        let f_aero_limited = Vector3::new(
            v_lift.x * lift_scale + v_drag.x,
            v_lift.y * lift_scale + v_drag.y,
            v_lift.z * lift_scale + v_drag.z,
        );
        let f = Vector3::new(
            f_aero_limited.x,
            f_aero_limited.y,
            f_aero_limited.z - weight,
        );

        // Coriolis parameter
        let latitude = -60.0_f64.to_radians(); // TODO: derive from FlightState
        let omega = 2.0 * PI / (self.target.sol * 60.0 * 60.0);
        let f_omega = 2.0 * omega * latitude.sin();

        let a_coriolis = Vector3::new(
          f_omega * state.velocity.y,
          -f_omega * state.velocity.x,
          0.0
        );

        let a = f / self.aircraft.mass + a_coriolis;
        let dp_dt = state.velocity;

        Numerics {
            dp_dt,
            dv_dt: a,
            dt_dt: 1.0,
        }
    }

    pub fn step_forward(
        &mut self,
        state: FlightState,
        wind: Vector3<f64>,
        input: f64,
        dt: f64,
    ) -> FlightState {
        // Runge-Kutta 4th-order integration

        let k1 = self.compute_derivatives(state, wind, input);

        let mut s = apply_derivatives(k1, state, 0.5 * dt);
        let k2 = self.compute_derivatives(s, wind, input);

        s = apply_derivatives(k2, state, 0.5 * dt);
        let k3 = self.compute_derivatives(s, wind, input);

        s = apply_derivatives(k3, state, dt);
        let k4 = self.compute_derivatives(s, wind, input);

        let derivatives = Numerics {
            dp_dt: k1.dp_dt + 2.0 * k2.dp_dt + 2.0 * k3.dp_dt + k4.dp_dt,
            dv_dt: k1.dv_dt + 2.0 * k2.dv_dt + 2.0 * k3.dv_dt + k4.dv_dt,
            dt_dt: k1.dt_dt + 2.0 * k2.dt_dt + 2.0 * k3.dt_dt + k4.dt_dt,
        };

        apply_derivatives(derivatives, state, dt / 6.0)
    }

    pub fn kinetic_energy(&self, airspeed: f64) -> f64 {
        0.5 * self.aircraft.mass * airspeed * airspeed
    }

    pub fn potential_energy(&self, altitude: f64) -> f64 {
        self.aircraft.mass * self.target.g * altitude
    }

    pub fn mach(&self, airspeed: f64) -> f64 {
        self.target.atmosphere.mach(airspeed)
    }
}
