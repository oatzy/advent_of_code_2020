print((lambda seen: next(i for i in (x * (2020 - x) if (2020 - x) in seen else seen.add(x)
                                     for x in map(int, open("../inputs/day01.txt"))) if i is not None))(set()))
