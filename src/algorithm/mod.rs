pub fn run<T, I: Ord>(
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
    while !is_converged(&fitness) {
        let selection = selection_function(&population, &fitness);
        population = crossover_function(&selection, population_size);
        mutation_function(&mut population);
        fitness = population
            .iter()
            .map(|specimen| fitness_function(specimen))
            .collect::<Vec<I>>();
    }

    (population, fitness)
}
