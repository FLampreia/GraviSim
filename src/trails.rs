//! Stores a bounded history of physical positions and draws orbital trails.

use std::collections::VecDeque;

use bevy::{
    color::Color,
    ecs::{resource::Resource, system::Res},
    gizmos::gizmos::Gizmos,
    math::Vec2,
};

use crate::{
    config::{MAX_TRAIL_SIZE, METERS_PER_UNIT},
    simulation::Simulation,
};

// Bounded queues of (x, y) positions in metres, oldest first.
#[derive(Resource)]
pub(crate) struct TrailHistory {
    pub(crate) earth: VecDeque<(f64, f64)>,
    pub(crate) moon: VecDeque<(f64, f64)>,
}

pub(crate) fn record_trails(simulation: &Simulation, trail: &mut TrailHistory) {
    // Called after each integration step to store the current positions.
    trail.earth.push_back(simulation.earth.position);
    trail.moon.push_back(simulation.moon.position);

    // Bound memory by discarding the oldest samples.
    while trail.earth.len() > MAX_TRAIL_SIZE {
        trail.earth.pop_front();
    }

    while trail.moon.len() > MAX_TRAIL_SIZE {
        trail.moon.pop_front();
    }
}

// Redraw gizmos every frame, including while paused; history stays unchanged.
pub(crate) fn draw_trails(trail: Res<TrailHistory>, mut gizmo: Gizmos) {
    // Use the bodies' spatial scale; convert to f32 after division.
    let earth_points = trail.earth.iter().map(|position: &(f64, f64)| {
        Vec2::new(
            (position.0 / METERS_PER_UNIT) as f32,
            (position.1 / METERS_PER_UNIT) as f32,
        )
    });

    // Connect consecutive points directly from the iterator.
    gizmo.linestrip_2d(earth_points, Color::srgb(0.0, 0.0, 1.0));

    let moon_points = trail.moon.iter().map(|position: &(f64, f64)| {
        Vec2::new(
            (position.0 / METERS_PER_UNIT) as f32,
            (position.1 / METERS_PER_UNIT) as f32,
        )
    });

    gizmo.linestrip_2d(moon_points, Color::srgb(0.7, 0.7, 0.7));
}
