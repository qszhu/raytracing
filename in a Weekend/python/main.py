from vec3 import Vec3

def main():
    nx, ny = 200, 100
    print 'P3'
    print nx, ny
    print 255
    for j in reversed(range(ny)):
        for i in range(nx):
            col = Vec3(i/float(nx), j/float(ny), 0.2)
            ir, ig, ib = int(255.99*col[0]), int(255.99*col[1]), int(255.99*col[2])
            print ir, ig, ib

if __name__ == '__main__':
    main()
