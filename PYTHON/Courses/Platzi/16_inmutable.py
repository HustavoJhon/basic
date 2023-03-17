items = [
    {
        'product': 'camisa',
        'price' : 100,
        'taxes' : 100 * .19
    },
    {
        'product': 'pantalon',
        'price': 300
    },
    {
        'product': 'chompa',
        'price' : 200
    }
]

def add_taxes(item):
    new_item = item.copy()
    item['taxes'] = new_item['price'] * .19
    return item

new_items = list(map(add_taxes, items))

print("NEW LIST")
print(new_items)
print("OLD LIST")
print(items)