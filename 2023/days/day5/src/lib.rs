pub mod almanac_mapping;
pub mod almanac_range;

pub use almanac_mapping::AlmanacMapping;
pub use almanac_range::AlmanacRange;
use std::{io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlmanacError {
    #[error("not enought value to unpack")]
    Unpack,
    #[error("integer parsing error")]
    IntError(ParseIntError),
    #[error("I/O error")]
    IOError(io::Error),
    #[error("missing seeds from input")]
    MissingSeeds,
    #[error("missing section")]
    MissingSection(String),
}

impl PartialEq for AlmanacError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Unpack, &Self::Unpack) => true,
            (&Self::IntError(_), &Self::IntError(_)) => true,
            (&Self::MissingSeeds, &Self::MissingSeeds) => true,
            _ => false,
        }
    }
}

fn check_header<'a>(
    iterator: &mut impl Iterator<Item = &'a str>,
    header_name: &str,
) -> Result<(), AlmanacError> {
    if let Some(header) = iterator.next() {
        if !header.starts_with(header_name) {
            return Err(AlmanacError::MissingSection(header_name.to_string()));
        }
    } else {
        return Err(AlmanacError::MissingSection(header_name.to_string()));
    }

    Ok(())
}

fn read_mapping<'a>(
    iterator: &mut impl Iterator<Item = &'a str>,
) -> Result<AlmanacMapping, AlmanacError> {
    let mut lines = Vec::new();

    while let Some(line) = iterator.next() {
        if line.is_empty() {
            break;
        }

        lines.push(line);
    }

    Ok(AlmanacMapping::from_iter(lines))
}

pub fn read_input() -> Result<
    (
        Vec<u64>,
        AlmanacMapping,
        AlmanacMapping,
        AlmanacMapping,
        AlmanacMapping,
        AlmanacMapping,
        AlmanacMapping,
        AlmanacMapping,
    ),
    AlmanacError,
> {
    let input = aoc2023_utils::read_input().map_err(|err| AlmanacError::IOError(err))?;
    let mut input = input.lines();

    // seed line
    let seeds = if let Some(seeds) = input.next() {
        if !seeds.starts_with("seeds: ") {
            return Err(AlmanacError::MissingSeeds);
        }

        seeds[7..]
            .split(" ")
            .map(|seed| seed.parse::<u64>().expect("invalid integer"))
            .collect()
    } else {
        return Err(AlmanacError::MissingSeeds);
    };

    let mut input = input.skip(1);

    // seed-to-soil
    check_header(&mut input, "seed-to-soil")?;
    let seed_to_soil = read_mapping(&mut input)?;

    // soil-to-fertilizer
    check_header(&mut input, "soil-to-fertilizer")?;
    let soil_to_fertilizer = read_mapping(&mut input)?;

    // fertalizer-to-water
    check_header(&mut input, "fertilizer-to-water")?;
    let fertilizer_to_water = read_mapping(&mut input)?;

    // water-to-light
    check_header(&mut input, "water-to-light")?;
    let water_to_light = read_mapping(&mut input)?;

    // light-to-temperature
    check_header(&mut input, "light-to-temperature")?;
    let light_to_temperature = read_mapping(&mut input)?;

    // temperature-to-humidity
    check_header(&mut input, "temperature-to-humidity")?;
    let temperature_to_humidity = read_mapping(&mut input)?;

    // humidity-to-location
    check_header(&mut input, "humidity-to-location")?;
    let humidity_to_location = read_mapping(&mut input)?;

    Ok((
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ))
}
