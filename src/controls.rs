use bevy::{
    camera::{Camera2d, Projection},
    ecs::{
        query::With,
        system::{Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseScroll},
    math::Vec2,
    time::Time,
    transform::components::Transform,
};

use crate::simulation::Simulation;

pub(crate) fn toggle_pause(button: Res<ButtonInput<KeyCode>>, mut simulation: ResMut<Simulation>) {
    if button.just_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
    }
}

pub(crate) fn zoom_camera(
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

pub(crate) fn move_camera(
    button: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    let mut pos: Vec2 = Vec2::new(0.0, 0.0);

    // Move up
    if button.pressed(KeyCode::KeyW) {
        pos.y += 1.0;
    }
    // Move down
    if button.pressed(KeyCode::KeyS) {
        pos.y -= 1.0;
    }
    // Move left
    if button.pressed(KeyCode::KeyA) {
        pos.x -= 1.0;
    }
    // Move right
    if button.pressed(KeyCode::KeyD) {
        pos.x += 1.0;
    }

    pos = pos.normalize_or_zero();

    let mut velocity: f32 = 300.0;

    if button.pressed(KeyCode::ShiftLeft) {
        velocity *= 2.0;
    }

    let displacement: Vec2 = Vec2::new(
        pos.x * velocity * time.delta_secs(),
        pos.y * velocity * time.delta_secs(),
    );

    for mut transform in &mut cameras {
        transform.translation.x += displacement.x;
        transform.translation.y += displacement.y;
    }
}
