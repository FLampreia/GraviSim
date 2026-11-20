import math


def newtonUniversalGravitationalLaw(
    r,
    M1,
    M2,
):
    """Return the magnitude of Earth's gravitational force on the Moon in newtons.

    The separation r is measured between the bodies' centres in metres.
    Newton's law of universal gravitation gives F = G * M_earth * M_moon / r**2.
    The masses are in kilograms and G is in N m**2 / kg**2.

    G = (Nm**2/Kg**2)
    """

    return 6.67e-11 * (M1 * M2) / r**2


def accelerationFromForce(F, m):
    """Return the acceleration magnitude in m/s**2 from a force F in newtons.

    Newton's second law gives a = F / m, where m is the mass in kilograms
    of the body experiencing the force. The caller sets the direction.
    """
    return F / m


def velocityVariationAfterStep(a, step):
    """Return the change in one velocity component over a time step, in m/s.

    Treat the acceleration component a (m/s**2) as constant over step (s).
    From a = delta_v / delta_t, the change is delta_v = a * step.
    """
    return a * step


def meanVelocityWithConstantAcceleration(vi, vf):
    """Return the time average of one velocity component during a step, in m/s.

    With constant acceleration the component varies linearly, so its time
    average is the arithmetic mean of vi and vf, both in m/s.
    """
    return (vi + vf) / 2


def distance_to_earth(position: list):
    """Return the distance from (x, y) to Earth at the origin, in metres.

    The coordinates are in metres. By the Pythagorean theorem, the distance
    is sqrt(x**2 + y**2), which math.hypot computes.
    """
    x, y = position
    return math.hypot(x, y)


def simulator(
    number_steps,
    print_steps,
    step_jump_time,
    moon_pos,
    moon_mass,
    earth_mass,
    moon_velocity,
):
    """Simulate the Moon in two dimensions and display its animated trajectory.

    Earth stays fixed at the origin. The simulation treats the Moon's
    gravitational acceleration as constant within each time step.
    """

    # Store the initial position and the samples used by the plot.
    list_pos = []

    list_pos.append(moon_pos)

    print(list_pos)

    for i in range(number_steps):

        # Distance from earth to moon
        # Pytagoras theorem
        d = distance_to_earth(moon_pos)

        F = newtonUniversalGravitationalLaw(d, earth_mass, moon_mass)

        # The negative sign points the acceleration towards Earth.
        a = -accelerationFromForce(F, moon_mass)

        # Project the acceleration onto the x and y axes at the current position.
        ax = a * (moon_pos[0] / d)
        ay = a * (moon_pos[1] / d)

        # Use the mean velocity over the step to estimate the displacement.
        mean_vx = meanVelocityWithConstantAcceleration(
            moon_velocity[0],
            moon_velocity[0] + velocityVariationAfterStep(ax, step_jump_time),
        )
        mean_vy = meanVelocityWithConstantAcceleration(
            moon_velocity[1],
            moon_velocity[1] + velocityVariationAfterStep(ay, step_jump_time),
        )

        moon_velocity = [
            moon_velocity[0] + velocityVariationAfterStep(ax, step_jump_time),
            moon_velocity[1] + velocityVariationAfterStep(ay, step_jump_time),
        ]

        dx = mean_vx * step_jump_time
        dy = mean_vy * step_jump_time

        # New moon position
        moon_pos = [moon_pos[0] + dx, moon_pos[1] + dy]

        if (i + 1) % print_steps == 0:

            # save positions into list
            list_pos.append(moon_pos)

            # print(f"--- After: {(i + 1) * STEP_JUMP_TIME} sec ---")
            # print(f"Moon Position: {POS_MOON}")
            # print(f"Moon Velocity: {VELO_MOON}")
            # print(f"Moon Acceleration used in the calculations: {a}")

    return list_pos
