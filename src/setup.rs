//! Creates visual entities and initial conditions for the Earth–Moon system.

use std::collections::VecDeque;

use bevy::asset::Assets;
use bevy::ecs::system::ResMut;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::shape::Circle;
use bevy::sprite_render::{ColorMaterial, MeshMaterial2d};
use bevy::{
    camera::Camera2d, color::Color, ecs::system::Commands, transform::components::Transform,
};

use crate::simulation::Simulation;
use crate::trails::TrailHistory;
use crate::visualization::{Earth, Moon};

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Unit circles; sync_visuals applies physical radii at the rendering scale.
    let earth_mesh = meshes.add(Circle::new(1.0));
    let earth_material = materials.add(Color::srgb(0.0, 0.0, 1.0));

    let moon_mesh = meshes.add(Circle::new(1.0));
    let moon_material = materials.add(Color::srgb(0.7, 0.7, 0.7));

    commands.spawn((
        Earth,
        Mesh2d(earth_mesh),
        MeshMaterial2d(earth_material),
        Transform::default(),
    ));

    commands.spawn((
        Moon,
        Mesh2d(moon_mesh),
        MeshMaterial2d(moon_material),
        Transform::default(),
    ));

    // Reuse the same initial conditions at startup and on reset.
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
