use std::io;

include!("./aulas_iniciais/aulas_iniciais_menu.rs");
include!("./funcoes/funcoes_menu.rs");
include!("./estrutura_dados/estrutura_dados_menu.rs");

fn main() {
    println!("Escolha uma opção:");
    println!("1. Funcoes Basicas");
    println!("2. Funcoes");
    println!("3. Estrutura de Dados");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let choice: u32 = input.trim().parse().expect("Entrada inválida");

    match choice {
        1 => aulas_iniciais_menu(),
        2 => funcoes_menu(),
        3 => estrutura_dados_menu(),
        _ => println!("Opção inválida"),
    }

    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}

