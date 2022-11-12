fn main(){    // escopo
    let mut total: i32 = 30;
    println!("Trabalhou {} horas", total);
    total = 44;
    println!("Trabalhou {} horas", total); 

} // fim
// drop
//Rust é uma linguagem tipada
// toda variável é imutavel por padrão
// mut = mutável, transforma minha variável em mutável