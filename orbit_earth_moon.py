from math import radians

from celestial_body import CelestialBody
from simulator import simulator
from visualization import visualize


def main():
    # Distance in X and Y (m)
    POS_EARTH = (0, 0)
    POS_MOON = (3.84e8, 0)

    # Velocity in X and Y (m/s)
    VELO_EARTH = 0
    EARTH_THETA_VELO = 0
    
    VELO_MOON = 1022  # 1022
    MOON_THETA_VELO = radians(90)

    # print(f"--- Start: ---")
    # print(f"Moon Position: {POS_MOON}")
    # print(f"Moon Velocity: {VELO_MOON}")

    # Mass (kg)
    MASS_EARTH = 5.97e24
    MASS_MOON = 7.35e22

    # Calculation step
    STEP_JUMP_TIME = 60  # (s)
    NUMBER_STEPS = 43200  # 43200 86400 518400

    # Save and print one sample every 360 simulation steps.
    PRINT_STEP = 360

    earth = CelestialBody("Earth", MASS_EARTH, POS_EARTH, VELO_EARTH, EARTH_THETA_VELO)
    moon = CelestialBody("Moon", MASS_MOON, POS_MOON, VELO_MOON, MOON_THETA_VELO)
    
    list_pos = simulator(
        NUMBER_STEPS,
        PRINT_STEP,
        STEP_JUMP_TIME,
        earth,
        moon,
    )

    visualize(list_pos)


if __name__ == "__main__":
    main()
