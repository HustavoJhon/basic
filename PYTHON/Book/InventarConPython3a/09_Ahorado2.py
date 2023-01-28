import random

HANGMAN_PICS = ['''
   +---+
       |
       |
       |
      ===''', '''
   +---+
   O   |
       |
       |
      ===''', '''
   +---+
   O   |
   |   |
       |
      ===''', '''
   +---+
   O   |
  /|   |
       |
      ===''', '''
   +---+
   O   |
  /|\  |
       |
      ===''', '''
   +---+
   O   |
  /|\  |
  /    |
      ===''', '''
   +---+
   O   |
  /|\  |
  / \  |
      ===''', '''
   +---+
  [O   |   
  /|\  |
  / \  |
      ===''', '''

   +---+
  [O]  |
  /|\  |
  / \  |
      ===''']

words = {'Colores': 'rojo naranja amrillo'.split(),
         'Formas': 'cuadrado triangulo rectangul'.split(),
         'Frutas': 'manzana naranja pera'.split(),
         'Animales': 'gato perro loro'.split()}

# selecciona una palabra 

def getRandomWord(wordDict):
    wordKey = random.choice(list(wordDict.keys()))
    #Segundo, elige una palabra aleatoria de la lista correspondiete a la clave en el diccionario
    wordIndex = random.randint(0, len(wordDict[wordKey]) -1)
    return [wordDict[wordKey][wordIndex], wordKey]


def displayBoard(missedLetters, correctLetters, secretWord):
    print(HANGMAN_PICS[len(missedLetters)])
    print()

    print('Missed letters:', end=' ')
    for letter in missedLetters:
        print(letter, end=' ')
    print()

    blanks = '_' * len(secretWord)
    
    for i in range(len(secretWord)):
        if secretWord[i] in correctLetters:
            blanks = blanks[:i] + secretWord[i] + blanks[i+1:]
    
    for letter in blanks:
        print(letter, end=' ')
    print()

def getGuess(alreadyGuessed):
    while True:
        print('Guess a letter.')
        guess = input()
        guess = guess.lower()
        if len(guess) != 1:
            print('Please enter a single letter')
        elif guess in alreadyGuessed:
            print('You have already guessed that letter. Choose agai.')
        elif guess not in 'abcdefghijklmnopqrstuvwxyz':
            print('Please enter a LETTER')
        else:
            return guess

def playAgain():
    print('Do you wnat to play again? (yes or no)')
    return input().lower().startswith('y')

print('HANGMAN')
missedLetters = ''
correctLetters = ''
secretWord, secretSet = getRandomWord(words)
gameIsDone = False
print(secretWord)

while True:
    print('The secret word is in the set: ' + secretSet)
    displayBoard(missedLetters, correctLetters, secretWord)
    guess = getGuess(missedLetters + correctLetters)
    
    if guess in secretWord:
        correctLetters = correctLetters + guess
        
        foundAllLetters = True 
        for i in range(len(secretWord)):
            if secretWord[i] not in correctLetters:
                foundAllLetters = False
                break
        if foundAllLetters:
            print(f'Yes! The secret word is {secretWord} You have won!')
            gameIsDone = True
    else:
        missedLetters = missedLetters + guess
        if len(missedLetters) == len(HANGMAN_PICS) - 1:
            displayBoard(missedLetters, correctLetters, secretWord)
            print(f'''You have run out of guesses!\nAfter 
                  {missedLetters} missed guesses and 
                  {correctLetters} correct guesses, the word was {secretWord}''')
            gameIsDone = True 
            
    if gameIsDone:
        if playAgain():
            missedLetters = ''
            correctLetters = ''
            gameIsDone = False 
            secretWord, secretSet = getRandomWord(words)
        else:
            break