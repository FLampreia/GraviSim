import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import numpy as np
from simulator import distance_to_earth

def visualize(list_pos):
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
