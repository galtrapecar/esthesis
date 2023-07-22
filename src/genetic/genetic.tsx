import { COLOR_TARGET, HEIGHT_MAX, HEIGHT_TARGET, IGenotype, MUTATION_PROBABILITY, POPULATION_SIZE, boxmuller, hexToRgb, randomColor, rgbToHex } from "./helpers"

export function init(): IGenotype[] {
    let population: IGenotype[] = []

    for (let i = 0; i < POPULATION_SIZE; i++) {
        let genotype: IGenotype = {
            color: randomColor(),
            height: Math.floor((Math.random() * 250 ) + 1)
        }
        population.push(genotype)
    }

    return population
}

export function fitness(phenotype: IGenotype) {
    let diviation = 0

    diviation += Math.abs(phenotype.height - HEIGHT_TARGET)
    let [r, g, b]    = hexToRgb(phenotype.color)
    let [tr, tg, tb] = hexToRgb(COLOR_TARGET)

    diviation += Math.abs(tr - r)
    diviation += Math.abs(tg - g)
    diviation += Math.abs(tb - b)

    phenotype.fitness = diviation

    return phenotype
}

function mutate(genotype: IGenotype) {
    const COLOR_DELTA_MAX = 15
    const HEIGHT_DELTA_MAX = 20
    let mutant: IGenotype = { color: "", height: 0 }

    let [r, g, b] = hexToRgb(genotype.color).map(c => c + Math.ceil((boxmuller() * (2 * COLOR_DELTA_MAX)) - COLOR_DELTA_MAX))
    let hex = rgbToHex(r, g, b)
    mutant.color = hex

    let height = genotype.height + Math.ceil((boxmuller() * (2 * HEIGHT_DELTA_MAX)) - HEIGHT_DELTA_MAX)
    mutant.height = height

    return mutant
}

function crossover(genotype: IGenotype, genotypes: IGenotype[]) {
    let child = {color: "", height: 0}
    let mate = genotypes[Math.floor(Math.random() * genotypes.length)]

    const avg = (a: number, b: number) => (a + b) / 2
    
    let [r, g, b]    = hexToRgb(genotype.color)
    let [rm, gm, bm] = hexToRgb(mate.color)

    child.color  = rgbToHex(avg(r, rm), avg(g, gm), avg(b, bm))
    child.height = (genotype.height + mate.height) / 2

    return child
}

export function new_population(_phenotypes: IGenotype[]) {
    let phenotypes = []
    for (const phenotype of _phenotypes) {
        phenotypes.push({...fitness(phenotype)})
    }

    const best = phenotypes.sort((a, b) => {
        if (!a.fitness) a.fitness = 99999
        if (!b.fitness) b.fitness = 99999
        if (a.fitness < b.fitness ){
            return -1;
        }
        if ( a.fitness > b.fitness ){
            return 1;
        }
            return 0;
    }).slice(0, 5)

    const chunk = Math.ceil(POPULATION_SIZE / best.length)

    let new_population: IGenotype[] = []

    let best_of_current_count = 0
    let best_of_all

    best.forEach(genotype => {
        for (let i = 0; i < chunk; i++) {
            if (new_population.length == POPULATION_SIZE) break
            if (genotype.fitness && genotype.fitness < 1.1) {
                delete genotype.fitness
                new_population.push({...genotype})
                // console.log("found best of current");
                best_of_current_count += 1
                if (best_of_current_count > 4) best_of_all = genotype
            } else {
                let new_genotype = crossover(genotype, best)
                if (Math.random() < MUTATION_PROBABILITY) new_genotype = mutate(new_genotype)
                new_population.push(new_genotype)
            }
        }
    })

    if (best_of_current_count > 4) {
        return Array(POPULATION_SIZE + 1).fill(best_of_all)
    }

    return new_population
}