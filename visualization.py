from turtle import onkey

import matplotlib.pyplot as plt
from matplotlib.backend_bases import Event
import numpy as np
from objects.celestial_body import CelestialBody
from simulator import distance_to_earth, simulate


def visualize(simulationsBetweenDraw, timeBetweenSimulations, body1: CelestialBody, body2: CelestialBody):
    # Create the figure and axes, then split the sampled positions into x and y coordinates.
    fig, ax = plt.subplots()

    # Start with an empty trajectory. The comma unpacks the single line returned by plot.
    (body2_trajectory,) = ax.plot(
        body2.position[0], body2.position[1], color=f"tab:{body2.color}"
    )

    # Use a marker to show the Moon's current position.
    (body2_marker,) = ax.plot(
        body2.position[0],
        body2.position[1],
        marker="o",
        linestyle="none",
        color="tab:purple",
        label=body2.name,
    )

    # Fix symmetric limits around Earth with a 10% margin beyond the largest coordinate.
    limit = 1.1 * max(body2.position[0], body2.position[1])
    ax.set_xlim(-limit, limit)
    ax.set_ylim(-limit, limit)

    # Show Body1
    ax.scatter(body1.position[0], body1.position[1], label=body1.name)
    # Use the same scale on both axes to preserve the shape of the orbit.
    ax.set_aspect("equal")
    ax.set_xlabel("x (m)")
    ax.set_ylabel("y (m)")
    ax.legend(loc="upper right")

    def update():
        # Simulate X steps before showing them
        for i in range(simulationsBetweenDraw):
            simulate(timeBetweenSimulations, body1, body2)

        body2_marker.set_data([body2.position[0]], [body2.position[1]])

        fig.canvas.draw_idle()

    timer = fig.canvas.new_timer(interval=80)
    timer.add_callback(update)
    
    paused = True
    
    def on_key(event):
        nonlocal paused
        
        if event.key != " " :
            return
        
        if paused:
            timer.start()
            paused = False
        else:
            timer.stop()
            paused = True

    fig.canvas.mpl_connect("key_press_event", on_key)
    
    plt.show()

