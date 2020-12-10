#print((lambda x: x[1] * x[3])(reduce(lambda (l, a, b, c), x: (x, a + (x - l == 1), b + (x - l == 2), c + (x - l == 3)), sorted(map(int, open('../inputs/day10.txt'))), (0, 0, 0, 1))))

from functools import reduce
print((lambda x: x[1] * x[3])(reduce(lambda a, x: (x, a[1] + (x - a[0] == 1), a[2] + (x - a[0] == 2), a[3] + (x - a[0] == 3)), sorted(map(int, open('../inputs/day10.txt'))), (0, 0, 0, 1))))
