use std::{process::Output, ops::IndexMut};

#[derive(Debug)]
struct Entries {
    seeds: (u64, u64),
    soils: Vec<(u64, u64)>,
    fertilizers: Vec<(u64, u64)>,
    waters: Vec<(u64, u64)>,
    lights: Vec<(u64, u64)>,
    temperatures: Vec<(u64, u64)>,
    humidities: Vec<(u64, u64)>,
    locations: Vec<(u64, u64)>,
}

struct Almanac {
    entries: Vec<Entries>,
    seeds: Vec<(u64, u64)>,
    soils: Vec<(u64, u64, u64)>,
    fertilizers: Vec<(u64, u64, u64)>,
    waters: Vec<(u64, u64, u64)>,
    lights: Vec<(u64, u64, u64)>,
    temperatures: Vec<(u64, u64, u64)>,
    humidities: Vec<(u64, u64, u64)>,
    locations: Vec<(u64, u64, u64)>,
}

#[derive(Clone, Copy)]
enum AlmanacState {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Almanac {
    fn generate(&mut self, input: &String) {
        let mut almanac_state: Option<AlmanacState> = None; 
        let matchers = [
            (AlmanacState::Soil, "seed-to-soil"),
            (AlmanacState::Fertilizer, "soil-to-fertilizer"),
            (AlmanacState::Water, "fertilizer-to-water"),
            (AlmanacState::Light, "water-to-light"),
            (AlmanacState::Temperature, "light-to-temperature"),
            (AlmanacState::Humidity, "temperature-to-humidity"),
            (AlmanacState::Location, "humidity-to-location")
        ];
        'outer: for line in input.lines() {
            if line.is_empty() {
                almanac_state = None;
                continue;
            }
            if line.contains("seeds") {
                let mut temp: (u64, u64) = (0, 0);
                let mut temp1: Vec<u64> = line[7..].split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
                loop {
                    temp.1 = temp1.pop().unwrap();
                    temp.0 = temp1.pop().unwrap();
                    self.seeds.push(temp);
                    if temp1.len() == 0 {
                        break;
                    } 
                }
                continue;
            }
            if almanac_state.is_none() {
                for matcher in matchers.iter() {
                    if line.contains(matcher.1) {
                        almanac_state = Some(matcher.0);
                        continue 'outer;
                    }
                }
            }
            let current_line: Vec<u64> = line.split(' ').map(|str| str.parse::<u64>().unwrap()).collect();
            let to_add = match &current_line[..] {
                &[x, y, z, ..] => (x, y, z),
                _ => unreachable!(),
            };
            if let Some(state) = almanac_state {
                match state {
                    AlmanacState::Soil => {
                        self.soils.push(to_add)
                    }
                    AlmanacState::Fertilizer => {
                        self.fertilizers.push(to_add)
                    }
                    AlmanacState::Water => {
                        self.waters.push(to_add)
                    }
                    AlmanacState::Light => {
                        self.lights.push(to_add)
                    }
                    AlmanacState::Temperature => {
                        self.temperatures.push(to_add)
                    }
                    AlmanacState::Humidity => {
                        self.humidities.push(to_add)
                    }
                    AlmanacState::Location => {
                        self.locations.push(to_add)
                    }
                };
            }
        }
        self.soils.sort_by(|a, b| a.1.cmp(&b.1));
        self.fertilizers.sort_by(|a, b| a.1.cmp(&b.1));
        self.waters.sort_by(|a, b| a.1.cmp(&b.1));
        self.lights.sort_by(|a, b| a.1.cmp(&b.1));
        self.temperatures.sort_by(|a, b| a.1.cmp(&b.1));
        self.humidities.sort_by(|a, b| a.1.cmp(&b.1));
        self.locations.sort_by(|a, b| a.1.cmp(&b.1));
        let len = self.seeds.len();
        for idx in 0..len {
            let (mut lower, range) = self.seeds[idx];
            let upper = lower + range;
            let mut properties: Vec<Vec<(u64, u64)>> = Vec::new();
            let length = 7;
            let properties_global: Vec<&Vec<(u64, u64, u64)>> = vec![
              &self.soils,
              &self.fertilizers,
              &self.waters,
              &self.lights,
              &self.temperatures,
              &self.humidities,
              &self.locations
            ];
            let mut local = vec![];
            for (dest, source, range) in properties_global[0].iter() {
                let source_up = source + range;
                let dest_up = dest + range;
                if lower >= *source && upper < source_up {
                    let lower_range = lower - source;
                    let upper_range = source_up - upper;
                    local.push((dest + lower_range, dest_up - upper_range));
                    lower = upper;
                    break;
                }
                if lower < *source && upper < source_up && upper >= *source {
                    let upper_range = source_up - upper;
                    local.push((lower, *source));
                    local.push((*dest, dest_up - upper_range));
                    lower = upper;
                    break;
                }
                if lower >= *source && upper >= source_up && lower < source_up{
                    let lower_range = lower - source;
                    local.push((dest + lower_range, dest_up));
                    lower = source_up;
                    continue;
                }
                if lower < *source && upper >= source_up {
                    local.push((lower, *source));
                    local.push((*dest, dest_up));
                    lower = source_up;
                    continue;
                }
                if lower < *source && upper < *source {
                    local.push((lower, upper));
                    lower = upper;
                    break;
                }
            }
            if lower != upper {
                local.push((lower, upper));
            }
            properties.push(local);

            for i in 1..length {
                let prev = properties.get(i - 1).unwrap();
                let mut loc = vec![]; 
                let global = properties_global.get(i).unwrap();
                for (x, y) in prev {
                    let mut l = *x;
                    let u = *y;
                    for (dest, source, range) in global.iter() {
                        let source_up = source + range;
                        let dest_up = dest + range;
                        if l >= *source && u < source_up {
                            let l_range = l - source;
                            let u_range = source_up - u;
                            loc.push((dest + l_range, dest_up - u_range));
                            l = u;
                            break;
                        }
                        if l < *source && u < source_up && u >= *source {
                            let u_range = source_up - u;
                            loc.push((l, *source));
                            loc.push((*dest, dest_up - u_range));
                            l = u;
                            break;
                        }
                        if l >= *source && u >= source_up && l < source_up{
                            let l_range = l - source;
                            loc.push((dest + l_range, dest_up));
                            l = source_up;
                            continue;
                        }
                        if l < *source && u >= source_up {
                            loc.push((l, *source));
                            loc.push((*dest, dest_up));
                            l = source_up;
                            continue;
                        }
                        if l < *source && u < *source {
                            loc.push((l, u));
                            l = u;
                            break;
                        }
                    }
                    if l != u {
                        loc.push((l, u));
                    }
                }
                properties.push(loc);
            }
            properties.get_mut(6).unwrap().sort_by(|a, b| a.0.cmp(&b.0));
            let current_entry = Entries {
                seeds: self.seeds[idx], 
                soils: properties[0].to_vec(),
                fertilizers: properties[1].to_vec(),
                waters: properties[2].to_vec(),
                lights: properties[3].to_vec(),
                temperatures: properties[4].to_vec(),
                humidities: properties[5].to_vec(),
                locations: properties[6].to_vec()
            };
            self.entries.push(current_entry);
        }
    }
}


pub fn solution(input: &String) {
    let mut almanac = Almanac {
        entries: vec![],
        seeds: vec![],
        soils: vec![],
        fertilizers: vec![],
        waters: vec![],
        lights: vec![],
        humidities: vec![],
        temperatures: vec![],
        locations: vec![]
    };
    almanac.generate(input);
    println!("{:?}", almanac.entries);
    let mut entries_iter = almanac.entries.iter();
    let entry = entries_iter.next().unwrap();
    let mut output = entry.locations[0].0; 
    entries_iter.for_each(|x| {
        if output > x.locations[0].0 {
            output = x.locations[0].0;
        }
    });
    println!("output: {}", output);
}
