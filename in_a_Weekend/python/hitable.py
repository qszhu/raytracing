import math

from vec3 import Vec3

class HitRecord(object):

    def __init__(self):
        self.t = None
        self.p = None
        self.normal = None
        self.material = None

class Hitable(object):

    def hit(self, r, t_min, t_max, rec):
        raise NotImplementedError

class HitableList(Hitable):

    def __init__(self, l=None):
        if l is None: l = []
        self.list = l

    def hit(self, r, t_min, t_max):
        res = False
        closest_so_far = t_max
        for hitable in self.list:
            h = hitable.hit(r, t_min, closest_so_far)
            if not h: continue
            closest_so_far = h.t
            res = h
        return res

class Sphere(Hitable):

    def __init__(self, cen=None, r=0.0, material=None):
        if cen is None: cen = Vec3()
        self.center = cen
        self.radius = r
        self.material = material

    def hit(self, r, t_min, t_max):
        def updateRec(t):
            if not (t_min < t < t_max):
                return False
            res = HitRecord()
            res.t = t
            res.p = r.pointAt(t)
            res.normal = (res.p - self.center) / self.radius
            res.material = self.material
            return res

        oc = r.origin - self.center
        a = r.direction.dot(r.direction)
        b = oc.dot(r.direction)
        c = oc.dot(oc) - self.radius*self.radius
        discriminant = b*b - a*c
        if discriminant > 0:
            res = updateRec((-b - math.sqrt(discriminant))/a)
            if not res:
                res = updateRec((-b + math.sqrt(discriminant))/a)
            if res: return res

        return False
