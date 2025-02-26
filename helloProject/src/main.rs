// Functions
// Any function / Variables should be written in the following snake or kebab cases.
// snake case: hello_world
// kebab case: hello-world

fn main() {
    hello_world();
    tell_height(182);
    human_id("Godwin", 12, 200.3);

    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result is: {}", _x);
    // add(3,5);
    let y: i32 = add(4, 6);
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}", add(4, 6));

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi);
}

// Hoisting - can call function anywhere in your code
fn hello_world() {
    println!("Hello, Rust!");
}

// you can insert input values
fn tell_height(height: u32) {
    println!("My height is {}cm.", height);
}

// you can insert more than one value
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, i am {} years old, and my height is {}cm.",
        name, age, height
    );
}

// functions returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value
// Statement: Anything that doesnt return a value

// Expressions:
// 5,
// true,
// false,
// add(3,4),
// if condition{value1} else {value2}
// ({codeblock})

// Statements
// Almost all statements in rust end with ;
// let y = let x = 10;
// 1 Variable declarations: let x = 5;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condition, while condition, etc

// Final example
// BMI = height(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
