use bevy::{
    ecs::system::{Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
};

use crate::simulation::Simulation;

pub(crate) fn toggle_pause(button: Res<ButtonInput<KeyCode>>, mut simulation: ResMut<Simulation>) {
    if button.just_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
    }
}
