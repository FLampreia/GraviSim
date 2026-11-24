//! Creates visual entities and initial conditions for the Earth–Moon system.

use std::collections::VecDeque;

use bevy::{
    camera::Camera2d, color::Color, ecs::system::Commands, math::Vec2, sprite::Sprite,
    transform::components::Transform,
};

use crate::simulation::Simulation;
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

    // Move ownership of both bodies into the shared simulation state.
    let simulation: Simulation = Simulation::initial();

    let mut trail: TrailHistory = TrailHistory {
        earth: VecDeque::new(),
        moon: VecDeque::new(),
    };

    trail.earth.push_back(simulation.earth.position);
    trail.moon.push_back(simulation.moon.position);

    commands.insert_resource(simulation);
    commands.insert_resource(trail);
}
