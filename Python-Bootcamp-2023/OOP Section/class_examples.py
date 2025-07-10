

class User:
    
    active_users = 0
    
    @classmethod
    def display_active_users(cls):
        return User.active_users
    
    @classmethod
    def from_string(cls, data_str):
        first, last, age = data_str.split(",")
        return cls(first, last, age)
        
    
    def __init__(self, first, last, age):
        #If you instatiate a class, you would have to make sure to pass in the above, similar to a function
        self.first = first
        self.last = last
        self.age = age
        User.active_users += 1
        
    def __repr__(self):
        return f"{self.first} is {self.age}"
    
    def logout(self):
        User.active_users -= 1
        return f"{self.first} has logged out"
    
    def full_name(self):
        return f"{self.first} {self.last}"
    
    def initials(self):
        return f"{self.first[0]}.{self.last[0]}."

    def likes (self,thing):
        return f"{self.first} likes {thing}"

user1 = User("Joe", "Smith", 68)
user2 = User("Blanca", "Lopez", 41)
print(user2.full_name())
user1.likes("Ice Cream")
user2.likes("Chips")

