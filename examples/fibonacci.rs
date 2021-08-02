use alco::{black_box, runner, ParamBuilder, ParamSamples};

fn fibonacci(params: ParamSamples) {
    let n = params.get_usize("n").unwrap();

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
    let mut params = ParamBuilder::new();

    params.add_range("n", 0..100usize).unwrap();
    //params.add_combination("n, k, l").unwrap();
    //params.num_interactions(1).unwrap();

    runner(&[&("fibonacci", fibonacci, params)]);

    //let result = Benchmark::default()
        //.with_var("Fibonacci number", 5..20)
        //.run(|n, _| => fibonacci(n));
    


    //let complexity = Complexity::from_benchmark(result)
    //.default_terms()
    //.estimate();
}
