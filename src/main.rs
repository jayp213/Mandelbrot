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
    println!("Choose the X coord (Recomend : 0)");
    io::stdin().read_line(&mut input).expect("failed");
    let x_center: f64 = input.trim().parse().unwrap();

    // let mut input = String::new();
    // println!("Choose the max X coord (Recomend : 1.0)");
    // io::stdin().read_line(&mut input).expect("failed");
    // let x_max: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the Y coord (Recomend : 0)");
    io::stdin().read_line(&mut input).expect("failed");
    let y_center: f64 = input.trim().parse().unwrap();

    // let mut input = String::new();
    // println!("Choose the max Y coord (Recomend : 1.5)");
    // io::stdin().read_line(&mut input).expect("failed");
    // let y_max: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the zoom level (Recommend : 1.0)");
    io::stdin().read_line(&mut input).expect("failed");
    let zoom: f64 = input.trim().parse().unwrap();

    let mut input = String::new();
    println!("Choose the max number of iterations (Recomend : 1000)");
    io::stdin().read_line(&mut input).expect("failed");
    let max_iters: i64 = input.trim().parse().unwrap();



    print_box(size_input, max_iters, x_center - 2.0/zoom, x_center + 1.0/zoom, y_center-1.5/zoom, y_center+1.5/zoom, zoom);
}

fn print_box(size:i64, max_iters:i64, x_min:f64, x_max:f64, y_min:f64, y_max:f64, zoom:f64) {
    let size: i64 = size;
    for i in 0..size {
        for j in 0..(size as f64 *X_SCALE_FACTOR) as i64 {
            let iter = calculate_mandelbrod(j,i,(size as f64 * X_SCALE_FACTOR) as i64,size, max_iters, x_min, x_max, y_min, y_max);
            print_mandelbrod(iter, max_iters, zoom);
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
        if num.re*num.re + num.im * num.im >= 2.0 {
            return i;
        }
        num = num*num + c;
        it_count+= 1;
    }
    return it_count;
}

fn print_mandelbrod(iter:i64, max_iters:i64, zoom:f64) {
    let iter = iter as f64 * zoom;
    let max_iters = max_iters as f64 * zoom;
    if iter <= 1.0 {
        print!(" ")
    } else if iter > max_iters /1.1 {
        print!("$");
    } else if iter > max_iters/1.2 {
        print!("@");
    } else if iter > max_iters/1.4 {
        print!("B");
    } else if iter > max_iters/1.8 {
        print!("%");
    } else if iter > max_iters/2.5 {
        print!("8");
    } else if iter > max_iters/4.0 {
        print!("&");
    } else if iter > max_iters/6.0 {
        print!("W");
    } else if iter > max_iters/9.0 {
        print!("M");
    } else if iter > max_iters/16.0 {
        print!("#");
    } else if iter > max_iters/32.0 {
        print!("*");
    } else if iter > max_iters/64.0 {
        print!("o");
    } else if iter > max_iters/128.0 {
        print!("a");
    } else if iter > max_iters/256.0 {
        print!("h");
    } else if iter > max_iters/512.0 {
        print!("k");
    } else if iter > max_iters/1024.0 {
        print!("b");
    } else if iter > max_iters/2048.0 {
        print!("d");
    }else if iter > max_iters/4096.0 {
        print!("q");
    } else if iter > max_iters/8192.0 {
        print!("w");
    } else if iter > max_iters/16384.0 {
        print!("m");
    } else if iter > max_iters/32768.0 {
        print!("Z");
    } else if iter > max_iters/65536.0 {
        print!("0");
    } else if iter > max_iters/131072.0 {
        print!("r");
    } else if iter > max_iters/262144.0 {
        print!("•");
    } else if iter > max_iters/524288.0 {
        print!(".");
    }
     else { 
        print!(" ")
    }
}
