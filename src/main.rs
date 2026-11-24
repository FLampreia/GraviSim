//! Entry point: configures Bevy and schedules each system.

mod celestial_body;
mod controls;
mod setup;
mod simulation;
mod trails;
mod ui;
mod visualization;

use crate::{
    controls::{CameraMode, move_camera, toggle_pause, zoom_camera},
    setup::setup,
    simulation::simulate,
    trails::{draw_trails, record_trails},
    ui::{setup_ui, update_ui},
    visualization::sync_visuals,
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
        // Startup runs once; Update handles visuals and controls each frame.
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                sync_visuals,
                toggle_pause,
                draw_trails,
                zoom_camera,
                move_camera,
                update_ui.after(toggle_pause),
            ),
        )
        // Record trails after each completed physics step.
        .add_systems(FixedUpdate, (simulate, record_trails).chain())
        // 0.02 application seconds per step, before the physical time multiplier.
        .insert_resource(Time::<Fixed>::from_seconds(0.02))
        .insert_resource(CameraMode::Free)
        .run();
}
