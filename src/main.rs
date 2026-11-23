fn main() {
    let mass_1: f64 = 5.97e24;
    let mut velocity_1: (f64, f64) = (0.0, 0.0);
    let mut position_1: (f64, f64) = (0.0, 0.0);
    let mut acceleration_1: (f64, f64);

    let mass_2: f64 = 7.35e22;
    let mut velocity_2: (f64, f64) = (0.0, 1000.0);
    let mut position_2: (f64, f64) = (3.84e8, 0.0);
    let mut acceleration_2: (f64, f64);

    let mut new_position_1: (f64, f64);
    let mut new_position_2: (f64, f64);
    let mut new_acceleration_1: (f64, f64);
    let mut new_acceleration_2: (f64, f64);

    let seconds: f64 = 60.0;

    // This cycle only afects body 2 parameters
    for _i in 1..=100 {
        // Calculate acceleration in the position
        acceleration_1 = gravitational_acceleration(position_2, position_1, mass_2, mass_1);
        acceleration_2 = gravitational_acceleration(position_1, position_2, mass_1, mass_2);

        // --- Verlet Method ---

        // Calculate the new position using the initial velocity and acceleration.
        new_position_1 = (
            position_1.0 + velocity_1.0 * seconds + 0.5 * acceleration_1.0 * (seconds.powi(2)),
            position_1.1 + velocity_1.1 * seconds + 0.5 * acceleration_1.1 * (seconds.powi(2)),
        );
        new_position_2 = (
            position_2.0 + velocity_2.0 * seconds + 0.5 * acceleration_2.0 * (seconds.powi(2)),
            position_2.1 + velocity_2.1 * seconds + 0.5 * acceleration_2.1 * (seconds.powi(2)),
        );

        // Predicts acceleration in the next position
        new_acceleration_1 = gravitational_acceleration(new_position_2, new_position_1, mass_2, mass_1);
        new_acceleration_2 = gravitational_acceleration(new_position_1, new_position_2, mass_1, mass_2);

        // Predicts velocity in the next position
        velocity_1 = (
            velocity_1.0 + ((acceleration_1.0 + new_acceleration_1.0) * 0.5) * seconds,
            velocity_1.1 + ((acceleration_1.1 + new_acceleration_1.1) * 0.5) * seconds,
        );
        velocity_2 = (
            velocity_2.0 + ((acceleration_2.0 + new_acceleration_2.0) * 0.5) * seconds,
            velocity_2.1 + ((acceleration_2.1 + new_acceleration_2.1) * 0.5) * seconds,
        );
        
        position_1 = new_position_1;
        position_2 = new_position_2;

        println!("-------");
        println!("Terra: {}, {}", position_1.0, position_1.1);
        println!("Lua: {}, {}", position_2.0, position_2.1);
    }
}

fn gravitational_force(mass_1: f64, mass_2: f64, r: f64) -> f64 {
    let gravitational_constant: f64 = 6.67e-11;

    gravitational_constant * ((mass_1 * mass_2) / (r.powi(2)))
}

fn acceleration_from_force(f: f64, m: f64) -> f64 {
    f / m
}

fn distance_between_bodies(position1: (f64, f64), position2: (f64, f64)) -> f64 {
    (position2.0 - position1.0).hypot(position2.1 - position1.1)
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
