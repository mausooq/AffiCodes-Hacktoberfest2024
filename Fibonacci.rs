fn fibonacci(n: u32) -> Vec<u32> {
    let mut fib = vec![0, 1];
    for i in 2..n {
        let next = fib[i as usize - 1] + fib[i as usize - 2];
        fib.push(next);
    }
    fib
}

fn main() {
    let n = 10;
    println!("Fibonacci sequence up to {} terms: {:?}", n, fibonacci(n));
}
