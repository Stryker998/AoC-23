use std::fs;

struct ArenaGraph {
    arena: Vec<Node>,
    numbers: Vec<usize>,
    symbols: Vec<usize>
}

enum DataType {
    Symbol,
    Number
}

struct Node {
    id: usize,
    char_id: Vec<(usize, usize)>,
    data: String,
    links: Vec<usize>
}

impl ArenaGraph {
    fn add(&mut self, char_id: Vec<(usize, usize)>, data: String, data_type: DataType, links: Vec<usize>) {
        let id = self.arena.len();
        match data_type {
            DataType::Symbol => self.symbols.push(id),
            DataType::Number => self.numbers.push(id),
        }
        self.arena.push( Node { id, char_id, data, links } );
    }
    fn links(&mut self){
        let symbol_ids = &self.symbols;
        let number_ids = &self.numbers;
        for idx in symbol_ids {
            let symbol = &self.arena[*idx];
            let (x, y) = symbol.char_id[0];
            let ids = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1), (x - 1, y), (x, y - 1), (x - 1, y - 1), (x + 1, y - 1), (x - 1, y + 1)];
            for num_idx in number_ids {
                let number = self.arena.get_mut(*num_idx).unwrap();
                for char_id in number.char_id.iter() {
                    if ids.contains(char_id) {
                        number.links.push(*idx);
                        break;
                    }
                }
            }
        }
        for num_idx in number_ids {
            let number = &self.arena[*num_idx];
            let link = number.links.get(0).cloned();
            if let Some(x) = link {
                self.arena.get_mut(x).unwrap().links.push(*num_idx);
            }
        }
    }
    fn part1(&self) {
        let number_ids = &self.numbers;
        let output: u32 = self.arena.iter().filter(|x| number_ids.contains(&x.id) && !x.links.is_empty()).map(|x| x.data.parse::<u32>().unwrap()).sum();
        println!("{}", output);
    }
    fn part2(&self) {
        let gears: Vec<&Node> = self.arena.iter().filter(|x| x.data == "*" && x.links.len() == 2 ).collect();
        let mut output = 0;
        gears.iter().for_each(|gear| {
            let mut gear_ratio: u32 = 1;
            gear.links.iter().for_each(|id| {
                gear_ratio *= self.arena[*id].data.parse::<u32>().unwrap();
            });
            output += gear_ratio;
        });
        println!("{}", output);
    }
}

impl Node {

}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut row = 0;
    let mut arena_graph = ArenaGraph {
        arena: vec![],
        numbers: vec![],
        symbols: vec![]
    };
    input.lines().for_each(|line| {
        let mut number = 0;
        let mut char_id = vec![];
        let mut number_flag = false;
        for (column, char) in line.char_indices() {
            if char.is_numeric() {
                number_flag = true;
                number = number * 10 + char.to_digit(10).unwrap();
                char_id.push((row, column));
                continue;
            }
            if number_flag {
                arena_graph.add(char_id.clone(), number.to_string(), DataType::Number, vec![]);
                number_flag = false;
                number = 0;
                char_id.clear();
            }
            if char == '.' {
                continue;
            }
            arena_graph.add(vec![(row, column)], char.to_string(), DataType::Symbol, vec![]);
        }
        if number_flag {
            arena_graph.add(char_id.clone(), number.to_string(), DataType::Number, vec![]);
        }
        row += 1;
    });
    arena_graph.links();
    arena_graph.part1();
    arena_graph.part2();
}
