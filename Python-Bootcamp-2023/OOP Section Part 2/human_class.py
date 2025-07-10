class Human:
    def __init__(self, first, last, age):
        self.first = first
        self.last = last
        
        @property
        def age(self):
            return self._age
        
        @age.setter
        def age(self,value):
            if value >= 0:
                self._age = value
            else:
                raise ValueError("Age can't be negative")