import string
my_dict = {
    " ":" ",
    "A":"4",
    "B":"I3",
    "C":"[",
    "D":")",
    "E":"3",
    "F":"|=",
    "G":"&",
    "H":"#",
    "I":"1",
    "J":",_|",
    "K":">|",
    "L":"1",
    "M":"JVI",
    "N":"^/",
    "O":"0",
    "P":"|*",
    "Q":"(_,)",
    "R":"I2",
    "S":"5",
    "T":"7",
    "U":"(_)",
    "V":"|/",
    "W":"\/\/",
    "X":"><",
    "Y":"j",
    "Z":"2",
    "1":"L",
    "2":"R",
    "3":"E",
    "4":"A",
    "5":"S",
    "6":"b",
    "7":"T",
    "8":"B",
    "9":"g",
    "0":"o",
}

def run(letter):
    blank = ""
    print(f"Tu letra es: {letter}".upper())
    for i in range(len(letter)):
        txt = letter.upper()[i]
        if txt in my_dict:
            print(my_dict[txt],end='')
        else:
           print("'",end="") 

name = input("Name: ")
if __name__ == "__main__":
    run(name)
