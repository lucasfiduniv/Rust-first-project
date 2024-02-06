fn dobro (num:i32)-> i32{
    let soma:i32 = num * 2;
    return soma;
}

fn teste_dobro(){
    let mut number = String::new();
    println!("Digite um valor");
    io::stdin().read_line(&mut number).expect("erro ao ler number1");

    let  numero_convertido = converter_valores(&number); 
    println!("o dobro de {} eh {}", numero_convertido, dobro(numero_convertido));

}