def main():
    nx, ny = 200, 100
    print 'P3'
    print nx, ny
    print 255
    for j in reversed(range(ny)):
        for i in range(nx):
            r, g, b = i/float(nx), j/float(ny), 0.2
            ir, ig, ib = int(255.99*r), int(255.99*g), int(255.99*b)
            print ir, ig, ib

if __name__ == '__main__':
    main()
