# Make a simple text-based game from the flowchart below
# https://viewer.diagrams.net/index.html?highlight=0000ff&edit=_blank&layers=1&nav=1&title=Treasure%20Island%20Conditional.drawio#Uhttps%3A%2F%2Fdrive.google.com%2Fuc%3Fid%3D1oDe4ehjWZipYRsVfeAx2HyB7LCQ8_Fvi#%7B%22pageId%22%3A%22C5RBs43oDa-KdzZeNtuy%22%7D


print("Welcome to Treasure Island. \n Your mission is to find the treasure.")
first_option = input("Do you want to go left or right? ").lower()

if first_option == "left":
    second_option = input("Do you want to swim or wait? ").lower()
    if second_option == "wait":
        last_option = input("Which door do you open: Red, Yellow, or Blue? ").lower()
        if last_option == "red":
            print("Burned by fire. Game over")
        elif last_option == "blue":
            print("Eaten by beasts. Game over")
        elif last_option == "yellow":
            print("You found the treasure. You win!")
        else:
            print("Game Over")
    else:
        print("Attacked by trout. Game over")
else:
    print("Fall into a hole. Game Over.")
