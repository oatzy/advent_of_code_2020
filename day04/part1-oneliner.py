required = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid",
    # "cid",
]

print(sum(all(f in s for f in required) for s in open("../inputs/day04.txt").read().split("\n\n")))