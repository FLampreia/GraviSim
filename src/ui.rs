use bevy::prelude::*;

use crate::simulation::Simulation;

#[derive(Component)]
pub(crate) struct SimulationStatus;

pub(crate) fn setup_ui(mut commands: Commands) {
    commands.spawn((
        SimulationStatus,
        Text::new("Physical Time: 0 d 00 h 00 min | Running"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

pub(crate) fn update_ui(
    simulation: Res<Simulation>,
    mut status: Query<&mut Text, With<SimulationStatus>>,
) {
    let state: String = if simulation.paused {
        "Stopped"
    } else {
        "Running"
    }
    .to_string();

    let total_minutes: u64 = (simulation.elapsed_seconds / 60.0) as u64;
    let days: u64 = total_minutes / 1440;
    let hours: u64 = (total_minutes / 60) % 24;
    let minutes: u64 = total_minutes % 60;

    for mut text in &mut status {
        text.0 = format!(
            "Physical Time: {} d {:02} h {:02} min | {}",
            days, hours, minutes, state
        )
    }
}
