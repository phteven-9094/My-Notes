# Create a tip calculator that determines a tip based on bill and how many people are paying
# If the bill was $150.00, split between 5 people, with 12% tip.

# Each person should pay (150.00 / 5) * 1.12 = 33.6
# Format the result to 2 decimal places = 33.60

bill = float(input("Please enter the bill total: $"))
amount_of_people = int(input("How big is your party? "))
tip_percent = int(input("Enter the tip percent you would like to leave: "))

bill_with_tip = tip_percent / 100 * bill + bill
per_person = round(bill_with_tip / amount_of_people, 2)

print(f"Each person should pay: {per_person}")
