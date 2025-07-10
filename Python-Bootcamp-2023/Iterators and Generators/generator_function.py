def count_up_to(max):
    count = 1
    while count <= max:
        yield count #returns the value of count and stops until next() is used
        count += 1
        
counter = count_up_to(5)
next(counter)
