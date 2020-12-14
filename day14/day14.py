from dataclasses import dataclass, field


@dataclass
class Decoder:
    mask: str = 'X' * 36
    memory: dict = field(default_factory=dict)


class DecoderV1(Decoder):

    def set_value(self, index, value):
        self.memory[index] = apply_mask(self.mask, value)


class DecoderV2(Decoder):

    def set_value(self, index, value):
        index = apply_mask(self.mask.replace('0', 'X'), index)
        for inx in apply_variants(self.mask, index):
            self.memory[inx] = value


def apply_variants(mask, value):
    values = [value]
    for i, c in enumerate(mask[::-1]):
        if c != 'X':
            continue
        tmp = []
        for v in values:
            tmp.append(v & ~(1<<i))
            tmp.append(v | (1<<i))
        values = tmp
    return values


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


def execute(decoder, input):
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
    with open('../inputs/day14.txt', 'r') as f:
        input = f.read().splitlines()

    print(execute(DecoderV1(), input))
    print(execute(DecoderV2(), input))



if __name__ == '__main__':
    main()
