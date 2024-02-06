
include!("dobro.rs");
include!("introducao_for.rs");

fn funcoes_menu(){
    println!("Escolha uma opção:");
    println!("1. Dobro");
    println!("2. FOR");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let choice: u32 = input.trim().parse().expect("Entrada inválida");

    match choice {
        1 => teste_dobro(),
        2 => introducao_for(),
        _ => println!("Opção inválida"),
    }

    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}