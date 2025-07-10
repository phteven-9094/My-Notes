class Animal:
    def __init__(self, name, species):
        self.name = name
        self.species = species

    def make_sound(self, sound):
        print(f"this animal says {sound}")

    def __repr__(self):
        return f"{self.name} is a {self.species}"


# Cat class inherits from Animal
class Cat(Animal):
    def __init__(self, name, species, breed, toy):
        super().__init__(name, species)
        self.breed = breed
        self.toy = toy


# Make a new cat instance
# blue = Cat()

# # Because of inheritance, a Cat has access to:
# blue.make_sound("Meow")
# print(blue.cool)

# # blue is both a Cat and Animal (and base object)
# print(isinstance(blue, Cat))
# print(isinstance(blue, Animal))
# print(isinstance(blue, object))
