fn main(){    // escopo
    let total: i32 = 30;
    { // escopo interno   inicio
        let total = total + 20;  //conseguimos acessar o escopo externo, através do escopo interno
        println!("Trabalhou {} horas", total);
    } // fim 
    println!("Trabalhou {} horas", total);
; 

} // fim
// drop
//Rust é uma linguagem tipada
// toda variável é imutavel por padrão
// mut = mutável, transforma minha variável em mutável
//let = inicializa uma variável