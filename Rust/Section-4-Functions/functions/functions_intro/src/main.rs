fn main() {
    open_store("Downtown");
    open_store("Uptown");
    bake_pizza(5, "pepperoni");
    swim_in_profit();
    let result = square(5);
    println!("The square of 5 is {result}");
    let result_implicit = square_implicit(6);
    println!("The square of 6 is {result_implicit}");

    let result: () = mystery();

    let multiplier = 3;

    let calculation: i32 = {
        let value = 5 + 4;
        value * multiplier
    };

    println!("{calculation}");
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(quantity: i32, topping: &str) {
    println!("Baking {quantity} {topping} pizzas");
}

fn swim_in_profit() {
    println!("So much $$$, so little time");
}

// Explicit Return Values
fn square(number: i32) -> i32 {
    return number * number;
}

// Implicit Return Values
fn square_implicit(number: i32) -> i32 {
    number * number // No need for 'return' keyword
}

// Unit Type
fn mystery() {
    println!("Hello there");
}
