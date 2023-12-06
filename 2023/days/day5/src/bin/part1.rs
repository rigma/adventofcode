use aoc2023_day5::read_input;

fn main() -> anyhow::Result<()> {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertalizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = read_input()?;

    let locations = seeds
        .iter()
        .map(|seed| seed_to_soil.transform(seed.clone()))
        .map(|soil| soil_to_fertalizer.transform(soil))
        .map(|fertalizer| fertilizer_to_water.transform(fertalizer))
        .map(|water| water_to_light.transform(water))
        .map(|light| light_to_temperature.transform(light))
        .map(|temperature| temperature_to_humidity.transform(temperature))
        .map(|humidity| humidity_to_location.transform(humidity))
        .collect::<Vec<_>>();

    println!("{}", locations.iter().min().unwrap());
    Ok(())
}
