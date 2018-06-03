import math
from random import random

from vec3 import Vec3
from ray import Ray

class Material(object):

    def scatter(self, r_in, rec, attenuation, scattered):
        raise NotImplementedError

def random_in_unit_sphere():
    while True:
        p = 2.0*Vec3(random(),random(),random()) - Vec3(1,1,1)
        if p.dot(p) < 1: return p

class Lambertian(Material):

    def __init__(self, a):
        self.albedo = a

    def scatter(self, r_in, rec, attenuation, scattered):
        target = rec.p + rec.normal + random_in_unit_sphere()
        scattered.update(Ray(rec.p, target-rec.p))
        attenuation.update(self.albedo)
        return True

def reflect(v, n):
    return v - 2*Vec3.Dot(v,n)*n

class Metal(Material):

    def __init__(self, a, f=1.0):
        self.albedo = a
        self.fuzz = min(f, 1.0)

    def scatter(self, r_in, rec, attenuation, scattered):
        reflected = reflect(Vec3.Unit(r_in.direction), rec.normal)
        scattered.update(Ray(rec.p, reflected + self.fuzz*random_in_unit_sphere()))
        attenuation.update(self.albedo)
        return Vec3.dot(scattered.direction, rec.normal) > 0
