
fn converter_valores(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn conversao(){

    let mut number1 = String::new();
    println!("Digite um valor");
    io::stdin().read_line(&mut number1).expect("erro ao ler number1");

    println!("Digite mais um valor");
    let mut number2 = String::new();
    io::stdin().read_line(&mut number2).expect("erro ao ler number2");

    if converter_valores(&number1) > converter_valores(&number2){
        println!("o numero eh {} maior que {}", number1,number2);
    }
    else{
        println!("o numero eh {} maior ou igual que {}", number2,number1);
    }
}