struct Matriz {
    vector: Vec<Vec<i32>>,
}


fn get_numbers() -> Vec<i32>{
    use std::io::{stdin};
    let mut line = String::new();
    stdin().read_line(&mut line).ok().expect("omg this is not string");
    line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn get_coordinates(v: Vec<Vec<i32>>,x: i32) -> (Vec<Vec<i32>>, Vec<Vec<(i32, i32)>>){
    let mut vector: Vec<Vec<(i32, i32)>> = Vec::new();
    for repeat in 0..x{
        let mut aux_vector: Vec<(i32, i32)> = Vec::new();
        let mut x_vector: Vec<(i32, i32)> = Vec::new();
        let mut y_vector: Vec<(i32, i32)> = Vec::new();
        for x_el in repeat as usize..(v.len() as i32 - repeat as i32) as usize{
            for y_el in repeat as usize..(v[x_el].len() as i32 - repeat as i32) as usize{
                let x_len: i32 = (v.len() as i32 - 1) - repeat;
                let y_len: i32 = (v[x_el].len() as i32 - 1) - repeat;
                if x_el == x_len as usize || y_el == repeat as usize{
                    x_vector.push((x_el as i32, y_el as i32));
                }
                else if x_el == repeat as usize || y_el == y_len as usize{
                    y_vector.push((x_el as i32, y_el as i32));
                }
            }
        }
        aux_vector.extend(x_vector);
        let mut y_v: Vec<(i32, i32)> = Vec::new();
        for tupla in (0..y_vector.len()).rev(){
            y_v.push(y_vector[tupla]);
        }
        aux_vector.extend(y_v);
        vector.push(aux_vector);
    }
    (v, vector)
}

fn iterate_coordinates(v: Vec<Vec<(i32, i32)>>, x: i32) -> Vec<Vec<((i32, i32), (i32, i32))>>{
    let mut v_vector: Vec<Vec<((i32, i32), (i32, i32))>> = Vec::new();
    for element in v{
        let mut v_coordinates: Vec<((i32, i32), (i32, i32))> = Vec::new();
        for coordinate in 0..element.len(){
            let y = if (coordinate as i32 + x) > (element.len() as i32 ){
                (coordinate as i32 + x) % (element.len() as i32 )
            } else if (coordinate as i32 + x) == (element.len() as i32 ){
                0
            }
            else {
                coordinate as i32 + x
            };
            v_coordinates.push((element[y as usize], element[coordinate as usize]));
        }
        v_vector.push(v_coordinates);
    }
    v_vector
}

fn new_vec(v: Vec<Vec<i32>>, v_coordinates: Vec<Vec<((i32, i32), (i32, i32))>>) -> Vec<Vec<i32>>{
    let mut matriz: Vec<Vec<i32>> = Vec::new();
    for x in 0..v.len() as usize{
        let mut vector_aux: Vec<i32> = Vec::new();
        for _ in 0..v[x].len() as usize{
            vector_aux.push(2);
        }
        matriz.push(vector_aux);
    }
    for x in 0..v_coordinates.len() as usize{
        for y in 0..v_coordinates[x].len() as usize{
            matriz[(v_coordinates[x][y].0).0 as usize][(v_coordinates[x][y].0).1 as usize] = v[(v_coordinates[x][y].1).0 as usize][(v_coordinates[x][y].1).1 as usize]
        }
    }
    matriz
}

 fn main() {
    let v: fn() -> Vec<i32> = get_numbers;
    let mut mat = Matriz {vector: Vec::new()};
    let v_coor: fn(v: Vec<Vec<i32>>, x: i32) -> (Vec<Vec<i32>>, Vec<Vec<(i32, i32)>>) = get_coordinates;
    let mut v_coordinates: Vec<Vec<((i32, i32), (i32, i32))>> = Vec::new();
    let rows: Vec<i32> = v();
    let repeat = if rows[0] % 2 == 0{ (rows[0] / 2) as i32 } else { ((rows[0] /2 ) as i32) + 1 };
    for _ in 0..rows[0]{
        mat.vector.push(v());
    }
    let coordinate_tuple: (Vec<Vec<i32>>, Vec<Vec<(i32, i32)>>) = v_coor(mat.vector, repeat);
    v_coordinates = iterate_coordinates(coordinate_tuple.1, rows[2]);
    let matriz_shida: Vec<Vec<i32>> = new_vec(coordinate_tuple.0, v_coordinates);
    for x in matriz_shida{
        for y in x{
            print!("{} ", y);
        }
        println!(" ");
    }
}
