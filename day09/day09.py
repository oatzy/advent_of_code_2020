
def isvalid(value, preamble):
    for i, x in enumerate(preamble):
        for j in range(i+1, len(preamble)):
            if x + preamble[j] == value:
                return True
    return False


def find_invalid(values, size):
    for i, x in enumerate(values[size:]):
        if not isvalid(x, values[i:i+size]):
            return x
    raise Exception("not found")


def is_contiguous_sum_from(target, values, offset):
    origin = offset
    while sum(values[origin:offset+1]) < target:
        offset += 1
    sub = values[origin:offset+1]
    if sum(sub) == target:
        return min(sub) + max(sub)
    return None


def find_contiguous_sum(target, values):
    for offset in range(len(values)):
        check = is_contiguous_sum_from(target, values, offset)
        if check is not None:
            return check
    raise Exception("not found")


def find_contiguous_sum_alt(target, values):
    start = 0
    end = 1

    while end < len(values):
        s = sum(values[start:end])
        if s == target:
            break
        elif s < target:
            end += 1
        elif s > target:
            start += 1

    if sum(values[start:end]) != target:
        raise Exception("not found")
    return min(values[start:end]) + max(values[start:end])


def main():
    with open('inputs/day09.txt', 'r') as f:
        values = [int(l) for l in f]

    target = find_invalid(values, 25)
    print(target)
    print(find_contiguous_sum_alt(target, values))


if __name__ == '__main__':
    main()
