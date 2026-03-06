use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use palindrome::printer;

#[path = "../nontrivial_palindrome"]
mod palindrome;

//cargo bench --benches
pub fn criterion_benchmark(c: &mut Criterion) {

	c.bench_function("palindrome", |b| {
		b.iter(|| {
			std::hint::black_box(for i in 1..=100 {
				let num = rand::thread_rng().gen_range(0..100);
				_ = printer(num);
			});
		});
	});

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);