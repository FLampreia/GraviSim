mod celestial_body;
mod simulation;

use crate::celestial_body::CelestialBody;
use crate::simulation::gravitational_acceleration;
use crate::simulation::simulate_step;

fn main() {
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

    let seconds: f64 = 60.0;

    // Initialize both accelerations before the first Velocity Verlet step.
    earth.acceleration =
        gravitational_acceleration(moon.position, earth.position, moon.mass, earth.mass);
    moon.acceleration =
        gravitational_acceleration(earth.position, moon.position, earth.mass, moon.mass);

    // Advance both bodies by the same time interval using Velocity Verlet.
    for _i in 1..=100 {
        simulate_step(&mut earth, &mut moon, seconds);

        println!("-------");
        println!("Terra: {}, {}", earth.position.0, earth.position.1);
        println!("Lua: {}, {}", moon.position.0, moon.position.1);
    }
}
