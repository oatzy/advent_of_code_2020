
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

    def load(self, rule_strings):
        for rule in rule_strings.splitlines():
            id, match = rule.split(': ')
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
            print(f"{id}: {match} -> {self.rules[id].__name__}")

    def match(self, rule):
        def inner(s):
            return self.rules[rule](s)
        inner.__name__ = f'match({rule})'
        return inner

    def seq(self, rules):
        def inner(s):
            t = s
            for r in rules:
                t = self.match(r)(t)
                if t is None:
                    return None
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


def main():
    with open("../inputs/day19.txt", 'r') as f:
        rules, messages = f.read().split("\n\n")
    validator = Validator()
    validator.load(rules)

    #print(validator.rules)

    # for m in messages.splitlines():
    #     print(f"{m} -> {validator.validate(m)}")

    print(sum(validator.validate(m) for m in messages.splitlines()))


if __name__ == '__main__':
    main()