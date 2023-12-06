use aoc2023_day5::read_input;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = read_input()?;

    if seeds.len() & 1 > 0 {
        println!("Invalid seeds input");
        return Ok(());
    }

    // let seeds = seeds
    //     .par_iter()
    //     .chunks(2)
    //     .map(|chunk| (*chunk[0]..*chunk[0] + *chunk[1]))
    //     .map(|range| range.map(|n| n).collect::<Vec<_>>())
    //     .flatten()
    //     .collect::<Vec<_>>();
    // println!("{} seeds to process…", seeds.len());
    //
    // let locations = seeds
    //     .par_iter()
    //     .map(|seed| seed_to_soil.transform(seed.clone()));
    // println!("seed-to-soil");
    // let locations = locations.map(|soil| soil_to_fertilizer.transform(soil));
    // println!("soil-to-fertilizer");
    // let locations = locations.map(|fertalizer| fertilizer_to_water.transform(fertalizer));
    // println!("fertilizer-to-water");
    // let locations = locations.map(|water| water_to_light.transform(water));
    // println!("water-to-light");
    // let locations = locations.map(|light| light_to_temperature.transform(light));
    // println!("light-to-temperature");
    // let locations = locations.map(|temperature| temperature_to_humidity.transform(temperature));
    // println!("temperature-to-humidity");
    // let locations = locations.map(|humidity| humidity_to_location.transform(humidity));

    let locations = seeds
        .par_iter()
        .chunks(2)
        .map(|chunk| (*chunk[0]..*chunk[0] + chunk[1]))
        .map(|range| range.map(|n| n).collect::<Vec<_>>())
        .map(|numbers| {
            numbers
                .par_iter()
                .map(|n| seed_to_soil.transform(*n))
                .map(|n| soil_to_fertilizer.transform(n))
                .map(|n| fertilizer_to_water.transform(n))
                .map(|n| water_to_light.transform(n))
                .map(|n| light_to_temperature.transform(n))
                .map(|n| temperature_to_humidity.transform(n))
                .map(|n| humidity_to_location.transform(n))
                .min()
        })
        .min();

    println!("{}", locations.unwrap().unwrap());
    Ok(())
}
