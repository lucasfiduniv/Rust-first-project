
enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn enum_teste (){
    let player:Direction = Direction::Right;
    match player{
        Direction::Up => println!("o jogador foi para cima"),
        Direction::Down => println!("o jogador foi para baixo"),
    }
}