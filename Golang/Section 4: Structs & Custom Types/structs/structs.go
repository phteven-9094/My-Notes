package main

import (
	"fmt"

	"example.com/structs/user"
)

// type User struct {
// 	firstName string
// 	lastName  string
// 	birthDate string
// 	createdAt time.Time
// }

// func (u *User) outputUserDetails() {
// 	// ...
// 	fmt.Println(u.firstName, u.lastName, u.birthDate)
// }

// func (u *User) clearUserName() {
// 	u.firstName = ""
// 	u.lastName = ""

// }

// func newUser(firstName, lastName, birthDate string) (*User, error) {
// 	if firstName == "" || lastName == "" || birthDate == "" {
// 		return nil, errors.New("first Name, last name, and birthdate are required")
// 	}
// 	return &User{
// 		firstName: firstName,
// 		lastName:  lastName,
// 		birthDate: birthDate,
// 		createdAt: time.Now(),
// 	}, nil
// }

func main() {
	userFirstName := getUserData("Please enter your first name: ")
	userLastName := getUserData("Please enter your last name: ")
	userBirthDate := getUserData("Please enter your birthdate (MM/DD/YYYY): ")

	//var appUser User
	// appUser := User{
	// 	firstName: userFirstName,
	// 	lastName:  userLastName,
	// 	birthDate: userBirthDate,
	// 	createdAt: time.Now(),
	// }

	// appUser, err := user.User.newUser(userFirstName, userLastName, userBirthDate)
	// appUser := user.User{
	// 	FirstName: "Max",
	// }

	appUser, err := user.NewUser(userFirstName, userLastName, userBirthDate)
	if err != nil {
		fmt.Println(err)
		return
	}

	admin := user.NewAdmin("test@example.come", "password")
	admin.User.OutputUserDetails()
	admin.User.ClearUserName()
	admin.User.OutputUserDetails()

	// ... do something awesome with that gathered data!

	appUser.OutputUserDetails()
	appUser.ClearUserName()
	appUser.OutputUserDetails()
}

// func outputUserDetails(u User) {
// 	// ...
// 	fmt.Println(u.firstName, u.lastName, u.birthDate)
// }

func getUserData(promptText string) string {
	fmt.Print(promptText)
	var value string
	fmt.Scan(&value)
	return value
}
