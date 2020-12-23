from collections import deque


def play(cups, rounds=1):
    position = 0

    for i in range(rounds):
        if i % 10_000 == 0:
            print(f"round {i}")

        current = cups[0]

        cups.rotate(-1)

        temp = []
        for _ in range(3):
            temp.append(cups.popleft())

        dest = current - 1 if current > 1 else len(cups) + 3
        while dest in temp:
            dest = dest - 1 if dest > 1 else len(cups) + 3

        position = cups.index(dest)

        for i in temp[::-1]:
            cups.insert((position + 1) % len(cups), i)


def play_list(cups, rounds=1):
    head = 0
    l = len(cups)
    temp = [0, 0, 0]

    for i in range(rounds):
        if i % 10_000 == 0:
            print(f"round {i}")

        current = cups[head]

        offset = (head + 1) % l

        for i in range(3):
            temp[i] = cups[(offset + i) % l]

        dest = current - 1 if current > 1 else l
        while dest in temp:
            dest = dest - 1 if dest > 1 else l

        while offset != head:
            cups[offset] = cups[(offset + 3) % l]
            if cups[offset] == dest:
                break
            offset = (offset + 1) % l

        for (i, x) in enumerate(temp):
            cups[(offset + i + 1) % l] = x

        head = (head + 1) % l


def main():
    cups = [9, 6, 2, 7, 1, 3, 8, 5, 4]
    cups.extend(range(10, 1_000_001))

    play_list(cups, 10_000_000)

    print(cups)


if __name__ == '__main__':
    main()
