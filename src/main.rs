use std::io;

include!("compararValores.rs");
include!("converterDeInt.rs");
include!("somaDeValores.rs");
include!("fatorial_while.rs");
fn main() {
    println!("Escolha uma opção:");
    println!("1. Comparar valores");
    println!("2. Conversão de inteiros");
    println!("3. Soma dos digitos do numero");
    println!("4. Fatorial de um numero");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let choice: u32 = input.trim().parse().expect("Entrada inválida");

    match choice {
        1 => comparar_valores(),
        2 => conversao(),
        3 => soma_valores(),
        4 => Fatorial_While(),
        _ => println!("Opção inválida"),
    }

    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}
