from enum import Enum
from functools import reduce

from PIL import Image, ImageDraw


MONSTER = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   "
]

def coords(lines):
    monster = []
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            if c == '#':
                monster.append((x,y))
    return monster


class Tile:

    def __init__(self, id, lines):
        self.id = id
        self.lines = lines
        self.edges = get_edges(lines)

    @property
    def right(self):
        return next(i for i, e in self.edges.items() if e == Edge.right)

    @property
    def bottom(self):
        return next(i for i, e in self.edges.items() if e == Edge.bottom)

    def flip_vertical(self):
        self.lines = self.lines[::-1]
        self.edges = get_edges(self.lines)

    def flip_horizontal(self):
        self.lines = [l[::-1] for l in self.lines]
        self.edges = get_edges(self.lines)

    def rotate90cw(self):
        self.lines = [
            "".join(l[i] for l in self.lines)[::-1]
            for i in range(len(self.lines[0]))
        ]
        self.edges = get_edges(self.lines)

    def rotate90cc(self):
        self.rotate90cw()
        self.rotate90cw()
        self.rotate90cw()


class Edge(Enum):
    top = 1
    right = 2
    bottom = 3
    left = 4
    top_flip = 5
    right_flip = 6
    bottom_flip = 7
    left_flip = 8


def get_edges(lines):
    edges = {
        lines[0]: Edge.top,
        lines[-1]: Edge.bottom,
        lines[0][::-1]: Edge.top_flip,
        lines[-1][::-1]: Edge.bottom_flip
    }
    left = "".join(l[0] for l in lines)
    edges.update({
        left: Edge.left,
        left[::-1]: Edge.left_flip
    })
    right = "".join(l[-1] for l in lines)
    edges.update({
        right: Edge.right,
        right[::-1]: Edge.right_flip
    })
    return edges

def find_match(tiles, edge, id):
    for t in tiles.values():
        if t.id == id:
            continue
        if edge in t.edges:
            return t, t.edges[edge]
    raise Exception("No match")

def rotflip_right(tile, edge):
    if edge is Edge.top:
        tile.rotate90cw()
        tile.flip_horizontal()
    elif edge is Edge.right:
        tile.flip_horizontal()
    elif edge is Edge.bottom:
        tile.rotate90cw()
    elif edge is Edge.left:
        pass
    elif edge is Edge.top_flip:
        tile.rotate90cc()
    elif edge is Edge.right_flip:
        tile.flip_horizontal()
        tile.flip_vertical()
    elif edge is Edge.bottom_flip:
        tile.flip_horizontal()
        tile.rotate90cw()
    elif edge is Edge.left_flip:
        tile.flip_vertical()


def rotflip_bottom(tile, edge):
    if edge is Edge.top:
        pass
    elif edge is Edge.right:
        tile.rotate90cc()
    elif edge is Edge.bottom:
        tile.flip_vertical()
    elif edge is Edge.left:
        tile.rotate90cw()
        tile.flip_horizontal()
    elif edge is Edge.top_flip:
        tile.flip_horizontal()
    elif edge is Edge.right_flip:
        tile.rotate90cw()
        tile.flip_vertical()
    elif edge is Edge.bottom_flip:
        tile.flip_horizontal()
        tile.flip_vertical()
    elif edge is Edge.left_flip:
        tile.rotate90cw()

def gen_tile_image(lines):
    im = Image.new('1', (100, 100))
    draw = ImageDraw.Draw(im)
    for y, line in enumerate(lines):
        for x, c in enumerate(line):
            draw.rectangle((10*x, 10*y, (10*(x+1), 10*(y+1))), fill=(c=='.'))
    return im

def gen_image(pixels):
    im = Image.new('1', (1000, 1000))
    draw = ImageDraw.Draw(im)
    for y in range(100):
        for x in range(100):
            draw.rectangle((10*x, 10*y, (10*(x+1), 10*(y+1))), fill=((x,y) not in pixels))
    return im

def transformations(lines):
    # noop
    yield coords(MONSTER)
    # flip vertical
    yield coords(MONSTER[::-1])
    # flip horizontal
    yield coords([l[::-1] for l in MONSTER])
    # rot180
    yield coords([l[::-1] for l in MONSTER][::-1])
    # rotate 90 CW
    rot90 = ["".join(l[i] for l in lines)[::-1]
        for i in range(len(lines[0]))]
    yield coords(rot90)
    # rot90 CCW
    yield coords(rot90[::-1])
    yield coords([l[::-1] for l in rot90])
    yield coords([l[::-1] for l in rot90][::-1])


def find_monsters(pixels, monster):
    found = 0
    for xoff in range(110):
        for yoff in range(110):
            if all((x+xoff, y+yoff) in pixels for x,y in monster):
                found += 1
    return found


def main():
    with open("../inputs/day20.txt", 'r') as f:
        input = f.read()

    tiles = {}

    for tile in input.split("\n\n"):
        tile = tile.splitlines()
        id = int(tile[0][5:-1])

        tiles[id] = Tile(id, tile[1:])

        # im = gen_tile_image(tile[1:])
        # im.save(f"tiles/{id}.png")

    # print(edge_map)
    #print(len(edge_map))

    shared_edges = {}

    for id, t in tiles.items():
        for jd, u in tiles.items():
            if id == jd:
                continue
            if any(e in u.edges for e in t.edges):
                shared_edges.setdefault(id, []).append(jd)

    # print(shared_edges)
    # for (id, edges) in sorted(shared_edges.items(), key=lambda x: x[0]):
    #     print(f"{id} -> {', '.join(str(e) for e in edges)}")

    assert all(len(e) <= 4 for e in shared_edges.values())

    corners = [id for id, edges in shared_edges.items() if len(edges) == 2]
    #print(corners)

    assert len(corners) == 4

    res = reduce(lambda x, y: x*y, corners)
    print(res)


    # faces = {}
    # for id, t in tiles.items():
    #     for jd, u in tiles.items():
    #         if id == jd:
    #             continue
    #         for e, f in t.edges.items():
    #             if e in u.edges:
    #                 faces[(id, jd)] = (f, u.edges[e])
    #                 break

    # for (ids, fs) in sorted(faces.items(), key=lambda x: x[0]):
    #     print(f"{ids} -> {fs}")

    ref = tiles[1433]
    image = [[ref]]

    for y in range(12):

        if y != 0:
            t, edge = find_match(tiles, image[y-1][0].bottom, image[y-1][0].id)
            rotflip_bottom(t, edge)
            image.append([t])

        for x in range(11):
            t, edge = find_match(tiles, image[y][x].right, image[y][x].id)
            rotflip_right(t, edge)
            image[y].append(t)

    # for row in image:
    #     print(" ".join(str(t.id) for t in row))

    pixels = set()

    for yoff, row in enumerate(image):
        for xoff, tile in enumerate(row):

            for y, line in enumerate(tile.lines[1:-1]):
                for x, pixel in enumerate(line[1:-1]):
                    if pixel == '#':
                        pixels.add((8*xoff + x, 8*yoff + y))


    # for y in range(100):
    #     print("".join("#" if (x,y) in pixels else '.' for x in range(100)))

    # im = gen_image(pixels)
    # im.save("merged.png")

    for monster in transformations(MONSTER):
        monsters = find_monsters(pixels, monster)
        if monsters:
            break

    print(len(pixels) - monsters * len(coords(MONSTER)))



if __name__ == '__main__':
    main()