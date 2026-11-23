//! Shared state, Bevy simulation system, and Newtonian two-body gravity.

use bevy::{
    ecs::{
        resource::Resource,
        system::{Res, ResMut},
    },
    time::{Fixed, Time},
};

use crate::celestial_body::CelestialBody;

// A resource holds the shared physical state; sprites only represent it.
#[derive(Resource)]
pub(crate) struct Simulation {
    pub(crate) earth: CelestialBody,
    pub(crate) moon: CelestialBody,
}

pub(crate) fn simulate(time: Res<Time<Fixed>>, simulation: ResMut<Simulation>) {
    // Res reads the clock; ResMut allows changes to the physical state.
    // A direct reference allows separate mutable borrows of the two fields.
    let state: &mut Simulation = simulation.into_inner();

    // 3000 physical seconds per application second: 0.02 s becomes 60 s.
    // Increasing this factor also increases the physical timestep and may reduce accuracy.
    let seconds: f64 = time.delta_secs_f64() * 3000.0;

    // One step per system execution; Bevy repeats FixedUpdate as needed.
    simulate_step(&mut state.earth, &mut state.moon, seconds);
}

// Velocity Verlet: positions -> new accelerations -> velocities.
// Both bodies advance from the same initial state by the same dt (s).
fn simulate_step(body1: &mut CelestialBody, body2: &mut CelestialBody, seconds: f64) {
    let new_position_1: (f64, f64);
    let new_position_2: (f64, f64);
    let new_acceleration_1: (f64, f64);
    let new_acceleration_2: (f64, f64);

    // Compute both positions without changing the bodies' initial state yet.
    new_position_1 = body1.new_position(seconds);
    new_position_2 = body2.new_position(seconds);

    // Final accelerations use both new positions at the same instant.
    new_acceleration_1 =
        gravitational_acceleration(new_position_2, new_position_1, body2.mass, body1.mass);
    new_acceleration_2 =
        gravitational_acceleration(new_position_1, new_position_2, body1.mass, body2.mass);

    // Keep the initial accelerations until both final velocities are computed.
    body1.velocity = body1.new_velocity(new_acceleration_1, seconds);
    body2.velocity = body2.new_velocity(new_acceleration_2, seconds);

    // Store positions and reuse the final accelerations in the next step.
    body1.position = new_position_1;
    body2.position = new_position_2;

    body1.acceleration = new_acceleration_1;
    body2.acceleration = new_acceleration_2;
}

fn gravitational_force(mass_1: f64, mass_2: f64, r: f64) -> f64 {
    // Newton's law: F = G*m1*m2/r². Masses in kg, separation r in m, force in N.
    // G has units of m³/(kg*s²). The model ignores other bodies and relativistic effects.
    let gravitational_constant: f64 = 6.67e-11;

    gravitational_constant * ((mass_1 * mass_2) / (r.powi(2)))
}

fn acceleration_from_force(f: f64, m: f64) -> f64 {
    // Newton's second law, F = m*a: N/kg = m/s². Assumes positive mass.
    f / m
}

// Return the acceleration of body 2 caused by body 1.
// Assumes positive masses and distinct positions; r = 0 is not handled.
pub(crate) fn gravitational_acceleration(
    position1: (f64, f64),
    position2: (f64, f64),
    mass_1: f64,
    mass_2: f64,
) -> (f64, f64) {
    // Vector from body 1 to body 2, in metres; hypot computes its length.
    let vector: (f64, f64) = (position2.0 - position1.0, position2.1 - position1.1);

    let distance_between_bodies: f64 = (vector.0).hypot(vector.1);

    // Magnitude of the gravitational force, in newtons.
    let force: f64 = gravitational_force(mass_1, mass_2, distance_between_bodies);

    // Divide by body 2's mass. The negative sign points toward body 1.
    let a: f64 = -acceleration_from_force(force, mass_2);

    // vector/distance is a dimensionless direction; the result is in m/s².
    (
        a * vector.0 / distance_between_bodies,
        a * vector.1 / distance_between_bodies,
    )
}
