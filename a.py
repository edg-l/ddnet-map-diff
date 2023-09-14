# MIT License
# Copyright Ravie 2023

import twmap
import sys

map_old = twmap.Map(sys.argv[1])
map_new = twmap.Map(sys.argv[2])

tiles_old = map_old.game_layer().tiles
tiles_new = map_new.game_layer().tiles

shape_old = tiles_old.shape
shape_new = tiles_new.shape

w = min(shape_old[1], shape_new[1])
h = min(shape_old[0], shape_new[0])

diff_group = map_new.groups.py_new()
diff_group.name = 'Difference'

layer_add = diff_group.layers.new_tiles(w, h)
layer_add.color = (0, 255, 0, 64)
layer_add.name = 'Added'
tiles_add = layer_add.tiles

layer_del = diff_group.layers.new_tiles(w, h)
layer_del.color = (255, 0, 0, 64)
layer_del.name = 'Deleted'
tiles_del = layer_del.tiles

layer_mod = diff_group.layers.new_tiles(w, h)
layer_mod.color = (255, 255, 0, 64)
layer_mod.name = 'Modified'
tiles_mod = layer_mod.tiles

for x in range(w):
    for y in range(h):
        index_old = tiles_old[y, x, 0]
        index_new = tiles_new[y, x, 0]

        if index_old == 0 and index_new != 0:
            tiles_add[y, x, 0] = True
        elif index_old != 0 and index_new == 0:
            tiles_del[y, x, 0] = True
        elif index_old != index_new:
            tiles_mod[y, x, 0] = True

layer_add.tiles = tiles_add
layer_del.tiles = tiles_del
layer_mod.tiles = tiles_mod

map_new.save('difference.map')
