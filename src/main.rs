use std::io;

// Inclua os módulos
include!("compararValores.rs");
include!("converterDeInt.rs");

fn main() {
    println!("Escolha uma opção:");
    println!("1. Comparar valores");
    println!("2. Conversão de inteiros");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let choice: u32 = input.trim().parse().expect("Entrada inválida");

    match choice {
        1 => comparar_valores(),
        2 => conversao(),
        _ => println!("Opção inválida"),
    }

    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}
