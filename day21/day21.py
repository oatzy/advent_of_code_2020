from collections import Counter
from functools import reduce


def resolve(allergens):
    all_set = False
    while not all_set:
        all_set = True
        for k, v in allergens.items():
            if isinstance(v, str):
                continue
            all_set = False
            if len(v) == 1:
                v = list(v)[0]
                allergens[k] = v
                for l, w in allergens.items():
                    if l != k and v in w:
                        w.remove(v)
    return allergens


def main():
    allergens = {}
    ingredients = Counter()

    with open("../inputs/day21.txt", 'r') as f:
        foods = f.read().splitlines()

    for food in foods:
        ings, alls = food[:-1].split(' (contains ')
        ings = ings.split()
        ingredients.update(ings)

        for a in alls.split(", "):
            if a in allergens:
                allergens[a] = allergens[a].intersection(ings)
            else:
                allergens[a] = set(ings)

    allergenic = reduce(lambda x, y: x | y, allergens.values())

    # print(allergens)
    # print(ingredients)

    print(sum(c for i, c in ingredients.items() if i not in allergenic))

    # for k, v in allergens.items():
    #     print(f"{k} -> {','.join(v)}")

    allergens = resolve(allergens)
    print(",".join(v for _, v in sorted(allergens.items(), key=lambda x: x[0])))



if __name__ == "__main__":
    main()