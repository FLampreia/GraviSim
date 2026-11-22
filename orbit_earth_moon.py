from math import radians

from objects.celestial_body import CelestialBody
from simulator import simulator
from visualization import visualize


def main():
    # Calculation step
    TIME_BETWEEN_SIMULATIONS = 60  # (s)
    # Print one sample every X simulation steps.
    SIMULATIONS_BETWEEN_DRAW = 320
    
    earth = CelestialBody(
        name="Earth",
        mass=5.97e24,
        position=(0, 0),
        velocity=0,
        velocityTheta=0,
        color="blue"
    )

    moon = CelestialBody(
        name="Moon",
        mass=7.35e22,
        position=(3.84e8, 0),
        velocity=1022,
        velocityTheta=radians(90),
        color="purple"
    )
    
    
    visualize(SIMULATIONS_BETWEEN_DRAW, TIME_BETWEEN_SIMULATIONS, earth, moon)

    # list_pos = simulator(
    #     NUMBER_STEPS,
    #     PRINT_STEP,
    #     STEP_JUMP_TIME,
    #     earth,
    #     moon,
    # )

    # visualize(list_pos)


if __name__ == "__main__":
    main()
