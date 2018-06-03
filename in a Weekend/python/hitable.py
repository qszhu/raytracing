import math

from vec3 import Vec3

class HitRecord(object):

    def __init__(self):
        self.t = None
        self.p = None
        self.normal = None

    def update(self, other):
        self.t = other.t
        self.p = other.p
        self.normal = other.normal

class Hitable(object):

    def hit(self, r, t_min, t_max, rec):
        raise NotImplementedError

class HitableList(Hitable):

    def __init__(self, l=None):
        if l is None: l = []
        self.list = l

    def hit(self, r, t_min, t_max, rec):
        temp_rec = HitRecord()
        hit_anything = False
        closest_so_far = t_max
        for hitable in self.list:
            if hitable.hit(r, t_min, closest_so_far, temp_rec):
                hit_anything = True
                closest_so_far = temp_rec.t
                rec.update(temp_rec)
        return hit_anything

class Sphere(Hitable):

    def __init__(self, cen=None, r=0.0):
        if cen is None: cen = Vec3()
        self.center = cen
        self.radius = r

    def hit(self, r, t_min, t_max, rec):
        def updateRec(t):
            if not (t_min < t < t_max):
                return False
            rec.t = t
            rec.p = r.pointAt(rec.t)
            rec.normal = (rec.p - self.center) / self.radius
            return True

        oc = r.origin - self.center
        a = Vec3.Dot(r.direction, r.direction)
        b = Vec3.Dot(oc, r.direction)
        c = Vec3.Dot(oc, oc) - self.radius*self.radius
        discriminant = b*b - a*c
        if discriminant > 0:
            temp = (-b - math.sqrt(discriminant))/a
            if updateRec(temp): return True
            temp = (-b + math.sqrt(discriminant))/a
            if updateRec(temp): return True
        return False
