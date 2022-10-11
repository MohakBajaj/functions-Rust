fn main() {
    println!("Hello, world! to Functions in Rust");

    anathor_function();
    anathor_function_with_parameter(5);
    anathor_function_with_multiple_parameters(5, 6);
    println!("{}", anathor_function_with_return_value(5));
    println!("{}", anathor_function_with_return_value_and_expression(5));
}

fn anathor_function() {
    println!("Another function.");
}

fn anathor_function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn anathor_function_with_multiple_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn anathor_function_with_return_value(x: i32) -> i32 {
    x + 1
}

// fn anathor_function_with_return_value(x: i32) -> i32 {
//     x + 1;
// }
// This will generate an error: expected `i32`, found `()`

fn anathor_function_with_return_value_and_expression(x: i32) -> i32 {
    let y = {
        let x = x + 1;
        x + 1
    };
    y
}
