use alco::{black_box, runner};

fn fibonacci(n: Vec<usize>, setup: bool) {
    if setup {
        return;
    }

    let n = n[0];

    fn f(n: usize) -> usize {
        match n {
            0 => 1,
            1 => 1,
            n => f(n-1) + f(n-2),
        }
    }

    f(n);
}

fn main() {
    runner(&[&("fibonacci", fibonacci)]);

    //let result = Benchmark::default()
        //.with_var("Fibonacci number", 5..20)
        //.run(|n, _| => fibonacci(n));
    


    //let complexity = Complexity::from_benchmark(result)
    //.default_terms()
    //.estimate();
}
