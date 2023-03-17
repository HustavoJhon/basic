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

prices = list(map(lambda item: item['price'], items))
print(prices)

print(items)

def add_taxes(item):
    item['taxes'] = item['price'] * .19
    return item

new_items = list(map(add_taxes, items))
print(new_items)

print(items)