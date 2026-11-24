//! Synchronizes body positions and visual sizes with physical data.

use bevy::{
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Query, Res},
    },
    transform::components::Transform,
};

use crate::{config::METERS_PER_UNIT, simulation::Simulation};

// Fieldless markers identify each body's visual entity in queries.
#[derive(Component)]
pub(crate) struct Earth;

#[derive(Component)]
pub(crate) struct Moon;

// Without filters prevent the mutable queries from accessing the same entity.
pub(crate) fn sync_visuals(
    simulation: Res<Simulation>,
    mut query_earth: Query<&mut Transform, (With<Earth>, Without<Moon>)>,
    mut query_moon: Query<&mut Transform, (With<Moon>, Without<Earth>)>,
) {
    // Convert metres to rendering units in f64, then cast to f32.
    for mut transform in &mut query_earth {
        transform.translation.x = (simulation.earth.position.0 / METERS_PER_UNIT) as f32;
        transform.translation.y = (simulation.earth.position.1 / METERS_PER_UNIT) as f32;

        // Equal x/y scales preserve the unit mesh's circular shape.
        transform.scale.x = (simulation.earth.radius / METERS_PER_UNIT) as f32;
        transform.scale.y = (simulation.earth.radius / METERS_PER_UNIT) as f32;
    }

    for mut transform in &mut query_moon {
        transform.translation.x = (simulation.moon.position.0 / METERS_PER_UNIT) as f32;
        transform.translation.y = (simulation.moon.position.1 / METERS_PER_UNIT) as f32;

        transform.scale.x = (simulation.moon.radius / METERS_PER_UNIT) as f32;
        transform.scale.y = (simulation.moon.radius / METERS_PER_UNIT) as f32;
    }
}
