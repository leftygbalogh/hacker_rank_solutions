use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
#[path = "../../util/screen.rs"]
mod screen;

#[path = "../palindrome.rs"]
mod palindrome;
use palindrome::isAlphabeticPalindrome;

//cargo bench --benches
pub fn criterion_benchmark1(c: &mut Criterion) {
	screen::clear();

	c.bench_function("palindrome one", |b| {
		b.iter(|| {
			std::hint::black_box(for i in 1..=100 {
				let input = black_box("abc!def<>?fed&cba");
				_ = isAlphabeticPalindrome(input);
			});
		});
	});

}

pub fn criterion_benchmark2(c: &mut Criterion) {
	

	c.bench_function("palindrome two", |b| {
		b.iter(|| {
			std::hint::black_box(for i in 1..=100 {
				let input = black_box("abc!def<>?fed&cba46AACC57#tcBT,abx0.£$%,,,,%8r4V4#75D771F35FD5AB64m07qFxDl.$lDxFq70m%46BA5DF56B929618&to4khLbW:~@%^WbLhk4ot)))__)(((816929B6NotpvSAB0D3CCAE568747E6yuuVHd7n568747E6yuuVHd7n6E747865DD5A014B%QbFUJct8DD5A014B%QbFUJct8B410A5DDE3825639.TxEf11k4E3825639.TxEf11k49365283EF385BD44.164N2Pd9F385BD44.164N2Pd944DB583FB06B82EDvykdfRQ7B06B82EDvykdfRQ7DE28B60B91030211%UluNYz5H91030211%UluNYz5H112030197C38BC87!KPtzSuVx7C38BC87!KPtzSuVx78CB83C7vSAB0D3CCAE568747E6yuuVHd7n568747E6yuuVHd7n6E747865DD5A014B%QbFUJct8DD5A014B%QbFUJct8B410A5DDE3825639.TxEf11k4E3825639.TxEf11k49365283EF385BD44.164N2Pd9F385BD44.164N2Pd944DB583FB06B82EDvykdfRQ7B06B82EDvykdfRQ7DE28B60B91030211%UluNYz5H91030211%UluNYz5H112030197C38BC87!KPtzSuVx7C38BC87!KPtzSuVx78CB83C7alindromesEACC3D0B/FDAIGvSAEACC3D0B/FDAIGvSAB0D3CCAE568747E6yuuVHd7n568747E6yuuVHd7n6E747865DD5A014B%QbFUJct8DD5A014B%QbFUJct8B410A5DDE3825639.TxEf11k4E3825639.TxEf11k49365283EF385BD44.164N2Pd9F385BD44.164N2Pd944DB583FB06B82EDvykdfRQ7B06B82EDvykdfRQ7DE28B60B91030211%UluNYz5H91030211%UluNYz5H112030197C38BC87!KPtzSuVx7C38BC87!KPtzSuVx78CB83C7");
				_ = isAlphabeticPalindrome(input);
			});
		});
	});

}


criterion_group!(benches, criterion_benchmark1, criterion_benchmark2);
criterion_main!(benches);