# Week Generator
def week():
    days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ]
    for day in days:
        yield day


# yes_or_no
def yes_or_no():
    answer = "yes"
    while True:
        yield answer
        answer = "no" if answer == "yes" else "yes"


# make_song
def make_song(verses=99, beverage="soda"):
    for num in range(verses, -1, -1):
        if num > 1:
            yield f"{num} bottles of {beverage} on the wall."
        elif num == 1:
            yield f"Only 1 bottle of {beverage} left!"
        else:
            yield f"No more {beverage}!"


# get_multiples
def get_multiples(num=1, count=10):
    next_num = num
    while count > 0:
        yield next_num
        count -= 1
        next_num += num


# get_unlimited_multiples
def get_unlimited_multiples(num=1):
    next_num = num
    while True:
        yield next_num
        next_num += num

#generator expression
def nums():
    for num in range(1,10):
        yield num
        
g = nums()        
g = (num for num in range(1,10))