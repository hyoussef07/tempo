use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempo::{dt, Duration};

fn bench_chain_operations(c: &mut Criterion) {
    let dur = Duration::from_object(&[("days", 100)]);
    c.bench_function("tempo chain", |b| {
        b.iter(|| {
            dt().plus(black_box(&dur))
                .start_of("year")
                .to_format("yyyy-MM-dd")
        });
    });
}

fn bench_format_iso(c: &mut Criterion) {
    let dt_val = dt();
    c.bench_function("to_iso", |b| {
        b.iter(|| black_box(&dt_val).to_iso());
    });
}

fn bench_format_custom(c: &mut Criterion) {
    let dt_val = dt();
    c.bench_function("to_format complex", |b| {
        b.iter(|| black_box(&dt_val).to_format("EEEE, MMMM do yyyy 'at' h:mm:ss a"));
    });
}

fn bench_duration_conversion(c: &mut Criterion) {
    let dur = Duration::from_object(&[("weeks", 2), ("days", 3), ("hours", 4)]);
    c.bench_function("duration as_unit", |b| {
        b.iter(|| black_box(&dur).as_unit("seconds"));
    });
}

fn bench_plus_operation(c: &mut Criterion) {
    let dt_val = dt();
    let dur = Duration::from_object(&[("days", 30)]);
    c.bench_function("datetime plus", |b| {
        b.iter(|| black_box(&dt_val).clone().plus(black_box(&dur)));
    });
}

fn bench_start_of(c: &mut Criterion) {
    let dt_val = dt();
    c.bench_function("start_of day", |b| {
        b.iter(|| black_box(&dt_val).clone().start_of("day"));
    });
}

fn bench_diff(c: &mut Criterion) {
    let dt1 = dt();
    let dt2 = dt().plus(&Duration::from_object(&[("days", 100)]));
    c.bench_function("diff days", |b| {
        b.iter(|| black_box(&dt1).diff(black_box(&dt2), "days"));
    });
}

criterion_group!(
    benches,
    bench_chain_operations,
    bench_format_iso,
    bench_format_custom,
    bench_duration_conversion,
    bench_plus_operation,
    bench_start_of,
    bench_diff
);
criterion_main!(benches);
