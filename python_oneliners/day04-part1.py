print(sum(all(f in s for f in ("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"))
          for s in open("../inputs/day04.txt").read().split("\n\n")))
