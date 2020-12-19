
def tag(c):
    def inner(s):
        if s[0] == c:
            return s[1:]
        return None
    inner.__name__ = f"tag({c})"
    return inner


class Validator:

    def __init__(self):
        self.rules = {}

    def add_rule(self, rule_string):
        id, match = rule_string.split(': ')
        if '"' in match:
            self.rules[id] = tag(match[1:-1])
        elif "|" in match:
            l, r = match.split(' | ')
            self.rules[id] = self.either(
                self.seq(l.split(" ")),
                self.seq(r.split(" "))
            )
        elif " " in match:
            self.rules[id] = self.seq(match.split(" "))
        else:
            self.rules[id] = self.match(match)
        #print(f"{id}: {match} -> {self.rules[id].__name__}")

    def match(self, rule):
        def inner(s):
            return self.rules[rule](s)
        inner.__name__ = f'match({rule})'
        return inner

    def seq(self, rules):
        def inner(s):
            t = s
            for r in rules:
                if not t:
                    return None
                t = self.match(r)(t)
            return t
        inner.__name__ = f"seq({rules})"
        return inner

    def either(self, first, second):
        def inner(s):
            t = first(s)
            if t is None:
                return second(s)
            return t
        inner.__name__ = f"either({first.__name__},{second.__name__})"
        return inner

    def validate(self, string):
        return self.match('0')(string) == ''

    def rule11_recursive(self, string):
        i = 1
        while True:
            s = self.seq(['42'] * i)(string)
            if s is None:
                return None
            t = self.seq(['31'] * i)(s)
            if t == '':
                return t
            i += 1

    def validate_part2(self, string):
        i = 1
        while True:
            s = self.seq(['42'] * i)(string)
            if s is None:
                return False
            t = self.rule11_recursive(s)
            if t == '':
                return True
            i += 1


def main():
    with open("../inputs/day19.txt", 'r') as f:
        rules, messages = f.read().split("\n\n")

    validator = Validator()
    for rule in rules.splitlines():
        validator.add_rule(rule)

    #print(validator.rules)

    #for m in messages.splitlines():
    #    print(f"{m} -> {validator.validate_part2(m)}")

    print(sum(validator.validate_part2(m) for m in messages.splitlines()))


if __name__ == '__main__':
    main()