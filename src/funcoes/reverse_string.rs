fn reversed_string(input: &str) -> String {
let reversed: String = input.chars().rev().collect(); 
reversed
}

fn reverve(){
    let mut string = String::new();
    println!("Digite uma string");
    io::stdin().read_line(&mut string).expect("erro ao ler string");
    let converted = reversed_string(&string);
    println!("{}", converted);
}