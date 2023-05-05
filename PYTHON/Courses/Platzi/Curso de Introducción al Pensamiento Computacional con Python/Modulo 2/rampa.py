x= 10
y = 1
while x > 0:
    z = '0' * x
    print(z + ' ' * y + z)
    x -= 1
    y += 2
y -= 2
x = 1
while x < 10:
    z = '0' * x
    print(z + ' ' * y + z)
    x += 1
    y -= 2
