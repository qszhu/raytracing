from vec3 import Vec3
from ray import Ray

def hit_sphere(center, radius, r):
    oc = r.origin - center
    a = Vec3.Dot(r.direction, r.direction)
    b = 2.0 * Vec3.Dot(oc, r.direction)
    c = Vec3.Dot(oc, oc) - radius*radius
    discriminant = b*b - 4*a*c
    return discriminant > 0

def color(r):
    if hit_sphere(Vec3(0, 0, -1), 0.5, r):
        return Vec3(1, 0, 0)
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
    for j in reversed(range(ny)):
        for i in range(nx):
            u, v = i/float(nx), j/float(ny)
            r = Ray(origin, lower_left_corner + u*horizontal + v*vertical)
            col = color(r)
            ir, ig, ib = int(255.99*col[0]), int(255.99*col[1]), int(255.99*col[2])
            print ir, ig, ib

if __name__ == '__main__':
    main()
