### Dates ###

from datetime import datetime 

now = datetime.now()

print(f'day > {now.day})
print(now.hour)
print(now.minute)
print(now.year)
print(now.second)
print(now.month)

timestamp = now.timestamp()

print(timestamp)
