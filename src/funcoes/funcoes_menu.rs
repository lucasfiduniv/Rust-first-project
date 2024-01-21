
include!("dobro.rs");
include!("introducao_for.rs");
include!("verifica_caracteres_unicos.rs");
include!("reverse_string.rs");
include!("anagran.rs");
fn funcoes_menu(){
    println!("Escolha uma opção:");
    println!("1. Dobro");
    println!("2. FOR");
    println!("3. Verifica Caracteres");
    println!("4. Reverte uma String");
    println!("5. Anagrama");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    let choice: u32 = input.trim().parse().expect("Entrada inválida");

    match choice {
        1 => teste_dobro(),
        2 => introducao_for(),
        3 => verifica_caracteres(),
        4 => reverve(),
        5 => anagran(),
        _ => println!("Opção inválida"),
    }

    println!("Pressione Enter para sair...");
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
}