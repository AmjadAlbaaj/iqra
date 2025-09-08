use criterion::{Criterion, black_box, criterion_group, criterion_main};
use iqra::lang::{lex, parse};

fn bench_parser(c: &mut Criterion) {
    let src = include_str!("../examples/basics.iqra");
    c.bench_function("parse_basics", |b| {
        b.iter(|| {
            let toks = lex(black_box(src)).unwrap();
            parse(&toks).unwrap()
        })
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
