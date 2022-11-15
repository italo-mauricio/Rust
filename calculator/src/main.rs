use clap::{App, Arg};


fn main(){

    let app = App::new("calculator")
        .arg(
            Arg::with_name("sum")
                .long("--sum")
                .takes_value(true)
        )
        .get_matches();

    if let Some(numbers) = app.value_of("sum") {
        let result: u32 = numbers
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .sum();

        println!("Result: {}", result.to_string());
    }

}