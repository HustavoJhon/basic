numbers = [1,2,3,4,5]
new_numbers = list(filter(lambda x: x % 2 == 0, numbers))
print(new_numbers)


nombres = ['hola','como','estas']
new_letter = list(filter(lambda y: y == 'hola',nombres))
print(new_letter)


example = [4,6,5,7,3]
filter_example = list(filter(lambda g: g * 2, example))
print(filter_example, example)