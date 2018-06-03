import math

from vec3 import Vec3
from ray import Ray

class Camera(object):

    def __init__(self):
        self.lower_left_corner = Vec3(-2, -1, -1)
        self.horizontal = Vec3(4, 0, 0)
        self.vertical = Vec3(0, 2, 0)
        self.origin = Vec3(0, 0, 0)

    def getRay(self, u, v):
        return Ray(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical)
