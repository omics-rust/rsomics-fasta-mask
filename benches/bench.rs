use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_fasta_mask(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-fasta-mask");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fasta = manifest.join("tests/golden/small.fa");
    let bed = manifest.join("tests/golden/mask.bed");
    c.bench_function("rsomics-fasta-mask golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .args([fasta.to_str().unwrap(), "-b", bed.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_fasta_mask);
criterion_main!(benches);
