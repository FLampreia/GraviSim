//! Keyboard and scroll controls for the simulation and camera.

use bevy::{
    camera::{Camera2d, Projection},
    ecs::{
        query::With,
        resource::Resource,
        system::{Query, Res, ResMut},
    },
    input::{
        ButtonInput,
        keyboard::KeyCode,
        mouse::{AccumulatedMouseScroll, MouseScrollPixelsPerLine},
    },
    math::Vec2,
    time::Time,
    transform::components::Transform,
};

use crate::{config::METERS_PER_UNIT, simulation::Simulation, trails::TrailHistory};

// Persistent camera mode; only Free allows manual movement.
#[derive(Resource)]
pub(crate) enum CameraMode {
    Free,
    Earth,
    Moon,
    CenterOfMass,
}

pub(crate) fn toggle_pause(button: Res<ButtonInput<KeyCode>>, mut simulation: ResMut<Simulation>) {
    if button.just_pressed(KeyCode::Space) {
        simulation.paused = !simulation.paused;
    }
}

pub(crate) fn reset_simulation(
    button: Res<ButtonInput<KeyCode>>,
    mut simulation: ResMut<Simulation>,
    mut trail: ResMut<TrailHistory>,
) {
    if button.just_pressed(KeyCode::KeyR) {
        *simulation = Simulation::initial();
        simulation.paused = true;

        trail.earth.clear();
        trail.moon.clear();

        trail.earth.push_back(simulation.earth.position);
        trail.moon.push_back(simulation.moon.position);
    } else {
        return;
    }
}

pub(crate) fn zoom_camera(
    scroll: Res<AccumulatedMouseScroll>,
    scroll_per_line: Res<MouseScrollPixelsPerLine>,
    mut cameras: Query<&mut Projection, With<Camera2d>>,
) {
    let scroll_lines: AccumulatedMouseScroll = scroll.to_lines(&scroll_per_line);
    let zoom_factor: f32 = 1.1_f32.powf(-scroll_lines.delta.y);

    for mut projection in &mut cameras {
        if let Projection::Orthographic(orthographic) = &mut *projection {
            orthographic.scale *= zoom_factor;

            // Store the clamped value; clamp does not modify scale in place.
            orthographic.scale = orthographic.scale.clamp(0.1, 10.0)
        }
    }
}

pub(crate) fn move_camera(
    button: Res<ButtonInput<KeyCode>>,
    mut camera_mode: ResMut<CameraMode>,
    simulation: Res<Simulation>,
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    // F: free, E: Earth, B: centre of mass, M: Moon.
    if button.just_pressed(KeyCode::KeyF) {
        *camera_mode = CameraMode::Free;
    }

    if button.just_pressed(KeyCode::KeyE) {
        *camera_mode = CameraMode::Earth;
    }

    if button.just_pressed(KeyCode::KeyB) {
        *camera_mode = CameraMode::CenterOfMass;
    }

    if button.just_pressed(KeyCode::KeyM) {
        *camera_mode = CameraMode::Moon;
    }

    let mut pos: Vec2 = Vec2::new(0.0, 0.0);

    // Held WASD keys define the movement direction.
    if button.pressed(KeyCode::KeyW) {
        pos.y += 1.0;
    }
    if button.pressed(KeyCode::KeyS) {
        pos.y -= 1.0;
    }
    if button.pressed(KeyCode::KeyA) {
        pos.x -= 1.0;
    }
    if button.pressed(KeyCode::KeyD) {
        pos.x += 1.0;
    }

    // Keep diagonal and axial movement equally fast; zero stays zero.
    pos = pos.normalize_or_zero();

    let mut velocity: f32 = 300.0;

    // Apply the speed boost after normalization, in rendering units per second.
    if button.pressed(KeyCode::ShiftLeft) {
        velocity *= 2.0;
    }

    // Frame duration keeps movement independent of frame rate.
    let displacement: Vec2 = Vec2::new(
        pos.x * velocity * time.delta_secs(),
        pos.y * velocity * time.delta_secs(),
    );

    for mut transform in &mut cameras {
        match &*camera_mode {
            CameraMode::Free => {
                transform.translation.x += displacement.x;
                transform.translation.y += displacement.y;
            }
            CameraMode::CenterOfMass => {
                // Mass-weighted position in metres, then (METERS_PER_UNIT) metres per rendering unit.
                transform.translation.x = ((((simulation.earth.mass
                    * simulation.earth.position.0)
                    + (simulation.moon.mass * simulation.moon.position.0))
                    / (simulation.earth.mass + simulation.moon.mass))
                    / METERS_PER_UNIT) as f32;
                transform.translation.y = ((((simulation.earth.mass
                    * simulation.earth.position.1)
                    + (simulation.moon.mass * simulation.moon.position.1))
                    / (simulation.earth.mass + simulation.moon.mass))
                    / METERS_PER_UNIT) as f32;
            }
            CameraMode::Earth => {
                // Set the camera position each frame to follow the body.
                transform.translation.x = (simulation.earth.position.0 / METERS_PER_UNIT) as f32;
                transform.translation.y = (simulation.earth.position.1 / METERS_PER_UNIT) as f32;
            }
            CameraMode::Moon => {
                transform.translation.x = (simulation.moon.position.0 / METERS_PER_UNIT) as f32;
                transform.translation.y = (simulation.moon.position.1 / METERS_PER_UNIT) as f32;
            }
        }
    }
}
