/*
1) Se o número for divisível por 3, 5, 7, o goku vai dar o kamehameha completo, caso contrário,
só irá retornar para o usuário partes da STRING.|

*/




pub fn goku(n: u32) -> String {
    let mut result = String::new();


    if n % 3 == 0{
        result.push_str("Ka");
    }

    if n % 5 == 0{
        result.push_str("MeHaMe");
    }

    if n % 7 == 0{
        result.push_str("HA!!!");
    }

    if result.is_empty(){
        return n.to_string();
    }


    result

}


fn main(){
    // chamada da função de resultado
}
