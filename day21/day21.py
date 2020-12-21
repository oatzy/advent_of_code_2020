from collections import Counter
from functools import reduce


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



if __name__ == "__main__":
    main()