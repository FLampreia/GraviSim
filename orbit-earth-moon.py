import math
import matplotlib.pyplot as plt
import numpy as np

# Mass (kg)
MASS_EARTH = 5.97e24
MASS_MOON = 7.35e22

# Constants
G = 6.67e-11  # (Nm**2/Kg**2)

# Calculation step
STEP_JUMP_TIME = 60  # (s)
NUMBER_STEPS = 43200  # 43200 86400
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


def distance_to_earth(position):
    x, y = position
    return math.hypot(x, y)

def main():

    list_pos = []

    # Distance in X and Y (m)
    POS_EARTH = [0, 0]
    POS_MOON = [3.84e8, 0]

    # Velocity in X and Y (m/s)
    VELO_EARTH = [0, 0]
    VELO_MOON = [0, 1018]

    print(f"--- Start: ---")
    print(f"Moon Position: {POS_MOON}")
    print(f"Moon Velocity: {VELO_MOON}")

    list_pos.append(tuple(POS_MOON))

    print(list_pos)

    for i in range(NUMBER_STEPS):

        # Distance from earth to moon
        # Pytagoras theorem
        d = distance_to_earth(POS_MOON)

        F = forceEarthAttractsMoon(d)

        # As moon is by the right side of the earth, acceleration needs to be negative
        a = -accelerationFromForce(F, MASS_MOON)

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

        if (i + 1) % PRINT_STEP == 0:

            # save positions into list
            list_pos.append((POS_MOON))

            print(f"--- After: {(i + 1) * STEP_JUMP_TIME} sec ---")
            print(f"Moon Position: {POS_MOON}")
            print(f"Moon Velocity: {VELO_MOON}")
            print(f"Moon Acceleration used in the calculations: {a}")

    fig, ax = plt.subplots()
    x_values, y_values = zip(*list_pos)
    ax.plot(x_values, y_values)
    
    perigee = min(list_pos, key=distance_to_earth)
    apogee = max(list_pos, key=distance_to_earth)

    for (x, y), label, color in [
        (perigee, "Perigeu", "tab:green"),
        (apogee, "Apogeu", "tab:orange"),
    ]:
        ax.scatter(x, y, color=color, label=label, zorder=3)
        ax.plot([0, x], [0, y], linestyle="--", color=color)
            
    
    ax.scatter(0, 0, label="Earth")
    ax.set_aspect("equal")
    ax.set_xlabel("x (m)")
    ax.set_ylabel("y (m)")
    ax.legend(loc="upper right")

    plt.show()


if __name__ == "__main__":
    main()
