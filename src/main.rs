use std::usize;
use std::time::Instant;

use rand::prelude::*;

const N:usize = 2000;

fn generate_matrix(is_random: bool) -> Vec<Vec<usize>> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut arr = vec![vec![0usize; N]; N];
    if is_random {
        for line in arr.iter_mut() {
            for item in line.iter_mut(){
                *item = rng.gen_range(0..10);
            }
        }
    }
    arr
}

#[allow(dead_code)]
fn print_matrix(arr: &[[usize; N]; N]) {
    for line in arr.iter(){
        println!("{:?}", line);
    }
    println!()
}

fn naive_multiply(arr1: &Vec<Vec<usize>>, arr2: &Vec<Vec<usize>>, res_arr: &mut Vec<Vec<usize>>) {
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                res_arr[i][j] += arr1[i][k] * arr2[k][j]
            }
        }
    }
}

fn main() {
    let array1 = generate_matrix(true);
    let array2 = generate_matrix(true);
    let mut array3 = generate_matrix(false);
    // print_matrix(&array1);
    // print_matrix(&array2);
    let start = Instant::now(); 
    naive_multiply(&array1, &array2, &mut array3);
    let duration = start.elapsed();
    println!("Time elapsed in naive_multiply() is: {:?}", duration);
    // print_matrix(&array3);

}
