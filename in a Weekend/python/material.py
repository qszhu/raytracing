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

    def __init__(self, a, f=0):
        self.albedo = a
        self.fuzz = min(f, 1.0)

    def scatter(self, r_in, rec, attenuation, scattered):
        reflected = reflect(Vec3.Unit(r_in.direction), rec.normal)
        scattered.update(Ray(rec.p, reflected + self.fuzz*random_in_unit_sphere()))
        attenuation.update(self.albedo)
        return Vec3.dot(scattered.direction, rec.normal) > 0

def refract(v, n, ni_over_nt, refracted):
    uv = Vec3.Unit(v)
    dt = Vec3.Dot(uv, n)
    discriminant = 1.0 - ni_over_nt*ni_over_nt*(1-dt*dt)
    if discriminant > 0:
        refracted.update(ni_over_nt*(uv - n*dt) - n*math.sqrt(discriminant))
        return True
    return False

def schlick(cosine, ref_idx):
    r0 = (1-ref_idx) / (1+ref_idx)
    r0 *= r0
    return r0 + (1-r0)*math.pow((1-cosine), 5)

class Dielectric(Material):

    def __init__(self, ri):
        self.ref_idx = ri

    def scatter(self, r_in, rec, attenuation, scattered):
        outward_normal = Vec3()
        reflected = reflect(r_in.direction, rec.normal)
        attenuation.update(Vec3(1.0, 1.0, 1.0))
        refracted = Vec3()

        if Vec3.Dot(r_in.direction, rec.normal) > 0:
            outward_normal = -rec.normal
            ni_over_nt = self.ref_idx
            cosine = self.ref_idx * Vec3.Dot(r_in.direction, rec.normal) / r_in.direction.length
        else:
            outward_normal = rec.normal
            ni_over_nt = 1.0 / self.ref_idx
            cosine = -Vec3.Dot(r_in.direction, rec.normal) / r_in.direction.length

        if refract(r_in.direction, outward_normal, ni_over_nt, refracted):
            reflect_prob = schlick(cosine, self.ref_idx)
        else:
            reflect_prob = 1.0

        if random() < reflect_prob:
            scattered.update(Ray(rec.p, reflected))
        else:
            scattered.update(Ray(rec.p, refracted))

        return True
