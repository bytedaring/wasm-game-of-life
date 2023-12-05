use criterion::{Criterion, criterion_main, criterion_group};
use wasm_game_of_life::Universe;

extern crate wasm_game_of_life;

fn criterion_universe_ticks(c: &mut Criterion) {
    let mut universe = Universe::new();
    c.bench_function("ticks", |b| {
        b.iter(|| {
            universe.tick();
        })
    });
}

criterion_group!(benches, criterion_universe_ticks);
criterion_main!(benches);
