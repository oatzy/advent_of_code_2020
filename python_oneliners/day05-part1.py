print(max(sum((1 << i) if c in 'BR' else 0 for i, c in enumerate(
    reversed(l.strip()))) for l in open("../inputs/day05.txt")))
