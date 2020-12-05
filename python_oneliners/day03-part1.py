# python 2 only
# print(reduce(lambda (l, w, t), (i, c): (l+1 if c == '\n' else l, i if c == '\n' and not w else w, t+1 if c == '#' and (i == 0 or w != 0 and i == l*(w+1)+(3*l % w)) else t), enumerate(open("../inputs/day03.txt").read()), (0, 0, 0))[2])

# python 3
from functools import reduce
print(reduce(lambda lwt, ic: (lwt[0]+1 if ic[1] == '\n' else lwt[0], ic[0] if ic[1] == '\n' and not lwt[1] else lwt[1], lwt[2]+1 if ic[1] == '#' and (
    ic[0] == 0 or lwt[1] != 0 and ic[0] == lwt[0]*(lwt[1]+1)+(3*lwt[0] % lwt[1])) else lwt[2]), enumerate(open("../inputs/day03.txt").read()), (0, 0, 0))[2])
