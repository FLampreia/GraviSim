//! Stores a bounded history of physical positions and draws orbital trails.

use std::collections::VecDeque;

use bevy::{
    color::Color,
    ecs::{
        resource::Resource,
        system::{Res, ResMut},
    },
    gizmos::gizmos::Gizmos,
    math::Vec2,
};

use crate::simulation::Simulation;

// Each queue stores (x, y) in metres, ordered from oldest to newest.
// VecDeque supports efficient insertion at the back and removal at the front.
#[derive(Resource)]
pub(crate) struct TrailHistory {
    pub(crate) earth: VecDeque<(f64, f64)>,
    pub(crate) moon: VecDeque<(f64, f64)>,
}

pub(crate) fn record_trails(simulation: Res<Simulation>, mut trail: ResMut<TrailHistory>) {
    // Paused physics must not add repeated samples to the history.
    if simulation.paused {
        return;
    }

    // Record after simulate so that each sample contains the completed step's position.
    // A pair of f64 values implements Copy: this copies coordinates without moving bodies.
    trail.earth.push_back(simulation.earth.position);
    trail.moon.push_back(simulation.moon.position);

    // Keep at most 5000 points per body, discarding the oldest samples first.
    // At 60 physical seconds per step, 5000 points span 4999 intervals (299940 s).
    while trail.earth.len() > 5000 {
        trail.earth.pop_front();
    }

    while trail.moon.len() > 5000 {
        trail.moon.pop_front();
    }
}

// Schedule in Update: immediate-mode gizmos must be redrawn every frame,
// including while paused. Drawing only reads history and never adds samples.
pub(crate) fn draw_trails(trail: Res<TrailHistory>, mut gizmo: Gizmos) {
    // iter() borrows the stored points; map() lazily converts each point to Vec2.
    // Divide by 1e6 metres per rendering unit in f64, then convert to rendering's f32.
    let earth_points = trail.earth.iter().map(|position: &(f64, f64)| {
        Vec2::new((position.0 / 1e6) as f32, (position.1 / 1e6) as f32)
    });

    // Consume the iterator directly and connect consecutive points with segments.
    // No collect() or extra point buffer is needed; fewer than two points form no segment.
    gizmo.linestrip_2d(earth_points, Color::srgb(0.0, 0.0, 1.0));

    // Use the same spatial scale as the sprites so trails follow the bodies.
    let moon_points = trail.moon.iter().map(|position: &(f64, f64)| {
        Vec2::new((position.0 / 1e6) as f32, (position.1 / 1e6) as f32)
    });

    gizmo.linestrip_2d(moon_points, Color::srgb(0.7, 0.7, 0.7));
}
