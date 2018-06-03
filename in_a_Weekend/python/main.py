import math
import sys
from random import random

from tqdm import tqdm

from hitable import HitRecord, HitableList, Sphere
from vec3 import Vec3
from ray import Ray
from camera import Camera
from material import Lambertian, Metal, Dielectric

def color(r, world, depth=0):
    rec = HitRecord()
    if world.hit(r, 0.001, sys.float_info.max, rec):
        scattered = Ray()
        attenuation = Vec3()
        if depth < 50 and rec.material.scatter(r, rec, attenuation, scattered):
            return attenuation*color(scattered, world, depth+1)
        return Vec3()

    unit_direction = Vec3.Unit(r.direction)
    t = 0.5*(unit_direction.y + 1.0)
    return (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)

def random_scene():
    hitables = []
    hitables.append(Sphere(Vec3(0,-1000,0), 1000, Lambertian(Vec3(0.5, 0.5, 0.5))))
    for a in range(-11, 11):
        for b in range(-11, 11):
            choose_mat = random()
            center = Vec3(a+0.9*random(), 0.2, b+0.9*random())
            if (center-Vec3(4,0.2,0)).length > 0.9:
                if choose_mat < 0.8:
                    hitables.append(Sphere(center, 0.2, Lambertian(Vec3(random()*random(), random()*random(), random()*random()))))
                elif choose_mat < 0.95:
                    hitables.append(Sphere(center, 0.2, Metal(Vec3(0.5*(1 + random()), 0.5*(1 + random()), 0.5*(1+random())), 0.5*random())))
                else:
                    hitables.append(Sphere(center, 0.2, Dielectric(1.5)))
    hitables.append(Sphere(Vec3(0, 1, 0), 1.0, Dielectric(1.5)))
    hitables.append(Sphere(Vec3(-4, 1, 0), 1.0, Lambertian(Vec3(0.4, 0.2, 0.1))))
    hitables.append(Sphere(Vec3(4, 1, 0), 1.0, Metal(Vec3(0.7, 0.6, 0.5), 0.0)))
    return HitableList(hitables)

def main():
    nx, ny, ns = 200, 100, 100
    print 'P3'
    print nx, ny
    print 255
    world = random_scene()
    lookfrom = Vec3(13,2,3)
    lookat = Vec3(0,0,0)
    dist_to_focus = 10.0
    aperture = 0.1
    cam = Camera(lookfrom, lookat, Vec3(0,1,0), 20, float(nx)/ny, aperture, dist_to_focus)
    for j in tqdm(range(ny)):
        for i in range(nx):
            col = Vec3(0, 0, 0)
            for s in range(ns):
                u, v = (i+random())/nx, ((ny-1-j)+random())/ny
                r = cam.getRay(u, v)
                col += color(r, world)
            col /= float(ns)
            col = Vec3(math.sqrt(col[0]), math.sqrt(col[1]), math.sqrt(col[2]))
            ir, ig, ib = int(255.99*col[0]), int(255.99*col[1]), int(255.99*col[2])
            print ir, ig, ib

if __name__ == '__main__':
    main()
