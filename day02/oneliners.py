# part 1
import re; print(sum(int(m.group(1)) <= m.group(4).count(m.group(3)) <= int(m.group(2)) for m in (re.match(r"(\d+)\-(\d+) (\w): (\w+)", line) for line in open("../inputs/day02.txt"))))

# part 2
import re; print(sum((m.group(4)[int(m.group(1))-1] == m.group(3)) ^ (m.group(4)[int(m.group(2))-1] == m.group(3)) for m in (re.match(r"(\d+)\-(\d+) (\w): (\w+)", line) for line in open("../inputs/day02.txt"))))