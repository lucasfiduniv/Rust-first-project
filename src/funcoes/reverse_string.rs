fn reversed_string(input: &str) -> String {
let reversed: String = input.chars().rev().collect(); 
reversed
}

fn reverve(){
let input_string = String::from("hello");
let reversed_string1 = reversed_string(&input_string);
println!("Original string {}", input_string);
println!("Reverse string {}", reversed_string1);
}