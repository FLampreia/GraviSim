import math

# Distance in X and Y (m)
POS_EARTH = [0, 0]
POS_MOON = [3.84e8, 0]

# Velocity in X and Y (m/s)
VELO_EARTH = [0, 0]
VELO_MOON = [0, 1000]

# Mass (kg)
MASS_EARTH = 5.97e24
MASS_MOON = 7.35e22

# Distance (m)
r = 3.84e8

# Constants
G = 6.67e-11  # (Nm**2/Kg**2)

# Calculation step
STEP = 60  # (s)
NUMBER_STEPS = 100

print("0 sec")
print(VELO_MOON)


# Force that earth attracts the moon
# (N)
def forceEarthAttractsMoon():

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
def velocityAfterStep(a, step):
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


F = forceEarthAttractsMoon()

# As moon is by the right side of the earth, acceleration needs to be negative
ax = -accelerationFromForce(F, MASS_MOON)

# -- 0 sec

mean_vx = meanVelocity(VELO_MOON[0], velocityAfterStep(ax, STEP))

VELO_MOON[0] = velocityAfterStep(ax, STEP)

print("60 sec")
print(VELO_MOON)

dx = mean_vx * STEP
dy = VELO_MOON[1] * STEP

# New moon position
POS_MOON[0] = POS_MOON[0] + dx
POS_MOON[1] = POS_MOON[1] + dy

# Distance from earth to moon
# Pytagoras theorem
d = math.sqrt(POS_MOON[0] ** 2 + POS_MOON[1] ** 2)

# Acceleration on each axis on the new position
ax = ax * (POS_MOON[0] / d)
ay = ax * (POS_MOON[1] / d)

# -- 60 sec

mean_vx = meanVelocity(VELO_MOON[0], VELO_MOON[0] + velocityAfterStep(ax, STEP))
mean_vy = meanVelocity(VELO_MOON[1], VELO_MOON[1] + velocityAfterStep(ay, STEP))

VELO_MOON = [
    VELO_MOON[0] + velocityAfterStep(ax, STEP),
    VELO_MOON[1] + velocityAfterStep(ay, STEP),
]

dlcx = mean_vx * STEP
dlcy = mean_vy * STEP

POS_MOON = [POS_MOON[0] + dlcx, POS_MOON[1] + dlcy]

print("120 sec")
print(VELO_MOON)
print(POS_MOON)
