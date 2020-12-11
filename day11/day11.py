
def adjacent(p):
    for x in (-1, 0, 1):
        for y in (-1, 0, 1):
            a, b = p[0] + x, p[1] + y
            if a < 0 or b < 0 or (a, b) == p:
                continue
            yield (a, b)


def iterate(seats):
    new = {}

    for p, s in seats.items():
        occupied = sum(seats.get(q) == '#' for q in adjacent(p))
        if s == 'L' and occupied == 0:
            new[p] = '#'
        elif s == '#' and occupied >= 4:
            new[p] = 'L'
        else:
            new[p] = s

    return new


def find_fixed_point(seats):
    old = seats

    while True:
        new = iterate(old)
        if new == old:
            return new
        old = new


def main():
    seats = {}

    with open('../inputs/day11.txt') as f:
        for (y, line) in enumerate(f):
            for (x, c) in enumerate(line):
                if c in 'L#':
                    seats[(x, y)] = c

    fixed = find_fixed_point(seats)

    print(sum(v == '#' for v in fixed.values()))


if __name__ == '__main__':
    main()
