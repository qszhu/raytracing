import sys

from hitable import HitRecord, HitableList, Sphere
from vec3 import Vec3
from ray import Ray

def color(r, world):
    rec = HitRecord()
    if world.hit(r, 0.0, sys.float_info.max, rec):
        return 0.5*Vec3(rec.normal.x+1, rec.normal.y+1, rec.normal.z+1)

    unit_direction = Vec3.Unit(r.direction)
    t = 0.5*(unit_direction.y + 1.0)
    return (1.0-t)*Vec3(1.0, 1.0, 1.0) + t*Vec3(0.5, 0.7, 1.0)

def main():
    nx, ny = 200, 100
    print 'P3'
    print nx, ny
    print 255
    lower_left_corner = Vec3(-2.0, -1.0, -1.0)
    horizontal = Vec3(4.0, 0.0, 0.0)
    vertical = Vec3(0.0, 2.0, 0.0)
    origin = Vec3(0.0, 0.0, 0.0)
    world = HitableList([
        Sphere(Vec3(0,0,-1), 0.5),
        Sphere(Vec3(0,-100.5,-1), 100)
    ])
    for j in reversed(range(ny)):
        for i in range(nx):
            u, v = i/float(nx), j/float(ny)
            r = Ray(origin, lower_left_corner + u*horizontal + v*vertical)
            col = color(r, world)
            ir, ig, ib = int(255.99*col[0]), int(255.99*col[1]), int(255.99*col[2])
            print ir, ig, ib

if __name__ == '__main__':
    main()
