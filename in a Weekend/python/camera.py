import math

from vec3 import Vec3
from ray import Ray

class Camera(object):

    def __init__(self, lookfrom, lookat, vup, vfov, aspect):
        theta = vfov*math.pi/180
        half_height = math.tan(theta/2)
        half_width = aspect * half_height
        self.origin = lookfrom
        w = Vec3.Unit(lookfrom - lookat)
        u = Vec3.Unit(Vec3.Cross(vup, w))
        v = Vec3.Cross(w, u)
        self.lower_left_corner = self.origin - half_width*u - half_height*v - w
        self.horizontal = 2*half_width*u
        self.vertical = 2*half_height*v

    def getRay(self, s, t):
        return Ray(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
