fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_caracteres = [false; 128];

    for &c in input.as_bytes() {
        let indice = c as usize;
        println!("Caractere: {} indice: {}", c as char, indice); 
        
        if conjunto_caracteres[indice] {
            println!("Caractere duplicado encontrado");
            return false;
        }
        
        conjunto_caracteres[indice] = true;
    }
    
    true 
}

fn verifica_caracteres() {
    let teste = "cateto";
    let resultado = tem_caracteres_unicos(teste);
    println!("Todos os caracteres são únicos: {}", resultado);
}
