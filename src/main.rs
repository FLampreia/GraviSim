mod celestial_body;
mod simulation;

use bevy::prelude::*;

use crate::celestial_body::CelestialBody;
use crate::simulation::gravitational_acceleration;
use crate::simulation::simulate_step;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GraviSim".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, sync_visuals)
        .add_systems(FixedUpdate, simulate)
        .insert_resource(Time::<Fixed>::from_seconds(0.02))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Earth,
        Sprite::from_color(Color::srgb(0.0, 0.0, 1.0), Vec2::new(40.0, 40.0)),
        // Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Moon,
        Sprite::from_color(Color::srgb(0.7, 0.7, 1.7), Vec2::new(16.0, 16.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let mut earth: CelestialBody = CelestialBody {
        mass: 5.97e24,
        position: (0.0, 0.0),
        velocity: (0.0, 0.0),
        acceleration: (0.0, 0.0),
    };

    let mut moon: CelestialBody = CelestialBody {
        mass: 7.35e22,
        position: (3.84e8, 0.0),
        velocity: (0.0, 1000.0),
        acceleration: (0.0, 0.0), // Initialized from both bodies' masses and positions below.
    };

    // Initialize both accelerations before the first Velocity Verlet step.
    earth.acceleration =
        gravitational_acceleration(moon.position, earth.position, moon.mass, earth.mass);
    moon.acceleration =
        gravitational_acceleration(earth.position, moon.position, earth.mass, moon.mass);

    let simulation: Simulation = Simulation { earth, moon };

    commands.insert_resource(simulation);
}

fn sync_visuals(
    simulation: Res<Simulation>,
    mut query_earth: Query<&mut Transform, (With<Earth>, Without<Moon>)>,
    mut query_moon: Query<&mut Transform, (With<Moon>, Without<Earth>)>,
) {
    for mut transform in &mut query_earth {
        transform.translation.x = (simulation.earth.position.0 / 1e6) as f32;
        transform.translation.y = (simulation.earth.position.1 / 1e6) as f32;
    }

    for mut transform in &mut query_moon {
        transform.translation.x = (simulation.moon.position.0 / 1e6) as f32;
        transform.translation.y = (simulation.moon.position.1 / 1e6) as f32;
    }
}

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Moon;

#[derive(Resource)]
struct Simulation {
    earth: CelestialBody,
    moon: CelestialBody,
}

fn simulate(time: Res<Time<Fixed>>, simulation: ResMut<Simulation>) {
    let state: &mut Simulation = simulation.into_inner();

    let seconds: f64 = time.delta_secs_f64() * 3000.0;

    simulate_step(&mut state.earth, &mut state.moon, seconds);
}

// fn simulate() {
//     // Advance both bodies by the same time interval using Velocity Verlet.
//     for _i in 1..=100 {
//         simulate_step(&mut earth, &mut moon, seconds);

//         println!("-------");
//         println!("Terra: {}, {}", earth.position.0, earth.position.1);
//         println!("Lua: {}, {}", moon.position.0, moon.position.1);
//         println!(
//             "Momento Linear: terra {}, lua {}",
//             (earth.mass * earth.velocity.0 + moon.mass * moon.velocity.0),
//             (earth.mass * earth.velocity.1 + moon.mass * moon.velocity.1)
//         );
//     }
// }
