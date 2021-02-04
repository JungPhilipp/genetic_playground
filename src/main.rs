#![feature(bool_to_option)]
mod algorithm;

fn main() {
    env_logger::init();
    algorithm::example0::run();
}
