#![feature(bool_to_option)]

use algorithm::run;
use itertools::Itertools;
use rand::{distributions::Uniform, Rng};
mod algorithm;

fn is_converged(specimens: &Vec<u32>) -> bool {
    let max = specimens.iter().max().unwrap();
    println!("{}", max);
    return (*max as i64 - 50).abs() < 2;
}

fn selection_function(specimens: &Vec<i32>, selection: &Vec<u32>) -> Vec<i32> {
    let selection_count = 2;
    specimens
        .into_iter()
        .zip(selection)
        .sorted_by(|(_, fitness_lhs), (_, fitness_rhs)| {
            fitness_lhs.partial_cmp(fitness_rhs).unwrap()
        })
        .map(|(specimen, _)| *specimen)
        .rev()
        .take(selection_count)
        .collect()
}

fn crossover_function(selection: &Vec<i32>, population_count: usize) -> Vec<i32> {
    assert!(selection.len() == 2);
    let a = selection[0];
    let b = selection[1];
    let dist = (b - a).abs();

    let range = Uniform::from(-50..150);
    rand::thread_rng()
        .sample_iter(&range)
        .take(population_count)
        .map(|r| {
            let factor = r as f64 / 100.;
            a + (factor * dist as f64) as i32
        })
        .collect()
}

fn mutation_function(_population: &mut Vec<i32>) {}

fn fitness_function(specimen: &i32) -> u32 {
    (50 - specimen).abs() as u32
}

fn main() {
    let range = Uniform::from(0..100);
    let initial_population: Vec<i32> = rand::thread_rng().sample_iter(&range).take(1000).collect();
    run::<i32, u32>(
        initial_population,
        &fitness_function,
        &is_converged,
        &selection_function,
        &crossover_function,
        &mutation_function,
    );
}
