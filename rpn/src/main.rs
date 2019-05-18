fn main() {
    println!("Hello, world!");
}

fn rpn(exp: &str) -> f64 {
    return 0.0
}

fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
    {
    }