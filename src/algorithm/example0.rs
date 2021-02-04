use itertools::Itertools;
use rand::{distributions::Uniform, Rng};

fn is_converged(fitness: &Vec<i64>) -> bool {
    let max = fitness.iter().max().unwrap();
    *max == 0
}

fn selection_function(specimens: &Vec<i32>, selection: &Vec<i64>) -> Vec<i32> {
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

fn mutation_function(population: &mut Vec<i32>) {
    let range = Uniform::from(-2..2);
    let mutation = rand::thread_rng()
        .sample_iter(&range)
        .take(population.len())
        .collect::<Vec<i32>>();
    population
        .iter_mut()
        .zip(mutation)
        .for_each(|(specimen, mutation)| *specimen += mutation);
}

fn fitness_function(specimen: &i32) -> i64 {
    -(specimen - 101).pow(3).abs() as i64
}

pub fn run() {
    let range = Uniform::from(0..10);
    let initial_population: Vec<i32> = rand::thread_rng().sample_iter(&range).take(10).collect();
    crate::algorithm::run::<i32, i64>(
        initial_population,
        &fitness_function,
        &is_converged,
        &selection_function,
        &crossover_function,
        &mutation_function,
    );
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::algorithm::run;
    use test_env_log::test;

    #[test]
    fn test_fitness_function() {
        assert_eq!(fitness_function(&500), 500);
        assert_eq!(fitness_function(&0), 0);
        assert_eq!(fitness_function(&1000), 0);
        assert_eq!(fitness_function(&510), 490);
    }

    #[test]
    fn test_fitness_function_on_vec() {
        let population = vec![10, 20, 30];
        let fitness = population
            .iter()
            .map(|s| fitness_function(s))
            .collect::<Vec<i64>>();
        assert_eq!(fitness, vec![10, 20, 30]);
    }
}
