
include!("compararValores.rs");
include!("fatorial_while.rs");
include!("converterDeInt.rs");
include!("somaDeValores.rs");
include!("media_nota_alunos.rs");

fn aulas_iniciais_menu(){
    println!("Escolha uma opção:");
    println!("1. Comparar valores");
    println!("2. Conversão de inteiros");
    println!("3. Soma dos digitos do numero");
    println!("4. Fatorial de um numero");
    println!("5. media de nota de um aluno");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let choice: u32 = input.trim().parse().expect("Entrada inválida");

    match choice {
        1 => comparar_valores(),
        2 => conversao(),
        3 => soma_valores(),
        4 => Fatorial_While(),
        5 => media_nota_aluno(),
        _ => println!("Opção inválida"),
    }

    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}