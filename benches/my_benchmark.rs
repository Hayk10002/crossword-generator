use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use crossword_generator::generator::{CrosswordGenerator, CrosswordGeneratorSettings};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("crossword");
    #[cfg(feature = "rec-iter")]
    group.bench_function(BenchmarkId::new("Recursive", ""), 
        |b| b.iter(||
        {
            let mut generator = CrosswordGenerator::default();
            generator.settings = CrosswordGeneratorSettings::default();
            generator.words = vec!["Hello", "world", "asdf", "myname", "sesame", "yeeee", "nouyt"].into_iter().map(|s| s.to_lowercase()).collect();
            generator.crossword_iter_rec().count();
        }));
    group.bench_function(BenchmarkId::new("Iterative", ""),
        |b| b.iter(||
        {
            let mut generator = CrosswordGenerator::default();
            generator.settings = CrosswordGeneratorSettings::default();
            generator.words = vec!["Hello", "world", "asdf", "myname", "sesame", "yeeee", "nouyt"].into_iter().map(|s| s.to_lowercase()).collect();
            generator.crossword_iter().count();
        }));

    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);