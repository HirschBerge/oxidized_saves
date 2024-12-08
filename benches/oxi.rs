use oxi::config::steam::discover_steamgames;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_steam_discovery() {
    discover_steamgames(false);
}

// #[divan::bench]
// fn bench_part2() {
//     part2();
// }
