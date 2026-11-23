//! Converts physical positions in the resource into entity rendering positions.

use bevy::{
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Query, Res},
    },
    transform::components::Transform,
};

use crate::simulation::Simulation;

// Fieldless markers identify each body's visual entity in queries.
#[derive(Component)]
pub(crate) struct Earth;

#[derive(Component)]
pub(crate) struct Moon;

// Res provides read access to physics; only Transform components are modified.
// Without filters make the two queries mutually exclusive,
// allowing Bevy to verify that they cannot modify the same Transform.
pub(crate) fn sync_visuals(
    simulation: Res<Simulation>,
    mut query_earth: Query<&mut Transform, (With<Earth>, Without<Moon>)>,
    mut query_moon: Query<&mut Transform, (With<Moon>, Without<Earth>)>,
) {
    // Scale: 1e6 metres per rendering unit. Divide in f64 before converting
    // to Transform's f32; this precision loss only affects the visuals.
    // Copy the absolute position to avoid accumulating errors in the visuals.
    for mut transform in &mut query_earth {
        transform.translation.x = (simulation.earth.position.0 / 1e6) as f32;
        transform.translation.y = (simulation.earth.position.1 / 1e6) as f32;
    }

    for mut transform in &mut query_moon {
        transform.translation.x = (simulation.moon.position.0 / 1e6) as f32;
        transform.translation.y = (simulation.moon.position.1 / 1e6) as f32;
    }
}
