use std::time::Instant;
use ndarray::Array;

use std::cmp::min;
use rand::prelude::*;

const N:usize = 2000;
const D:usize = 50;

fn generate_matrix(is_random: bool) -> Vec<u32> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut arr: Vec<u32> = vec![0u32; N * N];
    if is_random {
        for i in 0..N {
            for j in 0..N {
                arr[i * N + j] = rng.gen_range(0..10);
            }
        }
    }
    arr
}

// #[allow(dead_code)]
// fn print_matrix(arr: &[[usize; N]; N]) {
//     for line in arr.iter(){
//         println!("{:?}", line);
//     }
//     println!()
// }

fn naive_multiply(arr1: &[u32], arr2: &[u32], res_arr: &mut [u32]) {
    for i in 0..N {
        for j in 0..N {
            let mut sum = 0;
            for k in 0..N {
                unsafe {
                    sum += *arr1.get_unchecked(i * N + k) * *arr2.get_unchecked(k * N + j);
                }
            }
            res_arr[i * N + j] = sum;
        }
    }
}

fn naive_iter_mult(arr1: &[u32], arr2: &[u32], res_arr: &mut [u32]) {
    for (ci, ai) in res_arr.chunks_exact_mut(N).zip(arr1.chunks_exact(N)){
        for (aik, bk) in ai.iter().zip(arr2.chunks_exact(N)) {
            for (cij, bkj) in ci.iter_mut().zip(bk.iter()){
                *cij += (*aik) * (*bkj);
            }
        }
    }
}

fn opt_multiply(arr1: &[u32], arr2: &[u32], res_arr: &mut [u32]) {
    for o in 0..=(N/D) {
        for i in 0..N {
            for j in 0..N {
                let mut sum: u32 = 0;
                for k in o * D..min(N, (o + 1) * D) {
                    unsafe {
                        sum += *arr1.get_unchecked(i * N + k) * *arr2.get_unchecked(k * N + j)
                    }
                }
                res_arr[i * N + j] = sum;
            }
        }

    }
}

fn main() {
    let array1 = generate_matrix(true);
    let array2 = generate_matrix(true);
    let mut array3 = generate_matrix(false);
    let mut array4 = array3.clone();
    let mut array5 = array3.clone();

    let nd_arr1 = Array::from_vec(array1.clone()).into_shape((N, N)).unwrap();
    let nd_arr2 = Array::from_vec(array2.clone()).into_shape((N, N)).unwrap();
    //
    // let mut array4 = generate_matrix(false);
    // print_matrix(&array1);
    // print_matrix(&array2);

    let start = Instant::now(); 
    naive_multiply(&array1, &array2, &mut array3);
    let duration = start.elapsed();
    println!("Time elapsed in naive_multiply() is: {:?}", duration);

    let start = Instant::now(); 
    naive_iter_mult(&array1, &array2, &mut array4);
    let duration = start.elapsed();
    println!("Time elapsed in naive_iter_multiply() is: {:?}", duration);

    let start = Instant::now(); 
    opt_multiply(&array1, &array2, &mut array5);
    let duration = start.elapsed();
    println!("Time elapsed in opt_multiply() is: {:?}", duration);

    let start = Instant::now(); 
    let _res_arr = nd_arr1.dot(&nd_arr2);
    let duration = start.elapsed();
    println!("Time elapsed in ndarray_multiply() is: {:?}", duration);

    println!("{:?}", array3);
    println!("{:?}", array4);
    println!("{:?}", array5);
    println!("{:?}", _res_arr);

    // let start = Instant::now(); 
    // opt_multiply(&array1, &array2, &mut array4);
    // let duration = start.elapsed();
    // println!("Time elapsed in opt_multiply() is: {:?}", duration);
    // print_matrix(&array3);

}
