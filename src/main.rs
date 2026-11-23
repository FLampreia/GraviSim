fn main() {
    let mass_1: f64 = 5.97e24;

    let mass_2: f64 = 7.35e22;
    let mut velocity_2: (f64, f64) = (0.0, 1000.0);
    let mut position_2: (f64, f64) = (3.84e8, 0.0);
    let mut acceleration_2: (f64, f64);

    let mut new_position_2: (f64, f64);
    let mut new_acceleration_2: (f64, f64);

    let seconds: f64 = 60.0;

    // This cycle only afects body 2 parameters
    for _i in 1..=100 {
        // Calculate acceleration in the position
        acceleration_2 = gravitational_acceleration(position_2, mass_1, mass_2);

        // Verlet Method

        // Calculate the new position using the initial velocity and acceleration.
        new_position_2 = (
            position_2.0 + velocity_2.0 * seconds + 0.5 * acceleration_2.0 * (seconds.powi(2)),
            position_2.1 + velocity_2.1 * seconds + 0.5 * acceleration_2.1 * (seconds.powi(2)),
        );

        // Predicts acceleration in the next position
        new_acceleration_2 = gravitational_acceleration(new_position_2, mass_1, mass_2);

        // Predicts velocity in the next position
        velocity_2 = (
            velocity_2.0 + ((acceleration_2.0 + new_acceleration_2.0) * 0.5) * seconds,
            velocity_2.1 + ((acceleration_2.1 + new_acceleration_2.1) * 0.5) * seconds,
        );

        position_2 = new_position_2;

        println!("{}, {}", position_2.0, position_2.1)
    }
}

fn gravitational_force(mass_1: f64, mass_2: f64, r: f64) -> f64 {
    let gravitational_constant: f64 = 6.67e-11;

    gravitational_constant * ((mass_1 * mass_2) / (r.powi(2)))
}

fn acceleration_from_force(f: f64, m: f64) -> f64 {
    f / m
}

fn distance_to_earth_with_earth_in_origin(position: (f64, f64)) -> f64 {
    // TODO review becouse body1 might not be in origin
    position.0.hypot(position.1)
}

fn gravitational_acceleration(position: (f64, f64), mass_1: f64, mass_2: f64) -> (f64, f64) {
    let distance_between_1_and_2: f64 = distance_to_earth_with_earth_in_origin(position);

    // Force that body 1 does pulling body 2
    let force: f64 = gravitational_force(mass_1, mass_2, distance_between_1_and_2);

    // The negative sign points the acceleration from mass 2 to mass 1.
    let a: f64 = -acceleration_from_force(force, mass_2);

    // Project the acceleration onto the x and y axes at the current position.
    (
        a * (position.0 / distance_between_1_and_2),
        a * (position.1 / distance_between_1_and_2),
    )
}
