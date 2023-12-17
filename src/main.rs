// use std::time::Instant;
// use std::cmp::min;
use rand::prelude::*;

const N:usize = 2000;
// const D:usize = 50;

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

// fn opt_multiply(arr1: &Vec<u32>, arr2: &Vec<u32>, res_arr: &mut Vec<u32>) {
//     for o in 0..=(N/D) {
//         for i in 0..N {
//             for j in 0..N {
//                 for k in o * D..min(N, (o + 1) * D) {
//                     res_arr[i * N + j] += arr1[i * N + k] * arr2[k * N + j]
//                 }
//             }
//         }

//     }
// }

fn main() {
    let array1 = generate_matrix(true);
    let array2 = generate_matrix(true);
    let mut array3 = generate_matrix(false);
    // let mut array4 = generate_matrix(false);
    // print_matrix(&array1);
    // print_matrix(&array2);

    // let start = Instant::now(); 
    naive_multiply(&array1, &array2, &mut array3);
    // let duration = start.elapsed();
    // println!("Time elapsed in naive_multiply() is: {:?}", duration);

    // let start = Instant::now(); 
    // opt_multiply(&array1, &array2, &mut array4);
    // let duration = start.elapsed();
    // println!("Time elapsed in opt_multiply() is: {:?}", duration);
    // print_matrix(&array3);

}
