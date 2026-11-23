//! Physical body data and update formulas, independent of Bevy.

// Two-dimensional point-mass model; does not represent radius or collisions.
pub(crate) struct CelestialBody {
    // SI units: mass in kg, position in m, velocity in m/s, acceleration in m/s².
    // Pairs store (x, y); f64 retains more precision than the f32 used for rendering.
    pub(crate) mass: f64,
    pub(crate) position: (f64, f64),
    pub(crate) velocity: (f64, f64),
    pub(crate) acceleration: (f64, f64),
}

impl CelestialBody {
    // Velocity Verlet position step: r_new = r + v*dt + a*dt²/2.
    // From a Taylor expansion through acceleration; approximate when acceleration varies.
    // `seconds` is dt in s; all three terms in the sum have units of metres.
    // `&self` borrows the body for reading; returns a new position without changing it.
    pub(crate) fn new_position(&self, seconds: f64) -> (f64, f64) {
        (
            self.position.0
                + self.velocity.0 * seconds
                + 0.5 * self.acceleration.0 * (seconds.powi(2)),
            self.position.1
                + self.velocity.1 * seconds
                + 0.5 * self.acceleration.1 * (seconds.powi(2)),
        )
    }

    // Integrate dv/dt = a using the mean acceleration at the step's endpoints:
    // v_new = v + (a_initial + a_final)*dt/2. The result has units of m/s.
    pub(crate) fn new_velocity(&self, new_acceleration: (f64, f64), seconds: f64) -> (f64, f64) {
        (
            self.velocity.0 + ((self.acceleration.0 + new_acceleration.0) * 0.5) * seconds,
            self.velocity.1 + ((self.acceleration.1 + new_acceleration.1) * 0.5) * seconds,
        )
    }
}
