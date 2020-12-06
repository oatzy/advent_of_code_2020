# part 1
import re
print(sum(int(l) <= s.count(c) <= int(h) for (l, h, c, s) in (
    re.match(r"(\d+)\-(\d+) (\w): (\w+)", line).groups() for line in open("../inputs/day02.txt"))))

# part 2
print(sum((s[int(l)-1] == c) ^ (s[int(h)-1] == c)
          for (l, h, c, s) in (re.match(r"(\d+)\-(\d+) (\w): (\w+)", line).groups() for line in open("../inputs/day02.txt"))))
