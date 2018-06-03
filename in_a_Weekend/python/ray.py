from vec3 import Vec3

class Ray(object):

    def __init__(self, a=None, b=None):
        if a is None: a = Vec3()
        if b is None: b = Vec3()
        self.A = a
        self.B = b

    @property
    def origin(self):
        return self.A

    @property
    def direction(self):
        return self.B

    def pointAt(self, t):
        return self.A + t*self.B

    def update(self, other):
        self.A.update(other.A)
        self.B.update(other.B)

if __name__ == '__main__':
    r = Ray()
    assert r.origin == Vec3(0, 0, 0)
    assert r.direction == Vec3(0, 0, 0)

    a = Vec3(1, 2, 3)
    b = Vec3(4, 5, 6)
    r = Ray(a, b)
    assert r.origin == a
    assert r.direction == b

    c = r.pointAt(2)
    assert c == Vec3(9, 12, 15)

    print 'ok'
