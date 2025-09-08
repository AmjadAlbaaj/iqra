use criterion::{Criterion, black_box, criterion_group, criterion_main};
use iqra::lang::lex;

fn bench_lexer(c: &mut Criterion) {
    let src = include_str!("../examples/basics.iqra");
    c.bench_function("lexer_basics", |b| b.iter(|| lex(black_box(src)).unwrap()));
}

criterion_group!(benches, bench_lexer);
criterion_main!(benches);
