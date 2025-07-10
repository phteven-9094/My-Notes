#Defining Classes and Objects

#Classes
    #Like blueprints for objects. Classes can contain methods (functions) and attributes (Similar to keys in a dict)
#Instace
    # Objects that are constructed from a class blueprint that contains their class's methods and properties
#Encapsulation
    #the grouping of public and private attributes and methods into a programmatic class, making abstraction possible
#Abstraction
    #exposing only "relevant" data in a class interface, hiding private attributes and methods from users

#Creating Classes and Instances
    #see class_examples.py

#The __init__method
    #Anytime you make a new instance of a class, Python will automatically look for an __init__ method
    #which gets called every time you create an instance of the class (instantiate)
    #You don't need to use an __init__ method if you are creating a class with only methods
    
#Underscores: Dunder Methods, Name Mangling and More
    # __init__ example of dunder methods. Special things that Python looks for. Don't go defining those.
    # single underscore is a way to tell developers this is supposed to be a private attribute or method (ex. _secret = "var")
    # double underscore lets python change the name of the attribute (ex. __msg). Lets the attribute be specific to a class
    

#Adding Instance Methods
    #See class_examples.py
    
#Class Attributes
    #Defined once that is defined directly on the class
    
#Class Methods
    #Not Commonly Used
    # Methods that are not concerned with specific instances, but with the class itself
    #Uses the @classmethod decorator

#The __repr__method
    #Provides a nicer string representation
    #See class_examples