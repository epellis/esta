#[macro_use]
extern crate criterion;

//use criterion::black_box;
use criterion::Criterion;

fn fibonacci() {
    //    use crate::*;
    use std::fs;
    let path = "./demos/fibonacci.est";
    let buffer = fs::read_to_string(path).expect("Couldn't read file!");
    match esta::run(&buffer) {
        Ok(()) => {}
        Err(why) => eprintln!("{}", why),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Fib 30", |b| b.iter(|| fibonacci()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
