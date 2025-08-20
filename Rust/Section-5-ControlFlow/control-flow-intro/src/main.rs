fn main() {
    //If Statement
    let some_condition_that_we_cannot_predict_in_advace: bool = true;

    if some_condition_that_we_cannot_predict_in_advace {
        println!("This line will be output");
    }

    // If-Else and Else Statements
    let season = "summer";

    if season == "summer" {
        println!("Schools Out!");
    } else if season == "winter" {
        println!("It's cold outside!");
    } else {
        println!("It's a nice day!");
    }

    // Assigning variables to if statements - see fn even_or_odd
    even_or_odd(5);

    // The Match Statement
    let evaluation = true;
    let evaluation_example = false;

    match evaluation {
        true => println!("The evaluation is true!"), // Can use code blocks here as well
        false => {
            println!("The evaluation is false!")
        }
    }

    let value: i32 = match evaluation_example {
        true => 20,
        false => 40,
    };

    println!("The value is: {value}");

    // Underscores in a Match Arm
    let seasons: &str = "spring";

    match seasons {
        "summer" => {
            println!("Schools Out!")
        }
        "winter" => {
            println!("It's cold outside!")
        }
        "spring" => {
            println!("It's a nice day!")
        }
        "autumn" => {
            println!("The leaves are falling!")
        }
        _ => {
            println!("This is not a season!")
        }
    }
}

fn even_or_odd(number: i32) {
    let result: &str = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}
