use bevy::{
    camera::{Camera2d, Projection},
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseScroll},
};

use crate::simulation::Simulation;

pub(crate) fn toggle_pause(button: Res<ButtonInput<KeyCode>>, mut simulation: ResMut<Simulation>) {
    if button.just_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
    }
}

pub(crate) fn zoom(
    scroll: Res<AccumulatedMouseScroll>,
    mut cameras: Query<&mut Projection, With<Camera2d>>,
) {
    for mut projection in &mut cameras {
        if let Projection::Orthographic(orthographic) = &mut *projection {

            // Zoom in
            if scroll.delta.y > 0.0 {
                orthographic.scale /= 1.1
            }

            // Zoom out
            if scroll.delta.y < 0.0 {
                orthographic.scale *= 1.1
            }

            // Limit the zoom between 0.1 and 10
            orthographic.scale = orthographic.scale.clamp(0.1, 10.0)
        }
    }
}
