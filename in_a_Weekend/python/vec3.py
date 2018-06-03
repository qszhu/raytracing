import math

class Vec3(object):

    def __init__(self, e0=0, e1=0, e2=0):
        self.e = [e0, e1, e2]

    @property
    def x(self):
        return self.e[0]

    @property
    def y(self):
        return self.e[1]

    @property
    def z(self):
        return self.e[2]

    @property
    def r(self):
        return self.e[0]

    @property
    def g(self):
        return self.e[1]

    @property
    def b(self):
        return self.e[2]

    def __pos__(self):
        return self

    def __neg__(self):
        x, y, z = self.e
        return Vec3(-x, -y, -z)

    def __getitem__(self, idx):
        return self.e[idx]

    def __setitem__(self, idx, value):
        self.e[idx] = value

    def __add__(self, v2):
        x1, y1, z1 = self.e
        x2, y2, z2 = v2.e
        return Vec3(x1+x2, y1+y2, z1+z2)

    def __sub__(self, v2):
        x1, y1, z1 = self.e
        x2, y2, z2 = v2.e
        return Vec3(x1-x2, y1-y2, z1-z2)

    def __mul__(self, v2):
        x1, y1, z1 = self.e
        if isinstance(v2, Vec3):
            x2, y2, z2 = v2.e
            return Vec3(x1*x2, y1*y2, z1*z2)
        else:
            return Vec3(x1*v2, y1*v2, z1*v2)

    def __rmul__(self, v2):
        return self.__mul__(v2)

    def __div__(self, v2):
        x1, y1, z1 = self.e
        if isinstance(v2, Vec3):
            x2, y2, z2 = v2.e
            return Vec3(x1/float(x2), y1/float(y2), z1/float(z2))
        else:
            return Vec3(x1/float(v2), y1/float(v2), z1/float(v2))

    def __eq__(self, v2):
        x1, y1, z1 = self.e
        x2, y2, z2 = v2.e
        def close(a, b):
            return abs(a-b) <= 1e-8
        return close(x1, x2) and close(y1, y2) and close(z1, z2)

    @property
    def length(self):
        return math.sqrt(self.dot(self))

    @property
    def unitVector(self):
        return self / self.length

    def dot(self, v2):
        x1, y1, z1 = self.e
        x2, y2, z2 = v2.e
        return x1*x2 + y1*y2 + z1*z2

    def cross(self, v2):
        x1, y1, z1 = self.e
        x2, y2, z2 = v2.e
        return Vec3(
            y1*z2 - z1*y2,
            -(x1*z2 - z1*x2),
            x1*y2 - y1*x2
        )

    def update(self, other):
        self.e[0] = other.e[0]
        self.e[1] = other.e[1]
        self.e[2] = other.e[2]

    @staticmethod
    def Unit(v):
        return v.unitVector

    @staticmethod
    def Dot(v1, v2):
        return v1.dot(v2)

    @staticmethod
    def Cross(v1, v2):
        return v1.cross(v2)

    def __repr__(self):
        return 'Vec3({}, {}, {})'.format(self.e[0], self.e[1], self.e[2])

if __name__ == '__main__':
    v = Vec3()
    assert v[0] == 0 and v[1] == 0 and v[2] == 0

    v = Vec3(1, 2, 3)
    assert v[0] == 1 and v[1] == 2 and v[2] == 3

    v[0], v[1], v[2] = 4, 5, 6
    assert v[0] == 4 and v[1] == 5 and v[2] == 6

    v = Vec3(1, 2, 3)
    assert v.x == 1 and v.y == 2 and v.z == 3
    assert v.r == 1 and v.g == 2 and v.b == 3

    v = +v
    assert v.x == 1 and v.y == 2 and v.z == 3

    v = -v
    assert v.x == -1 and v.y == -2 and v.z == -3

    a, b = Vec3(1, 2, 3), Vec3(4, 5, 6)
    c = a + b
    assert c == Vec3(5, 7, 9)

    c = a - b
    assert c == Vec3(-3, -3, -3)

    c = a * b
    assert c == Vec3(4, 10, 18)

    c = a / b
    assert c == Vec3(0.25, 0.4, 0.5)

    c = a * 2
    assert c == Vec3(2, 4, 6)

    c = 2 * a
    assert c == Vec3(2, 4, 6)

    c = a / 2
    assert c == Vec3(0.5, 1, 1.5)

    a += b
    assert a == Vec3(5, 7, 9)

    a -= b
    assert a == Vec3(1, 2, 3)

    a *= b
    assert a == Vec3(4, 10, 18)

    a /= b
    assert a == Vec3(1, 2, 3)

    a *= 3
    assert a == Vec3(3, 6, 9)

    a /= 3
    assert a == Vec3(1, 2, 3)

    a = Vec3(3, 0, -4)
    assert a.length == 5
    assert a.unitVector == Vec3(0.6, 0, -0.8)

    c = a.dot(b)
    assert c == -12

    c = a.cross(b)
    assert c == Vec3(20, -34, 15)
    print 'ok'
