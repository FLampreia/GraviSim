struct Body {
    mass: f64,
    position: (f64, f64),
    velocity: (f64, f64),
}

fn main() {
    let mut earth: Body = Body {
        mass: 5.97e24,
        position: (0.0, 0.0),
        velocity: (0.0, 0.0),
    };
    let mut acceleration_1: (f64, f64);

    let mut moon: Body = Body {
        mass: 7.35e22,
        position: (3.84e8, 0.0),
        velocity: (0.0, 1000.0),
    };
    let mut acceleration_2: (f64, f64);

    let mut new_position_1: (f64, f64);
    let mut new_position_2: (f64, f64);
    let mut new_acceleration_1: (f64, f64);
    let mut new_acceleration_2: (f64, f64);

    let seconds: f64 = 60.0;

    // This cycle only afects body 2 parameters
    for _i in 1..=100 {
        // Calculate acceleration in the position
        acceleration_1 =
            gravitational_acceleration(moon.position, earth.position, moon.mass, earth.mass);
        acceleration_2 =
            gravitational_acceleration(earth.position, moon.position, earth.mass, moon.mass);

        // --- Verlet Method ---

        // Calculate the new position using the initial velocity and acceleration.
        new_position_1 = new_position(earth.position, earth.velocity, acceleration_1, seconds);
        new_position_2 = new_position(moon.position, moon.velocity, acceleration_2, seconds);

        // Predicts acceleration in the next position
        new_acceleration_1 =
            gravitational_acceleration(new_position_2, new_position_1, moon.mass, earth.mass);
        new_acceleration_2 =
            gravitational_acceleration(new_position_1, new_position_2, earth.mass, moon.mass);

        // Predicts velocity in the next position
        earth.velocity = new_velocity(earth.velocity, acceleration_1, new_acceleration_1, seconds);
        moon.velocity = new_velocity(moon.velocity, acceleration_2, new_acceleration_2, seconds);

        earth.position = new_position_1;
        moon.position = new_position_2;

        println!("-------");
        println!("Terra: {}, {}", earth.position.0, earth.position.1);
        println!("Lua: {}, {}", moon.position.0, moon.position.1);
    }
}



fn new_position(
    position: (f64, f64),
    velocity: (f64, f64),
    acceleration: (f64, f64),
    seconds: f64,
) -> (f64, f64) {
    (
        position.0 + velocity.0 * seconds + 0.5 * acceleration.0 * (seconds.powi(2)),
        position.1 + velocity.1 * seconds + 0.5 * acceleration.1 * (seconds.powi(2)),
    )
}

fn new_velocity(
    velocity: (f64, f64),
    acceleration: (f64, f64),
    new_acceleration: (f64, f64),
    seconds: f64,
) -> (f64, f64) {
    (
        velocity.0 + ((acceleration.0 + new_acceleration.0) * 0.5) * seconds,
        velocity.1 + ((acceleration.1 + new_acceleration.1) * 0.5) * seconds,
    )
}

fn gravitational_force(mass_1: f64, mass_2: f64, r: f64) -> f64 {
    let gravitational_constant: f64 = 6.67e-11;

    gravitational_constant * ((mass_1 * mass_2) / (r.powi(2)))
}

fn acceleration_from_force(f: f64, m: f64) -> f64 {
    f / m
}

fn gravitational_acceleration(
    position1: (f64, f64),
    position2: (f64, f64),
    mass_1: f64,
    mass_2: f64,
) -> (f64, f64) {
    //vector that points from 1 to 2
    let vector: (f64, f64) = (position2.0 - position1.0, position2.1 - position1.1);

    let distance_between_bodies: f64 = (vector.0).hypot(vector.1);

    // Force between 2 bodies
    let force: f64 = gravitational_force(mass_1, mass_2, distance_between_bodies);

    // The negative sign points the acceleration from mass 2 to mass 1.
    let a: f64 = -acceleration_from_force(force, mass_2);

    // Project the acceleration onto the x and y axes at the current position.
    (
        a * vector.0 / distance_between_bodies,
        a * vector.1 / distance_between_bodies,
    )
}
