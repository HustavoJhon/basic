import random

def play(r, p, s):
    print(f"Let's play {r}, {p}, {s}")
    choices = (r, p, s) 

    user_choice = input(f"Enter a choice {r}, {p}, {s}: ").capitalize()
    if user_choice not in choices:
        print("Invalid choice")
        return 

    computer_choice = random.choice(choices)
    print(f"Computer: {computer_choice}, You: {user_choice}")

    if user_choice == computer_choice:
        print("It's a Tie!!!")

    elif(
        (user_choice == r and computer_choice == p) or
        (user_choice == p and computer_choice == s) or
        (user_choice == s and computer_choice == r) 
    ):
        print("Computer Win")

    else:
        print("You Win")

# Rock, Paper, Scissors
# r = "Rock"
# p = "Paper"
# s = "Scissors"
# play(r, p, s)

#Water, Snake, Gun

w = "Water"
s = "Snake"
g = "Gun"
play(w, s, g)
