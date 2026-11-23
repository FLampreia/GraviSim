use crate::celestial_body::CelestialBody;

pub(crate) fn simulate_step(body1: &mut CelestialBody, body2: &mut CelestialBody, seconds: f64) {
    let new_position_1: (f64, f64);
    let new_position_2: (f64, f64);
    let new_acceleration_1: (f64, f64);
    let new_acceleration_2: (f64, f64);

    // Calculate the new position using the initial velocity and acceleration.
    new_position_1 = body1.new_position(seconds);
    new_position_2 = body2.new_position(seconds);

    // Calculate both final accelerations using the two new positions.
    new_acceleration_1 =
        gravitational_acceleration(new_position_2, new_position_1, body2.mass, body1.mass);
    new_acceleration_2 =
        gravitational_acceleration(new_position_1, new_position_2, body1.mass, body2.mass);

    // Update velocities while the bodies still store their initial accelerations.
    body1.velocity = body1.new_velocity(new_acceleration_1, seconds);
    body2.velocity = body2.new_velocity(new_acceleration_2, seconds);

    // Store the new positions and reuse the final accelerations in the next step.
    body1.position = new_position_1;
    body2.position = new_position_2;

    body1.acceleration = new_acceleration_1;
    body2.acceleration = new_acceleration_2;
}

fn gravitational_force(mass_1: f64, mass_2: f64, r: f64) -> f64 {
    let gravitational_constant: f64 = 6.67e-11;

    gravitational_constant * ((mass_1 * mass_2) / (r.powi(2)))
}

fn acceleration_from_force(f: f64, m: f64) -> f64 {
    f / m
}

pub(crate) fn gravitational_acceleration(
    position1: (f64, f64),
    position2: (f64, f64),
    mass_1: f64,
    mass_2: f64,
) -> (f64, f64) {
    // Displacement from body 1 to body 2, in metres; separation must be nonzero.
    let vector: (f64, f64) = (position2.0 - position1.0, position2.1 - position1.1);

    let distance_between_bodies: f64 = (vector.0).hypot(vector.1);

    // Magnitude of the gravitational force between the bodies, in newtons.
    let force: f64 = gravitational_force(mass_1, mass_2, distance_between_bodies);

    // The negative sign points the acceleration from mass 2 to mass 1.
    let a: f64 = -acceleration_from_force(force, mass_2);

    // Return the acceleration of body 2 towards body 1, in m/s².
    (
        a * vector.0 / distance_between_bodies,
        a * vector.1 / distance_between_bodies,
    )
}
