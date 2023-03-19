fn bisect(f: fn(f64) -> f64, min: f64, max: f64) -> Option<f64> {
    let mut a = min;
    let mut b = max;

    let mut i = 0;

    loop {
        let midpoint = a + (b - a) / 2.0;

        println!("{i}: {midpoint}");
        i += 1;

        if f64::abs(f(midpoint)) < 1e-4 {
            return Some(midpoint);
        } else if f(midpoint) * f(a) > 0.0 {
            a = midpoint;
            continue;
        } else {
            b = midpoint;
            continue;
        }
    }
}

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
    let f = |x: f64| x * x - 6.0 * x + 8.0;
    println!("{:?}", bisect(f, -2.0, 9.0));

    println!();

    // x = sqrt(6x - 8)
    let g = |x| f64::sqrt(6.0 * x - 8.0);
    println!("{:?}", stable_point(g, 12.0));

    println!();

    // x = -(x**2 + 8) / 6
    let g = |x| (f64::powi(x, 2) + 8.0) / 6.0;
    println!("{:?}", stable_point(g, 3.3));
}
