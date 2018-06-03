import math
from random import random

from vec3 import Vec3
from ray import Ray

def random_in_unit_disk():
    while True:
        p = 2.0*Vec3(random(),random(),0) - Vec3(1,1,0)
        if p.dot(p) < 1: return p

class Camera(object):

    def __init__(self, lookfrom, lookat, vup, vfov, aspect, aperture, focus_dist):
        self.lens_radius = aperture / 2;
        theta = vfov*math.pi/180
        half_height = math.tan(theta/2)
        half_width = aspect * half_height
        self.origin = lookfrom
        self.w = (lookfrom - lookat).unit
        self.u = vup.cross(self.w).unit
        self.v = self.w.cross(self.u)
        self.lower_left_corner = self.origin - half_width*focus_dist*self.u - half_height*focus_dist*self.v - focus_dist*self.w
        self.horizontal = 2*half_width*focus_dist*self.u
        self.vertical = 2*half_height*focus_dist*self.v

    def getRay(self, s, t):
        rd = self.lens_radius*random_in_unit_disk()
        offset = self.u*rd.x + self.v*rd.y
        return Ray(self.origin + offset, self.lower_left_corner + s*self.horizontal
            + t*self.vertical - self.origin - offset)
