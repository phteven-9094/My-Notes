# Instructions

1. Create a function called encrypt that takes the text and shift as inputs
2. Inside the encrypt function, shift each letter of the text forwards in the alphabet by the shift amount and print the encrypted text
3. Call the encrypt function and pass in the user inputs.
4. Create a function called decrypt that takes the text and shift as inputs.
5. Inside the decrypt function, shift each letter of the text backwards in teh alphabet by the shift amount and print the decrypted text
6. Check if the user wanted to encrypt or decrypt the message by checking the direction variable. Then call the correct function based on that direction variable.
7. Combine the encrypt and decrypt functions into a single function called ceasar
8. Call the ceasar function, passing over the text, shift, and direction values
9. Import and print the logo from art.py when the program starts
10. What if the user enters a shift that is greater than the number of letters in the alphabet? Try running the program and entering a shift number of 45. Add some code so that the program continues to work even if the user enters a shift number greater than 26. Hint: Think about how you can use the modulus.
11. Can you figure out a way to ask the user if they want to restart the cipher program? Hint: Try creating a while loop that continues to execute the program if the user types yes.
