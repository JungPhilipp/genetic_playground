pub mod example0;
use log::{debug, info};
use std::{thread, time};

pub fn run<T: std::fmt::Debug, I: Ord + std::fmt::Display + std::fmt::Debug>(
    initial_population: Vec<T>,
    fitness_function: &dyn Fn(&T) -> I,
    is_converged: &dyn Fn(&Vec<I>) -> bool,
    selection_function: &dyn Fn(&Vec<T>, &Vec<I>) -> Vec<T>,
    crossover_function: &dyn Fn(&Vec<T>, usize) -> Vec<T>,
    mutation_function: &dyn Fn(&mut Vec<T>),
) -> (Vec<T>, Vec<I>) {
    let population_size = initial_population.len();
    let mut population = initial_population;
    let mut fitness = population
        .iter()
        .map(|specimen| fitness_function(specimen))
        .collect::<Vec<I>>();
    debug!("Initial Population: {:?}", population);
    debug!("Initial Population Size: {}", population_size);
    info!("Initial Fitness: {:?}", fitness);
    let mut _generation = 0;
    while !is_converged(&fitness) {
        _generation += 1;
        let selection = selection_function(&population, &fitness);
        population = crossover_function(&selection, population_size);
        mutation_function(&mut population);
        fitness = population
            .iter()
            .map(|specimen| fitness_function(specimen))
            .collect::<Vec<I>>();
        debug!("Generation: {}", _generation);
        debug!("Maximum Fitness: {}", fitness.iter().max().unwrap());
        debug!("{:?} \n {:?}", population, fitness);
    }

    info!("Converged Generation: {}", _generation);
    info!(
        "Converged Fitness: {:?}",
        population
            .iter()
            .zip(fitness.iter())
            .max_by(|(_, lhs), (_, rhs)| lhs.partial_cmp(rhs).unwrap())
            .unwrap()
    );
    (population, fitness)
}
