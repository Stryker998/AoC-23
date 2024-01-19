mod part2;
use std::{fs, collections::HashMap};

struct Almanac {
    seeds: HashMap<usize, u64>,
    soils: HashMap<usize, u64>,
    fertilizers: HashMap<usize, u64>,
    waters: HashMap<usize, u64>,
    lights: HashMap<usize, u64>,
    temperatures: HashMap<usize, u64>,
    humidities: HashMap<usize, u64>,
    locations: HashMap<usize, u64>,
}

#[derive(Clone, Copy)]
enum AlmanacState {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Almanac {
    fn generate(&mut self, input: &String) {
        let mut almanac_state: Option<AlmanacState> = None; 
        let matchers = [
            (AlmanacState::SeedToSoil, "seed-to-soil"),
            (AlmanacState::SoilToFertilizer, "soil-to-fertilizer"),
            (AlmanacState::FertilizerToWater, "fertilizer-to-water"),
            (AlmanacState::WaterToLight, "water-to-light"),
            (AlmanacState::LightToTemperature, "light-to-temperature"),
            (AlmanacState::TemperatureToHumidity, "temperature-to-humidity"),
            (AlmanacState::HumidityToLocation, "humidity-to-location")
        ];
        'outer: for line in input.lines() {
            if line.contains("seeds") {
                line[7..].split(' ').for_each(|x| {
                    let seed = x.parse::<u64>().unwrap();
                    let v = self.seeds.len(); 
                    self.seeds.insert(v, seed);
                });
                continue;
            }
            for matcher in matchers.iter() {
                if line.contains(matcher.1) {
                    almanac_state = Some(matcher.0);
                    continue 'outer;
                }
            }
            if line.is_empty() {
                if let Some(state) = almanac_state {
                    match state {
                        AlmanacState::SeedToSoil => {
                            for (key, value) in self.seeds.iter() {
                                if !self.soils.contains_key(key) {
                                    self.soils.insert(*key, *value);
                                }
                            }
                        }
                        AlmanacState::SoilToFertilizer => {
                            for (key, value) in self.soils.iter() {
                                if !self.fertilizers.contains_key(key) {
                                    self.fertilizers.insert(*key, *value);
                                }
                            }
                        }
                        AlmanacState::FertilizerToWater => {
                            for (key, value) in self.fertilizers.iter() {
                                if !self.waters.contains_key(key) {
                                    self.waters.insert(*key, *value);
                                }
                            }
                        }
                        AlmanacState::WaterToLight => {
                            for (key, value) in self.waters.iter() {
                                if !self.lights.contains_key(key) {
                                    self.lights.insert(*key, *value);
                                }
                            }
                        }
                        AlmanacState::LightToTemperature => {
                            for (key, value) in self.lights.iter() {
                                if !self.temperatures.contains_key(key) {
                                    self.temperatures.insert(*key, *value);
                                }
                            }
                        }
                        AlmanacState::TemperatureToHumidity => {
                            for (key, value) in self.temperatures.iter() {
                                if !self.humidities.contains_key(key) {
                                    self.humidities.insert(*key, *value);
                                }
                            }
                        }
                        AlmanacState::HumidityToLocation => {
                            for (key, value) in self.humidities.iter() {
                                if !self.locations.contains_key(key) {
                                    self.locations.insert(*key, *value);
                                }
                            }
                        }
                    };
                }
                continue 'outer;
            }
            let current_line: Vec<u64> = line.split(' ').map(|str| str.parse::<u64>().unwrap()).collect();
            let (dest, source, range) = match &current_line[..] {
                &[x, y, z, ..] => (x, y, z),
                _ => continue 'outer,
            };
            let source_range = source..source+range;
            if let Some(state) = almanac_state {
                match state {
                    AlmanacState::SeedToSoil => {
                        for (key, value) in self.seeds.iter() {
                            if source_range.contains(value) {
                                self.soils.insert(*key, dest + (value - source));
                            }
                        }
                    }
                    AlmanacState::SoilToFertilizer => {
                        for (key, value) in self.soils.iter() {
                            if source_range.contains(value) {
                                self.fertilizers.insert(*key, dest + (value - source));
                            }
                        }
                    }
                    AlmanacState::FertilizerToWater => {
                        for (key, value) in self.fertilizers.iter() {
                            if source_range.contains(value) {
                                self.waters.insert(*key, dest + (value - source));
                            }
                        }
                    }
                    AlmanacState::WaterToLight => {
                        for (key, value) in self.waters.iter() {
                            if source_range.contains(value) {
                                self.lights.insert(*key, dest + (value - source));
                            }
                        }
                    }
                    AlmanacState::LightToTemperature => {
                        for (key, value) in self.lights.iter() {
                            if source_range.contains(value) {
                                self.temperatures.insert(*key, dest + (value - source));
                            }
                        }
                    }
                    AlmanacState::TemperatureToHumidity => {
                        for (key, value) in self.temperatures.iter() {
                            if source_range.contains(value) {
                                self.humidities.insert(*key, dest + (value - source));
                            }
                        }
                    }
                    AlmanacState::HumidityToLocation => {
                        for (key, value) in self.humidities.iter() {
                            if source_range.contains(value) {
                                self.locations.insert(*key, dest + (value - source));
                            }
                        }
                    }
                };
            }
        }
        let mut location_iter = self.locations.iter();
        let mut output = *location_iter.next().unwrap().1;
        location_iter.for_each(|x| {
            if output > *x.1 {
                output = *x.1;
            }
        });
        println!("location: {}", output); 
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut almanac = Almanac {
        seeds: HashMap::new(),
        soils: HashMap::new(),
        fertilizers: HashMap::new(),
        waters: HashMap::new(),
        lights: HashMap::new(),
        humidities: HashMap::new(),
        temperatures: HashMap::new(),
        locations: HashMap::new()
    };
    // almanac.generate(&input);
    part2::solution(&input);
}
