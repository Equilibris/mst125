# import turtle, 
import time, math

GRID_SIZE = 30

class Matrix:
    @classmethod
    def from_rot(cls,rot):
        cos = math.cos(rot)
        sin = math.sin(rot)

        return cls((cos, sin), (-sin, cos))

    def __init__(self, a, b):
        self.a = Vec(a)
        self.b = Vec(b)

    def __mul__(self, other):
        if isinstance(other, Matrix):
            return Matrix (
                    (self.a.x * other.a.x + self.b.x * other.a.x,
                     self.a.y * other.a.x + self.b.y * other.a.y),
                    (self.a.x * other.b.x + self.b.y * other.a.y,
                     self.a.y * other.a.x + self.b.y * other.a.y),
                    )

        return NotImplemented

class Vec:
    @classmethod
    def from_deg(cls, d, m = 1):
        return cls((
            m * math.cos(math.radians(d)),
            m * math.sin(math.radians(d))
        ))

    @classmethod
    def bivector_balance(cls, base, angle_a, angle_b):
        from math import sin, cos

        ca, sa = cos(math.radians(angle_a)), sin(math.radians(angle_a))
        cb, sb = cos(math.radians(angle_b)), sin(math.radians(angle_b))

        cx, cy = base.base

        # Given the set of eqs [cx = a ca + b cb, cy = a sa + b sb] where sx = sinx and cx = cos
        #
        # a = (cx - b cb) / ca
        # cy = (cx / ca - b cb / ca) sa + b sb
        # cy = sa cx / ca - sa b cb / ca + b sb
        # cy - sa cx / ca = b (- sa cb / ca + sb)
        # (cy - sa cx / ca) / (- sa cb / ca + sb) = b

        b = (cy - sa * cx / ca) / (- sa * cb / ca + sb)
        a = (cx - b * cb) / ca

        return cls.from_deg(angle_a, a), cls.from_deg(angle_b, b)

    def __init__(self, base, offset = (0,0)):
        self.offset = offset
        self.base = base

    def __repr__(self) -> str:
        if self.offset == (0,0):
            return f'{self.__class__.__name__}({self.base})'
        return f'{self.__class__.__name__}({self.base}, {self.offset})'

    @property
    def x(self):
        return self.base[0]
    @property
    def y(self):
        return self.base[1]

    def __add__(self, other):
        return Vec((self.x + other.x, self.y + other.y))

    def __mul__(self, other):
        if isinstance(other, Matrix):
            return other.a * self.x + other.b * self.y

        elif isinstance(other,int):
            return Vec((self.base[0] * other, self.base[1] * other))

        elif isinstance(other,float):
            return Vec((self.base[0] * other, self.base[1] * other))

        return NotImplemented

    def __str__(self):
        return '[{}, {}]'.format(*self.base)

    def shift(self,offset):
        return Vec(self.base, offset = offset.base)

    def magnitude(self):
        return math.sqrt(self.x * self.x + self.y * self.y)

    def angle(self):
        return math.atan2(self.y, self.x)

    def draw(self):
        global GRID_SIZE

        turtle.penup()

        a, b = self.offset[0] * GRID_SIZE, self.offset[1] * GRID_SIZE

        turtle.goto(a, b)

        turtle.pendown()
        turtle.goto(a + self.x * GRID_SIZE, b + self.y * GRID_SIZE)

if __name__ == "__main__":
    mat = Matrix.from_rot(math.pi/2)

    a = Vec((2,2)) * mat
    b = Vec((1,-2)) * mat

    a.draw()
    b.draw()

    a.shift(b).draw()
    b.shift(a).draw()
    (a+b).draw()
    print(a+b)

