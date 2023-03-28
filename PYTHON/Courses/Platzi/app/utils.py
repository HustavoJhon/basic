def population_by_country(data, country):
    result = list(filter(lambda item: item['Country'] == country, data))
    return result
