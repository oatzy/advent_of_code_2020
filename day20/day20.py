from functools import reduce

from PIL import Image, ImageDraw

def get_edges(lines):
    edges = {lines[0], lines[-1], lines[0][::-1], lines[-1][::-1]}
    left = "".join(l[0] for l in lines)
    edges.update([left, left[::-1]])
    right = "".join(l[-1] for l in lines)
    edges.update([right, right[::-1]])
    return edges

def gen_image(lines):
    im = Image.new('1', (100, 100))
    draw = ImageDraw.Draw(im)
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            draw.rectangle((10*x, 10*y, (10*(x+1), 10*(y+1))), fill=(c=='.'))
    return im

def main():
    with open("../inputs/day20.txt", 'r') as f:
        input = f.read()

    edge_map = {}

    for tile in input.split("\n\n"):
        tile = tile.splitlines()
        id = int(tile[0][5:-1])

        edge_map[id] = get_edges(tile[1:])

        # im = gen_image(tile[1:])
        # im.save(f"tiles/{id}.png")

    # print(edge_map)
    print(len(edge_map))

    shared_edges = {}

    for id, edges in edge_map.items():
        for jd, jges in edge_map.items():
            if id == jd:
                continue
            if any(e in jges for e in edges):
                shared_edges.setdefault(id, []).append(jd)

    # print(shared_edges)
    # for (id, edges) in sorted(shared_edges.items(), key=lambda x: len(x[1])):
    #     print(f"{id} -> {', '.join(str(e) for e in edges)}")

    assert all(len(e) <= 4 for e in shared_edges.values())

    corners = [id for id, edges in shared_edges.items() if len(edges) == 2]
    print(corners)

    assert len(corners) == 4

    res = reduce(lambda x, y: x*y, corners)
    print(res)


if __name__ == '__main__':
    main()