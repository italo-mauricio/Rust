fn main(){
    let mut numbers = (1, 2, 3);
    numbers.0 = 50; // consigo adicionar um novo valor a tupla

    println!("{:?}", numbers.0);   // adicionando o .0 eu pego o primeiro item da tupla
    array();

}

fn array(){

    let mut numbers = [1.1, 2.0, 3.3]; 
    numbers[0] = 100.5;
    // slice :  

    println!("{:?}", &numbers[1..]); // usando esse método, consigo chamar um local especifico em um array

    //usando slice, posso fatiar o array
}