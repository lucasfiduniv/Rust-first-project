
fn calculo_fatorial(){
let mut entrada_fatorial = String::new();
io::stdin().read_line(&entrada_fatorial).expect("falha ao gravar valor");
let mut fatorial = 1;
let mut valor_convertido = converter_valores(&entrada_fatorial);
while valor_convertido > 1 {
    fatorial = fatorial * converter_valores(&entrada_fatorial);
    entrada_fatorial = valor_convertido - 1;
}
println!("o fatorial eh{}",fatorial);
}