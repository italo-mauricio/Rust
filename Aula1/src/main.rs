
const SECONDS_IN_MINUTE: u32 = 60;

// constantes são imutáveis e não podem ser definidas duas vezes



fn main(){    // escopo
    let total: i32 = 30;
    { // escopo interno   inicio
        // podemos definir constantes dentro ou fora do escopo a qualquer momento.
        const MINUTES_IN_HOUR: u32 = 60;
        const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE + MINUTES_IN_HOUR;
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