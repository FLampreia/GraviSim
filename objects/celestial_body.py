
from math import atan2, cos, degrees, sin
import math


class CelestialBody:
    
    def __init__(self, name: str, mass, position: tuple , velocity, velocityTheta, color: str) -> None:
        self.name = name
        self.mass = mass
        self.position = position
        self.velocity = velocity
        self.velocityTheta = velocityTheta
        self.color = color
    
    def velocityX(self):
        return self.velocity * (cos(self.velocityTheta))
    
    def velocityY(self):
        return self.velocity * (sin(self.velocityTheta))
    