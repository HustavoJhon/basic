import random 
import string

letters_upper = string.ascii_uppercase
letters_lower = string.ascii_lowercase
simbol = string.punctuation
number = string.digits

print("="*25)
print("THE PASSWORD GENERATOR")
print("="*25)

def run(password):
    txt = letters_upper + letters_lower + simbol + number
    psw = []
    for i in range(0,password):
        chars_random = random.choice(txt)
        psw.append(chars_random)
    psw = ''.join(psw)
    print(f"You password is <|  {psw}  |>")

if __name__ == "__main__":
    log = int(input("Log password: "))
    run(log)