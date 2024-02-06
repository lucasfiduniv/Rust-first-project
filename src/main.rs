use std::io;

fn main() {
    println!("Bem-vindo ao Jogo da Forca!");

    let palavra_secret: &str = "RUST"; // Palavra a ser adivinhada
    let mut palavra_descoberta: Vec<char> = vec!['_'; palavra_secret.len()]; // Inicializa com underscores

    let mut tentativas = 6; // Número máximo de tentativas

    loop {
        println!("\nPalavra: {:?}", palavra_descoberta);
        println!("Tentativas restantes: {}", tentativas);

        // Lê a tentativa do jogador
        let mut tentativa = String::new();
        io::stdin().read_line(&mut tentativa).expect("Falha ao ler a linha");

        // Converte para maiúsculas para evitar problemas de case
        let tentativa = tentativa.trim().to_uppercase();

        // Verifica se a tentativa está correta
        if tentativa.len() == 1 {
            let letra = tentativa.chars().next().unwrap();
            if palavra_secret.contains(letra) {
                // Substitui os underscores pela letra correta nas posições corretas
                for (i, c) in palavra_secret.chars().enumerate() {
                    if c == letra {
                        palavra_descoberta[i] = letra;
                    }
                }
            } else {
                println!("Letra incorreta!");
                tentativas -= 1;
            }
        } else {
            println!("Por favor, insira apenas uma letra!");
        }

        // Verifica se o jogador ganhou ou perdeu
        if palavra_descoberta.iter().collect::<String>() == palavra_secret {
            println!("Parabéns, você ganhou! A palavra era: {}", palavra_secret);
            break;
        } else if tentativas == 0 {
            println!("Você perdeu! A palavra era: {}", palavra_secret);
            break;
        }
    }
}
