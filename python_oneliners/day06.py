# part 1
print(sum(len(set(group.replace('\n', '')))
          for group in open('../inputs/day06.txt').read().split('\n\n')))

# part 2
print(sum(sum(all(c in g for g in group) for c in group[0]) for group in (
    groups.split() for groups in open('../inputs/day06.txt').read().split('\n\n'))))
