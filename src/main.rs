fn main() {
    let mass_1: f64 = 5.97e24;

    let mass_2: f64 = 7.35e22;
    let mut velocity_2: (f64, f64) = (0.0, 1000.0);
    let mut new_velocity_2: (f64, f64);
    let mut position_2: (f64, f64) = (3.84e8, 0.0);

    let seconds: f64 = 15.0;


    for _i in 1..=400 {
        let distance_between_1_and_2: f64 = distance_to_earth_with_earth_in_origin(position_2);

        // Force that body 1 does pulling body 2
        let force = gravitational_force(mass_1, mass_2, distance_between_1_and_2);

        // The negative sign points the acceleration towards Earth.
        let a: f64 = -acceleration_from_force(force, mass_2);

        // Project the acceleration onto the x and y axes at the current position.
        let acceleration_2: (f64, f64) = (
            a * (position_2.0 / distance_between_1_and_2),
            a * (position_2.1 / distance_between_1_and_2),
        );

        // Set new velocity for body 2
        new_velocity_2 = (
            velocity_2.0 + velocity_change(acceleration_2.0, seconds),
            velocity_2.1 + velocity_change(acceleration_2.1, seconds),
        );

        // Use the mean velocity over the step to estimate the displacement.
        let mean_velocity_2: (f64, f64) = (
            mean_velocity_with_constant_acceleration(velocity_2.0, new_velocity_2.0),
            mean_velocity_with_constant_acceleration(velocity_2.1, new_velocity_2.1),
        );

        velocity_2 = new_velocity_2;

        position_2 = (
            position_2.0 + mean_velocity_2.0 * seconds,
            position_2.1 + mean_velocity_2.1 * seconds,
        );

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

fn velocity_change(a: f64, step: f64) -> f64 {
    a * step
}

fn mean_velocity_with_constant_acceleration(vi: f64, vf: f64) -> f64 {
    // TODO review becouse acceleration might not be a constant
    (vi + vf) / 2.0
}

fn distance_to_earth_with_earth_in_origin(position: (f64, f64)) -> f64 {
    // TODO review becouse body1 might not be in origin
    position.0.hypot(position.1)
}
