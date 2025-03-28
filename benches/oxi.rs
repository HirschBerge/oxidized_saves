use oxi::config::steam::discover_games;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_steam_discovery() {
    discover_games(false);
}

// #[divan::bench]
// fn bench_part2() {
//     part2();
// }
