import sys
from random import random

from hitable import HitRecord, HitableList, Sphere
from vec3 import Vec3
from camera import Camera

def color(r, world):
    rec = HitRecord()
    if world.hit(r, 0.0, sys.float_info.max, rec):
        return 0.5*Vec3(rec.normal.x+1, rec.normal.y+1, rec.normal.z+1)

    unit_direction = Vec3.Unit(r.direction)
    t = 0.5*(unit_direction.y + 1.0)
    return (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)

def main():
    nx, ny, ns = 200, 100, 100
    print 'P3'
    print nx, ny
    print 255
    world = HitableList([
        Sphere(Vec3(0,0,-1), 0.5),
        Sphere(Vec3(0,-100.5,-1), 100)
    ])
    cam = Camera()
    for j in reversed(range(ny)):
        for i in range(nx):
            col = Vec3(0, 0, 0)
            for s in range(ns):
                u, v = (i+random())/nx, (j+random())/ny
                r = cam.getRay(u, v)
                col += color(r, world)
            col /= float(ns)
            ir, ig, ib = int(255.99*col[0]), int(255.99*col[1]), int(255.99*col[2])
            print ir, ig, ib

if __name__ == '__main__':
    main()
