use std::io;
use num::complex::Complex;

// const  X_MIN:f64 = -2.0;
// const  X_MAX:f64 = 1.0;
// const  Y_MIN:f64 = -1.5;
// const  Y_MAX:f64 = 1.5;
const X_SCALE_FACTOR:f64 = 2.5;

fn main() {
    let mut input = String::new();
    println!("Choose the size (Recomend : 450)");
    io::stdin().read_line(&mut input).expect("failed");
    let size_input: i64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the min X coord (Recomend : -2.0)");
    io::stdin().read_line(&mut input).expect("failed");
    let x_min: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the max X coord (Recomend : 1.0)");
    io::stdin().read_line(&mut input).expect("failed");
    let x_max: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the min Y coord (Recomend : -1.5)");
    io::stdin().read_line(&mut input).expect("failed");
    let y_min: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the max Y coord (Recomend : 1.5)");
    io::stdin().read_line(&mut input).expect("failed");
    let y_max: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the max number of iterations (Recomend : 1000)");
    io::stdin().read_line(&mut input).expect("failed");
    let max_iters: i64 = input.trim().parse().unwrap();
    print_box(size_input, max_iters, x_min, x_max, y_min, y_max);
}

fn print_box(x:i64, max_iters:i64, x_min:f64, x_max:f64, y_min:f64, y_max:f64) {
    let size: i64 = x;
    for i in 0..size {
        for j in 0..(size as f64 *X_SCALE_FACTOR) as i64 {
            let iter = calculate_mandelbrod(j,i,(size as f64 * X_SCALE_FACTOR) as i64,size, max_iters, x_min, x_max, y_min, y_max);
            print_mandelbrod(iter);
        }
        println!();
    }
}

fn calculate_mandelbrod(x:i64,y:i64,x_size:i64,y_size:i64, max_iters:i64, x_min:f64, x_max:f64, y_min:f64, y_max:f64) -> i64 {
    // return 0;
    let mut num = Complex{re: 0.0, im: 0.0};
    let c = Complex::new(x_min + (x_max - x_min) * (x as f64/x_size as f64), y_min + (y_max - y_min) * (y as f64/y_size as f64));
    let mut it_count = 0;
    for i in 0..max_iters {
        if num.re*num.re + num.im * num.im > 2.0 {
            return i;
        }
        num = num*num + c;
        it_count+= 1;
    }
    return it_count;
}

fn print_mandelbrod(iter:i64) {
    if iter > 500 {
        print!("#");
    } else if iter > 250 {
        print!("$");
    } else if iter > 125 {
        print!("&");
    }
    else if iter > 50 {
        print!("%");
    }
    else if iter > 25 {
        print!("0");
    } else if iter > 10 {
        print!("*");
    } else if iter > 5 {
        print!("•");
    } else if iter > 2 {
        print!(".");
    } else { 
        print!(" ")
    }
}
