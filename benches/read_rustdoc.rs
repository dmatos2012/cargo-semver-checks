use anyhow::Context;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::path::Path;
use trustfall_rustdoc::{load_rustdoc, simd_load_rustdoc, VersionedCrate};

fn parse_rustdoc_small(path: &Path) -> VersionedCrate {
    //load_rustdoc(path)
    simd_load_rustdoc(path)
            .with_context(|| format!("Could not load {} file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?", path.display()))
            .expect("failed to load baseline rustdoc")
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("trait_newly_sealed", |b| {
        b.iter(|| {
            parse_rustdoc_small(black_box(Path::new(
                "./localdata/test_data/trait_newly_sealed/old/rustdoc.json",
            )))
        })
    });

    c.bench_function("auto_trait_impl_removed", |b| {
        b.iter(|| {
            parse_rustdoc_small(black_box(Path::new(
                "./localdata/test_data/auto_trait_impl_removed/old/rustdoc.json",
            )))
        })
    });

    c.bench_function("derive_trait_impl_removed", |b| {
        b.iter(|| {
            parse_rustdoc_small(black_box(Path::new(
                "./localdata/test_data/derive_trait_impl_removed/old/rustdoc.json",
            )))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
