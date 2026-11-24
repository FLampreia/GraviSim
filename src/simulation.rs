//! Shared state, Bevy simulation system, and Newtonian two-body gravity.

use bevy::{
    ecs::{
        resource::Resource,
        system::{Res, ResMut},
    },
    time::{Fixed, Time},
};

use crate::{
    celestial_body::CelestialBody,
    config::{PHYSICS_STEP_SECONDS, TIME_SCALE},
    trails::{TrailHistory, record_trails},
};

// Shared physical state, independent of its visual representation.
#[derive(Resource)]
pub(crate) struct Simulation {
    pub(crate) earth: CelestialBody,
    pub(crate) moon: CelestialBody,
    pub(crate) paused: bool,
    // Physical time already integrated, in seconds.
    pub(crate) elapsed_seconds: f64,
    // Physical time awaiting integration, in seconds.
    pub(crate) pending_seconds: f64,
}

impl Simulation {
    pub(crate) fn initial() -> Self {
        // Earth starts at rest; the moving Moon gives the centre of mass a nonzero velocity.
        let mut earth: CelestialBody = CelestialBody {
            mass: 5.97e24,
            position: (0.0, 0.0),
            velocity: (0.0, 0.0),
            acceleration: (0.0, 0.0),
            radius: 6.371e6,
        };

        let mut moon: CelestialBody = CelestialBody {
            mass: 7.35e22,
            position: (3.84e8, 0.0),
            velocity: (0.0, 1000.0),
            acceleration: (0.0, 0.0),
            radius: 1.737e6
        };

        // Velocity Verlet needs the initial acceleration before advancing positions.
        earth.acceleration =
            gravitational_acceleration(moon.position, earth.position, moon.mass, earth.mass);
        moon.acceleration =
            gravitational_acceleration(earth.position, moon.position, earth.mass, moon.mass);

        Simulation {
            earth,
            moon,
            paused: false,
            elapsed_seconds: 0.0,
            pending_seconds: 0.0,
        }
    }
}

pub(crate) fn simulate(
    time: Res<Time<Fixed>>,
    mut simulation: ResMut<Simulation>,
    mut trail: ResMut<TrailHistory>,
) {
    if simulation.paused == true {
        return;
    }

    // Convert application time to physical seconds.
    simulation.pending_seconds += time.delta_secs_f64() * TIME_SCALE;

    // A direct reference allows separate mutable borrows of both bodies.
    let state: &mut Simulation = simulation.into_inner();

    // Each iteration advances one fixed physical step.
    while state.pending_seconds >= PHYSICS_STEP_SECONDS {
        simulate_step(&mut state.earth, &mut state.moon, PHYSICS_STEP_SECONDS);
        state.elapsed_seconds += PHYSICS_STEP_SECONDS;
        state.pending_seconds -= PHYSICS_STEP_SECONDS;

        // Store every completed step, including intermediate steps in this update.
        record_trails(state, &mut trail);
    }
}

// Velocity Verlet advances both bodies from the same initial instant by dt seconds.
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
    // Newtonian gravity: F = G*m1*m2/r², in N; masses in kg and r in m.
    // G is in m³/(kg*s²); only these two bodies are modelled.
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
    // Displacement from body 1 to body 2, in metres.
    let vector: (f64, f64) = (position2.0 - position1.0, position2.1 - position1.1);

    let distance_between_bodies: f64 = (vector.0).hypot(vector.1);

    // Magnitude of the gravitational force, in newtons.
    let force: f64 = gravitational_force(mass_1, mass_2, distance_between_bodies);

    // Divide by body 2's mass. The negative sign points toward body 1.
    let a: f64 = -acceleration_from_force(force, mass_2);

    // Project acceleration onto the unit direction; result in m/s².
    (
        a * vector.0 / distance_between_bodies,
        a * vector.1 / distance_between_bodies,
    )
}
