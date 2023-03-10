// use tailcall::tailcall;

// fn stable_point_rec(g: fn(f64) -> f64, initial_guess: f64) -> f64 {
//
//     #[tailcall]
//     fn inner_loop(x: f64, g: fn(f64) -> f64) -> f64 {
//         let value = g(x);
//         println!("x0 = {x}");
//
//         let error = f64::abs((value - x) / value);
//
//         if error < 1e-7 {
//             x
//         } else {
//             inner_loop(value, g)
//         }
//     }
//
//     inner_loop(initial_guess, g)
// }

fn stable_point(g: fn(f64) -> f64, initial_guess: f64) -> Option<f64> {
    let mut x = initial_guess;

    let mut i = 0;
    while !x.is_infinite() && !x.is_nan() {
        let new_x = g(x);
        let error = f64::abs((new_x - x) / new_x);

        println!("{i}: {x}");

        if error < 1e-2 {
            return Some(new_x);
        } else {
            x = new_x;
            i += 1;
            continue;
        }
    }

    return None;
}

fn main() {
    // f(x) = x**2 - 6x + 8 = 0

    // x = sqrt(6x - 8)
    let g = |x| f64::sqrt(6.0 * x - 8.0);
    println!("{:?}", stable_point(g, 12.0));

    println!();

    // x = -(x**2 + 8) / 6
    let g = |x| (f64::powi(x, 2) + 8.0) / 6.0;
    println!("{:?}", stable_point(g, 3.3));
}
