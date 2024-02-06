pub fn comparar_valores(){
   let number1:i32 = 10;
   let number2:i32 =30;

   if number1 > number2 {
    println!("{} e maior que {}", number1, number2);
   }else if number1 < number2 {
    println!("{} e maior que {}", number2, number1);
   }else{
    println!("{} e igual {}", number1, number2);
   }
}