import re

full_regex = re.compile(
    r"(\w+ \w+) bags contain (no other bags|.+)\."
)
child_regex = re.compile(r"(\d+) (\w+ \w+)")


def part1(rules, target):

    seen = set()
    to_check = [target]

    while to_check:
        colour = to_check.pop(0)

        for key, rule in rules.items():
            if any(colour in children for children in rule):
                if key not in seen:
                    to_check.append(key)
                    seen.add(key)

    return len(seen)


def _contains(rules, target):
    if not rules[target]:
        return 1
    else:
        return sum(count * _contains(rules, colour) for count, colour in rules[target]) + 1


def part2(rules, target):
    # -1 because we don't count the outer bag itself
    return _contains(rules, target) - 1


def parse_rules(lines):
    rule_dict = {}
    for rule in lines:
        groups = full_regex.match(rule).groups()
        # print(rule)
        # print(groups)
        key = groups[0]
        children = []
        if groups[1] != 'no other bags':
            for (count, name) in child_regex.findall(groups[1]):
                children.append((int(count), name))
        rule_dict[key] = children
    return rule_dict


def main():
    with open('../inputs/day07.txt', 'r') as f:
        lines = f.read().splitlines()

    rules = parse_rules(lines)

    # print(rules)

    print(part1(rules, 'shiny gold'))
    print(part2(rules, 'shiny gold'))


if __name__ == '__main__':
    main()
