//! Creates visual entities and initial conditions for the Earth–Moon system.

use std::collections::VecDeque;

use bevy::{
    camera::Camera2d, color::Color, ecs::system::Commands, math::Vec2, sprite::Sprite,
    transform::components::Transform,
};

use crate::celestial_body::CelestialBody;
use crate::simulation::{Simulation, gravitational_acceleration};
use crate::trails::TrailHistory;
use crate::visualization::{Earth, Moon};

// `pub(crate)` allows access from other modules in this program.
// Bevy provides Commands; entity creation requests are applied later.
pub(crate) fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Sprite sizes are visual choices and do not represent physical radii to scale.
    // Sprite lists Transform as a required component:
    // Bevy adds a default transform when one is not supplied.
    commands.spawn((
        Earth,
        Sprite::from_color(Color::srgb(0.0, 0.0, 1.0), Vec2::new(40.0, 40.0)),
    ));

    commands.spawn((
        Moon,
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(16.0, 16.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Initial frame: Earth at rest at the origin; Moon to the right and moving upward.
    // Total linear momentum is nonzero, so the centre of mass moves.
    let mut earth: CelestialBody = CelestialBody {
        mass: 5.97e24,
        position: (0.0, 0.0),
        velocity: (0.0, 0.0),
        acceleration: (0.0, 0.0),
    };

    let mut moon: CelestialBody = CelestialBody {
        mass: 7.35e22,
        position: (3.84e8, 0.0),
        velocity: (0.0, 1000.0),
        acceleration: (0.0, 0.0),
    };

    // Velocity Verlet needs the initial acceleration before advancing positions.
    earth.acceleration =
        gravitational_acceleration(moon.position, earth.position, moon.mass, earth.mass);
    moon.acceleration =
        gravitational_acceleration(earth.position, moon.position, earth.mass, moon.mass);

    let mut trail: TrailHistory = TrailHistory {
        earth: VecDeque::new(),
        moon: VecDeque::new(),
    };

    trail.earth.push_back(earth.position);
    trail.moon.push_back(moon.position);

    // Ownership of both bodies moves into Simulation; the local bindings can no longer be used.
    let simulation: Simulation = Simulation {
        earth,
        moon,
        paused: false,
        elapsed_seconds: 0.0,
    };

    // Bevy stores this state; systems access it through Res or ResMut.
    commands.insert_resource(simulation);
    commands.insert_resource(trail);
}
