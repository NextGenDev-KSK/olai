//! Criterion benchmark for citation verification (fuzzy substring match).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use olai_lib::services::verification;

fn bench_similarity(c: &mut Criterion) {
    let span = "The tenant must vacate the premises within seven days of receipt of this notice.";
    let quote = "vacate the premises within seven days";
    c.bench_function("similarity_confirmed", |b| {
        b.iter(|| verification::similarity(black_box(quote), black_box(span)))
    });

    let fabricated = "you will definitely win this case in court next month";
    c.bench_function("similarity_fabricated", |b| {
        b.iter(|| verification::similarity(black_box(fabricated), black_box(span)))
    });
}

criterion_group!(benches, bench_similarity);
criterion_main!(benches);
