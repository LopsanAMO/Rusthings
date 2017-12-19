fn get_number() -> i32{
    use std::io::{stdin};
    let mut line = String::new();
    stdin().read_line(&mut line).ok().expect("omg this is not a String");
    line.trim().parse::<i32>().unwrap()
}

fn main() {
    let number: i32 = get_number();
    for x in 0..number{
        for _ in 1..number-x{
            print!(" ");
        }
        for _ in (0..x+1).rev(){
            print!("#");
        }
        println!(" ");
    }
}
