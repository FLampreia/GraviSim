pub(crate) struct CelestialBody {
    // SI units: mass in kg, position in m, velocity in m/s, acceleration in m/s².
    pub(crate) mass: f64,
    pub(crate) position: (f64, f64),
    pub(crate) velocity: (f64, f64),
    pub(crate) acceleration: (f64, f64),
}

impl CelestialBody {
    // Return the next position without changing the body's current state.
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

    // Return the next velocity using the mean of the initial and final accelerations.
    pub(crate) fn new_velocity(&self, new_acceleration: (f64, f64), seconds: f64) -> (f64, f64) {
        (
            self.velocity.0 + ((self.acceleration.0 + new_acceleration.0) * 0.5) * seconds,
            self.velocity.1 + ((self.acceleration.1 + new_acceleration.1) * 0.5) * seconds,
        )
    }
}
