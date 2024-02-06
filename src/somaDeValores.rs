fn soma_valores(){
    let mut soma = 0;
    let mut valor_entrada = String::new();
    io::stdin().read_line(&mut valor_entrada).expect("erro ao ler a entrada");
    let mut valor_i32 = converter_valores(&valor_entrada);

    while valor_i32 !=0{
        let mut r = valor_i32 %10;
        soma= soma+r;
        valor_i32 = valor_i32 / 10;
    }
    println!("o valor da soma dos digitos {}", soma);
}