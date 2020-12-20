from functools import reduce

def get_edges(lines):
    edges = {lines[0], lines[-1], lines[0][::-1], lines[-1][::-1]}
    left = "".join(l[0] for l in lines)
    edges.update([left, left[::-1]])
    right = "".join(l[-1] for l in lines)
    edges.update([right, right[::-1]])
    return edges


def main():
    with open("../inputs/day20.txt", 'r') as f:
        input = f.read()

    edge_map = {}

    for tile in input.split("\n\n"):
        tile = tile.splitlines()
        id = int(tile[0][5:-1])

        edge_map[id] = get_edges(tile[1:])

    print(edge_map)

    shared_edges = {}

    for id, edges in edge_map.items():
        for jd, jges in edge_map.items():
            if id == jd:
                continue
            if any(e in jges for e in edges):
                shared_edges.setdefault(id, []).append(jd)

    print(shared_edges)

    assert all(len(e) <= 4 for e in shared_edges.values())

    corners = [id for id, edges in shared_edges.items() if len(edges) == 2]
    print(corners)

    assert len(corners) == 4

    res = reduce(lambda x, y: x*y, corners)
    print(res)


if __name__ == '__main__':
    main()