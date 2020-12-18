from operator import add, mul


def _evaluate(string, inx):
    value = None
    op = None

    while inx < len(string):
        c = string[inx]

        if c.isdigit():
            if value is None:
                value = int(c)
            else:
                value = op(value, int(c))
                op = None

        elif c == '+':
            op = add
        elif c == '*':
            op = mul

        elif c == '(':
            v, inx = _evaluate(string, inx+1)
            if value is None:
                value = v
            else:
                value = op(value, v)
                op = None

        elif c in ')\n':
            break
        elif c in ' ':
            pass
        else:
            raise Exception(f"unknown char {c}")

        inx += 1

    return value, inx


def evaluate(string):
    return _evaluate(string, 0)[0]


def main():
    with open('../inputs/day18.txt', 'r') as f:
        print(sum(evaluate(l) for l in f))


if __name__ == '__main__':
    #print(evaluate("1 + 2 * 3 + 4 * 5 + 6"))
    main()
