def be_polite(fn):
    def wrapper():
        print("what a pleasure to meet you!")
        fn()
        print("Have a great day!")
    return wrapper

@be_polite
def greet():
    print("my name is Colt.")

#wrapped_greet = be_polite(greet) #Calling the wrapped function with a variable

#print(greet())

# def shout(fn):
#     def wrapper(name):
#         return fn(name).upper()
#     return wrapper

def shout(fn): #Example of using decorators with multiple arguments
    def wrapper(*args, **kwargs):
        return fn(*args, **kwargs).upper()
    return wrapper

@shout
def greet(name):
    return f"Hi, I'm {name}"

@shout
def order(main, side):
    return f"Hi, I'd like the {main}, with a side of {side}, please"
        
