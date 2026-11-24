//! Physical body data and update formulas, independent of Bevy.

// 2D point-mass gravity with physical radii for rendering; no collisions.
pub(crate) struct CelestialBody {
    // SI units: kg, m, m/s, m/s². Pairs are (x, y); physics uses f64 precision.
    pub(crate) mass: f64,
    pub(crate) position: (f64, f64),
    pub(crate) velocity: (f64, f64),
    pub(crate) acceleration: (f64, f64),
    pub(crate) radius: f64, // Physical radius in metres.
}

impl CelestialBody {
    // Verlet position step from Taylor expansion: r + v*dt + a*dt²/2.
    // dt is in seconds; the returned position is in metres.
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

    // Verlet velocity step uses endpoint accelerations: v + (a_old + a_new)*dt/2, in m/s.
    pub(crate) fn new_velocity(&self, new_acceleration: (f64, f64), seconds: f64) -> (f64, f64) {
        (
            self.velocity.0 + ((self.acceleration.0 + new_acceleration.0) * 0.5) * seconds,
            self.velocity.1 + ((self.acceleration.1 + new_acceleration.1) * 0.5) * seconds,
        )
    }
}
