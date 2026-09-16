//! Criterion benchmark for document segmentation.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use olai_lib::services::segmentation;

fn sample_text() -> String {
    // A realistic multi-sentence page repeated to a few kilobytes.
    let clause = "The tenant must vacate the premises within seven days. \
        The deposit of Rs 60,000 is forfeited in full. \
        Maintenance charges are payable immediately. ";
    clause.repeat(40)
}

fn bench_segment_page(c: &mut Criterion) {
    let text = sample_text();
    c.bench_function("segment_page_5kb", |b| {
        b.iter(|| segmentation::segment_page(1, 1, black_box(&text)))
    });
}

criterion_group!(benches, bench_segment_page);
criterion_main!(benches);
