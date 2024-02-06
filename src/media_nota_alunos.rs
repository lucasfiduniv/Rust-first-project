

fn media_nota_aluno(){
    println!("digite a nota 1 do aluno");
    let mut entrada_nota1 = String::new();
    io::stdin().read_line(&mut entrada_nota1).expect("erro ao ler entrada");

    println!("digite a nota 2 do aluno");
    let mut entrada_nota2 = String::new();
    io::stdin().read_line(&mut entrada_nota2).expect("erro ao ler entrada");
    
    println!("digite a nota 3 do aluno");
    let mut entrada_nota3 = String::new();
    io::stdin().read_line(&mut entrada_nota3).expect("erro ao ler entrada");

    let mut soma = 
    converter_valores(&entrada_nota1) + 
    converter_valores(&entrada_nota2) + 
    converter_valores(&entrada_nota3);

    let mut media:i32 = soma / 3;

    println!("a media do aluno foi {}", media);
}