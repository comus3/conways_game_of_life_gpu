mod runner;
mod app;

use pollster::block_on;
use runner::Runner;

fn main() {
    let mut runner = Runner::new();
    block_on(runner.run());
}