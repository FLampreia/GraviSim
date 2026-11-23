//! Entry point: configures Bevy and schedules each system.

// `mod` includes files in the module tree; `use` shortens paths to their items.
mod celestial_body;
mod controls;
mod setup;
mod simulation;
mod visualization;
mod trails;

use crate::{
    controls::toggle_pause, setup::setup, simulation::simulate, trails::{draw_trails, record_trails}, visualization::sync_visuals,
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GraviSim".into(),
                ..default()
            }),
            ..default()
        }))
        // Initialize once, advance physics in fixed steps, and update the visuals.
        // The order of these declarations does not set Bevy's schedule execution order.
        .add_systems(Startup, setup)
        .add_systems(Update, (sync_visuals, toggle_pause))
        .add_systems(FixedUpdate, (simulate, record_trails, draw_trails).chain())
        // Each step represents 0.02 s of application time, before time scaling.
        .insert_resource(Time::<Fixed>::from_seconds(0.02))
        .run();
}
