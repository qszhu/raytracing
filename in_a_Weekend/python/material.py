import math
from random import random

from vec3 import Vec3
from ray import Ray

class ScatterRecord(object):

    def __init__(self, attenuation, scattered):
        self.attenuation = attenuation
        self.scattered = scattered

class Material(object):

    def scatter(self, r_in, hit):
        raise NotImplementedError

def random_in_unit_sphere():
    while True:
        p = 2.0*Vec3(random(),random(),random()) - Vec3(1,1,1)
        if p.dot(p) < 1: return p

class Lambertian(Material):

    def __init__(self, a):
        self.albedo = a

    def scatter(self, r_in, hit):
        target = hit.p + hit.normal + random_in_unit_sphere()
        return ScatterRecord(self.albedo, Ray(hit.p, target-hit.p))

def reflect(v, n):
    return v - 2*v.dot(n)*n

class Metal(Material):

    def __init__(self, a, f=0.3):
        self.albedo = a
        self.fuzz = min(f, 1.0)

    def scatter(self, r_in, hit):
        reflected = reflect(r_in.direction.unit, hit.normal)
        scattered = Ray(hit.p, reflected + self.fuzz*random_in_unit_sphere())
        if scattered.direction.dot(hit.normal) > 0:
            return ScatterRecord(self.albedo, scattered)
        return False

def refract(v, n, ni_over_nt):
    uv = v.unit
    dt = uv.dot(n)
    discriminant = 1.0 - ni_over_nt*ni_over_nt*(1-dt*dt)
    if discriminant > 0:
        return ni_over_nt*(uv - n*dt) - n*math.sqrt(discriminant)
    return False

def schlick(cosine, ref_idx):
    r0 = (1-ref_idx) / (1+ref_idx)
    r0 *= r0
    return r0 + (1-r0)*math.pow((1-cosine), 5)

class Dielectric(Material):

    def __init__(self, ri):
        self.ref_idx = ri

    def scatter(self, r_in, hit):
        reflected = reflect(r_in.direction, hit.normal)
        attenuation = Vec3(1.0, 1.0, 1.0)

        if r_in.direction.dot(hit.normal) > 0:
            outward_normal = -hit.normal
            ni_over_nt = self.ref_idx
            cosine = self.ref_idx * r_in.direction.dot(hit.normal) / r_in.direction.length
        else:
            outward_normal = hit.normal
            ni_over_nt = 1.0 / self.ref_idx
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length

        refracted = refract(r_in.direction, outward_normal, ni_over_nt)
        if not refracted:
            reflect_prob = 1.0
        else:
            reflect_prob = schlick(cosine, self.ref_idx)

        if random() < reflect_prob:
            return ScatterRecord(attenuation, Ray(hit.p, reflected))

        return ScatterRecord(attenuation, Ray(hit.p, refracted))
