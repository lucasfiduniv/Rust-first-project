

fn Fatorial_While(){
    println!("digite um numero");
    let mut entrada_fatorial = String::new();
    io::stdin().read_line(&mut entrada_fatorial).expect("erro ao ler entrada");
    let mut fatorial = 1;
    let mut entrada_int = converter_valores(&entrada_fatorial);

    while entrada_int > 1 {
        fatorial = fatorial * entrada_int;
        entrada_int = entrada_int - 1;
    }
    println!("o fatorial eh {}", fatorial);
}