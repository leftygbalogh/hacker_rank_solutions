use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use proj_template::printer;
#[path = "../../util/screen.rs"]
mod screen;

#[path = "../proj_template.rs"]
mod proj_template;

//en/disable in Cargo.toml
//run with cargo bench --benches
pub fn criterion_benchmark(c: &mut Criterion) {

	c.bench_function("First Bench:\n", |b| {
		b.iter(|| {
			std::hint::black_box(for i in 1..=1 {
				let num = rand::thread_rng().gen_range(0..=42);
				_ = printer(num);
			});
		});
		println!("\n================\n");
	});

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(screen::clear, benches);