mark = ["love","25","30","40"]
points = ["P1", "P1", "P2", "P2", "P1", "P2", "P1", "P1"]

P1 = 0
P2 = 0

total1 = points.count("P1")
total2 = points.count("P2")

if total1 - total2 == 3 or total2 - total1 == 3 or total2 + total1 < 4:
    print("Error")
    quit()

for i in points:
    if i == "P1":
        P1 += 1
    elif i == "P2":
        P2 += 1 
    if P1 > 3 or P2 > 3:
        if P1 == P2:
            print("Deuce")
        elif P1 - P2 == 2:
            print("Gana P1")  
        elif P2 - P1 == 2:
            print("Gana P2")
        elif P1 > P2:
            print("Ventaja para P1")
        elif P2 > P1:
            print("Ventaja para P2")
    elif P1 == 3 and P2 == 3:
        print("Deuce")
    else:
        print(f"{mark[P1]} - {mark[P2]}")