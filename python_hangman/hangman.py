from words import words
import random

class Letter:
    def __init__(self, letter, found):
        self.letter = letter
        self.found = found

def choose_word(words):
    return random.choice(words).lower()

def handle_guess():
    guess = input("Enter your guess: ")
    guess = guess.lower()[0]
    print(guess)

def print_game(game_state):
    output = ""
    for letter in game_state:
        if(letter.found):
            output += letter.letter
            output += " "
        else:
            output += "_ "
    print(output)

def play():
    all_found = False
    lives = 5
    word = choose_word(words)
    game_state = []
    for letter in word:
        game_state.append(Letter(letter, False))
    print("Welcome to Hangman!")
    print_game(game_state)
    guesses = []
    while (lives > 0):
        if(lives == 1):
            print(f"You have {lives} life!")
        else:
            print(f"You have {lives} lives!")
        print_game(game_state)
        guess = input(("Please guess a letter!\n"))
        while(not guess.isalpha() or guess in guesses):
            if(guess in guesses):
                guess = input("You already guessed this letter, please enter a new letter!\n")
            if(not guess.isalpha()):
                guess = input("Please enter a valid letter!\n")
        guesses.append(guess)
        if(not guess in word):
            lives -= 1
            all_found = False
        else:
            all_found = True
            for letter in game_state:
                if letter.letter == guess:
                    letter.found = True
                else:
                    if(not letter.found):
                        all_found = False
        if(all_found):
            print_game(game_state)
            print("Congratulations, you won!\n")
            return
    print("\nGame over, better luck next time!\n")



if __name__ == '__main__':
    play()