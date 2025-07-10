f = open("story.txt")
f.read()  # The cursor will be at the end of the file after this read.
# Even though we updated the file, the cursor is still at the end of the previous version because "open" creates a tunnel like connection to the file.
f.seek(
    0
)  # This will move the cursor back to the beginning. #You can pass in any number to indicate what character you want the read to start from

f.readline(
    1
)  # Specify a specific line to read from a file. (This does move the cursor, so it will need to be moved again)
f.readlines()  # Returns a list of lines

f.close()  # Ends the connection mentioned above


with open("story.txt") as file:
    data = file.read()

with open("haiku.txt", "w") as file:
    file.write("Writing files is great\n")
    file.write("Here's another line of text \n")
    file.write("Closing now, goodbye! \n")

with open("File IO/haiku.txt", "a") as file:
    file.write("Here's one more haiku")
    file.write("What about the older one?")
    file.write("Let's go check it out")
