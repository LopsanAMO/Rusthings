use std::io::{stdin};

fn get_number() -> i32{
    let mut line = String::new();
    stdin().read_line(&mut line).ok().expect("omg this is not string");
    line.trim().parse::<i32>().unwrap()
}

fn get_numbers() -> Vec<i32>{
    let mut line = String::new();
    stdin().read_line(&mut line).ok().expect("omg this is not string");
    line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
}


fn get_fractions(v: Vec<i32>) -> Vec<f32>{
    let mut aux_v: Vec<f32> = vec![0.0; 3];
    for element in v{
        if element.is_positive(){
            aux_v[0] += 1.0
        }
        else if !(element.is_positive()) && element != 0{
            aux_v[1] += 1.0
        }
        else if element == 0{
            aux_v[2] += 1.0
        }
    }
    aux_v
}


fn main() {
    let size = get_number() as f32;
    let v_fractions: Vec<f32> = get_fractions(get_numbers());
    for element in v_fractions{
        println!("{:.6}", element/size);
    }

}
