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

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Sprite sizes are illustrative; Bevy adds a default Transform when omitted.
    commands.spawn((
        Earth,
        Sprite::from_color(Color::srgb(0.0, 0.0, 1.0), Vec2::new(40.0, 40.0)),
    ));

    commands.spawn((
        Moon,
        Sprite::from_color(Color::srgb(0.7, 0.7, 0.7), Vec2::new(16.0, 16.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Earth starts at rest; the moving Moon gives the centre of mass a nonzero velocity.
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

    // Move ownership of both bodies into the shared simulation state.
    let simulation: Simulation = Simulation {
        earth,
        moon,
        paused: false,
        elapsed_seconds: 0.0,
    };

    commands.insert_resource(simulation);
    commands.insert_resource(trail);
}
