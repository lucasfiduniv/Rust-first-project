fn is_anagran(a1: &str, a2: &str) -> bool {
    if a1.len() != a2.len() {
        return false;
    }
let mut a1_chars: Vec<char> = a1.chars().collect();
let mut a2_chars: Vec<char> = a2.chars().collect();

a1_chars.sort();
a2_chars.sort();

return a1_chars == a2_chars;
}

fn anagran(){
    let mut string1 = String::new();
    let mut string2 = String::new();
    println!("Digite uma string");
    io::stdin().read_line(&mut string1).expect("erro ao ler string1");
    println!("Digite uma nova string");
    io::stdin().read_line(&mut string2).expect("erro ao ler string2");

    let isAnagran = is_anagran(&string1, &string2);
    if isAnagran == true { println!("Anagran")}
    else if isAnagran == false { println!("Not anagran")} 
}