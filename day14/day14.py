from dataclasses import dataclass, field


@dataclass
class DecoderV1:
    mask: str = 'X' * 36
    memory: dict = field(default_factory=dict)

    def set_value(self, index, value):
        self.memory[index] = apply_mask(self.mask, value)


@dataclass
class DecoderV2:
    mask: str = 'X' * 36
    memory: dict = field(default_factory=dict)

    def set_value(self, index, value):
        for variant in variants(self.mask):
            inx = apply_mask2(variant, index)
            self.memory[inx] = value


def variants(s):
    c = [s[0]] if s[0] != 'X' else ['0', '1']
    if not s[1:]:
        yield from c
    else:
        for i in c:
            for j in variants(s[1:]):
                yield i + j


def apply_mask(mask, value):
    for (inx, c) in enumerate(mask[::-1]):
        if c == 'X':
            continue
        if c == '0':
            value &= ~(1 << inx)
        elif c == '1':
            value |= (1 << inx)
        else:
            raise Exception(f"got {c}")
    return value


def apply_mask2(mask, value):
    # TODO: this isn't right
    for (inx, c) in enumerate(mask[::-1]):
        if c in 'X0':
            continue
        elif c == '1':
            value |= (1 << inx)
        else:
            raise Exception(f"got {c}")
    return value


def part1(input):
    decoder = DecoderV1()

    for line in input:
        if line.startswith("mem"):
            parts = line[4:].split("] = ")
            index = int(parts[0])
            value = int(parts[1])

            decoder.set_value(index, value)
        else:
            decoder.mask = line.split(" = ")[1].strip()

    return sum(decoder.memory.values())


def part2(input):
    decoder = DecoderV2()

    for line in input:
        if line.startswith("mem"):
            parts = line[4:].split("] = ")
            index = int(parts[0])
            value = int(parts[1])

            decoder.set_value(index, value)
        else:
            decoder.mask = line.split(" = ")[1].strip()

    return sum(decoder.memory.values())


def main():
    m = '000000000000000000000000000000X1001X'
    i = 42
    for v in variants(m):
        print(apply_mask2(v, i))

    with open('../inputs/day14-test.txt', 'r') as f:
        print(part1(f))

    with open('../inputs/day14-test2.txt', 'r') as f:
        print(part2(f))


if __name__ == '__main__':
    main()
