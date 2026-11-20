import math

# Mass (kg)
MASS_EARTH = 5.97e24
MASS_MOON = 7.35e22

# Constants
G = 6.67e-11  # (Nm**2/Kg**2)

# Calculation step
STEP_JUMP_TIME = 60  # (s)
NUMBER_STEPS = 43200
PRINT_STEP = 360


# Force that earth attracts the moon
# (N)
def forceEarthAttractsMoon(r):

    # Newton's Universal Gravitational Law
    # F = G (M1 * M2) / r**2
    return G * (MASS_EARTH * MASS_MOON) / r**2


# Acceleration direct from Moon to Earth calculated from force
# (m/s**2)
def accelerationFromForce(F, m):

    # Newton's 2ª law
    # F = ma
    # a = F/m
    return F / m


#
def velocityVariationAfterStep(a, step):
    """
    acceleration is constant in each step

    Args:
        a (m/s**2): acceleration in each step
        step (s): time after last instant

    Returns:
        Velocity (m/s): Velocity after the instant
    """

    # Delta v = a * Delta t
    # vf - vi = a * (tf - ti)
    # vf = (a * (tf - ti)) + vi
    return a * step


# Mean velocity with constant acceleration in each instant
def meanVelocity(vi, vf):
    return (vi + vf) / 2


def main():

    # Distance in X and Y (m)
    POS_EARTH = [0, 0]
    POS_MOON = [3.84e8, 0]

    # Velocity in X and Y (m/s)
    VELO_EARTH = [0, 0]
    VELO_MOON = [0, 1000]

    print(f"--- Start: ---")
    print(f"Moon Position: {POS_MOON}")
    print(f"Moon Velocity: {VELO_MOON}")

    for i in range(NUMBER_STEPS):

        # Distance from earth to moon
        # Pytagoras theorem
        d = math.sqrt(POS_MOON[0] ** 2 + POS_MOON[1] ** 2)

        F = forceEarthAttractsMoon(d)

        # As moon is by the right side of the earth, acceleration needs to be negative
        a = -accelerationFromForce(F, MASS_MOON)

        if i % PRINT_STEP == 0:
            print(f"Moon Acceleration: {a}")

        # Acceleration on each axis on the new position
        ax = a * (POS_MOON[0] / d)
        ay = a * (POS_MOON[1] / d)

        mean_vx = meanVelocity(
            VELO_MOON[0], VELO_MOON[0] + velocityVariationAfterStep(ax, STEP_JUMP_TIME)
        )
        mean_vy = meanVelocity(
            VELO_MOON[1], VELO_MOON[1] + velocityVariationAfterStep(ay, STEP_JUMP_TIME)
        )

        VELO_MOON = [
            VELO_MOON[0] + velocityVariationAfterStep(ax, STEP_JUMP_TIME),
            VELO_MOON[1] + velocityVariationAfterStep(ay, STEP_JUMP_TIME),
        ]

        dx = mean_vx * STEP_JUMP_TIME
        dy = mean_vy * STEP_JUMP_TIME

        # New moon position
        POS_MOON = [POS_MOON[0] + dx, POS_MOON[1] + dy]

        if i % PRINT_STEP == 0:
            print(f"--- After: {(i + 1) * STEP_JUMP_TIME} sec ---")
            print(f"Moon Position: {POS_MOON}")
            print(f"Moon Velocity: {VELO_MOON}")


if __name__ == "__main__":
    main()
