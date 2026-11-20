import math
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import numpy as np

# Mass (kg)
MASS_EARTH = 5.97e24
MASS_MOON = 7.35e22

# Constants
G = 6.67e-11  # (Nm**2/Kg**2)

# Calculation step
STEP_JUMP_TIME = 60  # (s)
NUMBER_STEPS = 43200  # 43200 86400 518400

# Save and print one sample every 360 simulation steps.
PRINT_STEP = 360


def forceEarthAttractsMoon(r):
    """Return the magnitude of Earth's gravitational force on the Moon in newtons.

    The separation r is measured between the bodies' centres in metres.
    Newton's law of universal gravitation gives F = G * M_earth * M_moon / r**2.
    The masses are in kilograms and G is in N m**2 / kg**2.
    """
    return G * (MASS_EARTH * MASS_MOON) / r**2


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


def meanVelocity(vi, vf):
    """Return the time average of one velocity component during a step, in m/s.

    With constant acceleration the component varies linearly, so its time
    average is the arithmetic mean of vi and vf, both in m/s.
    """
    return (vi + vf) / 2


def distance_to_earth(position):
    """Return the distance from (x, y) to Earth at the origin, in metres.

    The coordinates are in metres. By the Pythagorean theorem, the distance
    is sqrt(x**2 + y**2), which math.hypot computes.
    """
    x, y = position
    return math.hypot(x, y)


def main():
    """Simulate the Moon in two dimensions and display its animated trajectory.

    Earth stays fixed at the origin. The simulation treats the Moon's
    gravitational acceleration as constant within each time step.
    """

    # Store the initial position and the samples used by the plot.
    list_pos = []

    # Distance in X and Y (m)
    POS_EARTH = [0, 0]
    POS_MOON = [3.84e8, 0]

    # Velocity in X and Y (m/s)
    VELO_EARTH = [0, 0]
    VELO_MOON = [0, 1022] # 1022

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

        # The negative sign points the acceleration towards Earth.
        a = -accelerationFromForce(F, MASS_MOON)

        # Project the acceleration onto the x and y axes at the current position.
        ax = a * (POS_MOON[0] / d)
        ay = a * (POS_MOON[1] / d)

        # Use the mean velocity over the step to estimate the displacement.
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

    # Create the figure and axes, then split the sampled positions into x and y coordinates.
    fig, ax = plt.subplots()
    x_values, y_values = zip(*list_pos)

    # Start with an empty trajectory. The comma unpacks the single line returned by plot.
    (trajectory,) = ax.plot([], [], color="tab:purple")
    
    # Use a marker to show the Moon's current position.
    (moon,) = ax.plot(
        [], [], marker="o", linestyle="none", color="tab:purple", label="Moon"
    )

    # Fix symmetric limits around Earth with a 10% margin beyond the largest coordinate.
    limit = 1.1 * max(max(map(abs, x_values)), max(map(abs, y_values)))
    ax.set_xlim(-limit, limit)
    ax.set_ylim(-limit, limit)

    def update(index):
        # Draw all positions up to and including the current one.
        trajectory.set_data(x_values[: index + 1], y_values[: index + 1])
        # Move the marker to the current position using one value per axis.
        moon.set_data([x_values[index]], [y_values[index]])

    # Find the closest and farthest positions among the saved samples.
    perigee = min(list_pos, key=distance_to_earth)
    apogee = max(list_pos, key=distance_to_earth)

    for (x, y), label, color in [
        (perigee, "Perigeu", "tab:green"),
        (apogee, "Apogeu", "tab:orange"),
    ]:
        # Mark each extreme and connect it to Earth with a dashed line.
        ax.scatter(x, y, color=color, label=label, zorder=3)
        ax.plot([0, x], [0, y], linestyle="--", color=color)

    ax.scatter(0, 0, label="Earth")
    # Use the same scale on both axes to preserve the shape of the orbit.
    ax.set_aspect("equal")
    ax.set_xlabel("x (m)")
    ax.set_ylabel("y (m)")
    ax.legend(loc="upper right")

    # Keep a reference to the animation while the figure is open.
    animation = FuncAnimation(
        fig,
        update,
        frames=len(list_pos),  # One frame per saved position, starting at index 0.
        interval=80,  # Wait 80 milliseconds between frames.
        repeat=False,  # Stop after the last frame.
    )

    plt.show()


if __name__ == "__main__":
    main()
