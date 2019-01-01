#![warn(clippy::all)]

mod day1;
mod utils;

fn main() {
    let res = day1::run();
    match res {
        Ok(res) => println!("result is: {}", res),
        Err(err) => panic!("error: {:?}", err),
    };
}
