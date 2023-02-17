def find_volume(length=1, width=1, depth=1):
    return length * width * depth, width, 'Hola'

result, width, text = find_volume(width=10)
print(result)
print(width)
print(text)

def fun(txt=0):
    return "lol", txt, "h"
function = fun()
print(function[0])