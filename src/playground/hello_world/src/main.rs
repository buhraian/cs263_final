fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        return 1;
    } else {
        // The recursive case.
        return fib(n-1) + fib(n-2);
    }
}

fn main() {
    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }
}