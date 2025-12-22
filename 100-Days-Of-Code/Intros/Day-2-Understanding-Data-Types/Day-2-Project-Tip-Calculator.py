# Welcome to the tip calculator!
# Totel Bill
# Tip Percentages
# Number of people to split the bill
# Each person should pay

print("Welcome to the tip calculator!")
total_bill = float(input("What was the total bill? $"))
tip_percentage = int(
    input("What percentage tip would you like to give? 10, 12, or 15? ")
)
number_of_people = int(input("How many people to split the bill? "))

bill_after_tip = total_bill * (1 + tip_percentage / 100)
bill_per_person = bill_after_tip / number_of_people
print(f"Each person should pay: ${bill_per_person:.2f}")
