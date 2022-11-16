use std::io;



fn main(){

    println!("Jogo da adivinhação!");
    let mut adivinha = String::new();
    
    io::stdin().read_line(&mut adivinha)
        .expect("Falha!");

    println!("Você disse: {}", adivinha);












}