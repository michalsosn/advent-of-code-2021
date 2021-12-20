def rotate_x(p): 
    return (p[0], p[2], -p[1])
def rotate_y(p): 
    return (p[2], p[1], -p[0])
def rotate_z(p): 
    return (p[1], -p[0], p[2])

result = set()
init_x = (1, 2, 3)
for x in range(4):
    init_y = init_x
    for y in range(4):
        init_z = init_y
        for z in range(4):
            result.add(init_z)
            init_z = rotate_z(init_z)
        init_y = rotate_y(init_y)
    init_x = rotate_x(init_x)

for p in sorted(list(result)):
    print(p[0], p[1], p[2])
