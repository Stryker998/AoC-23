use std::{fs, error::Error, collections::HashMap};

#[derive(Debug)]
enum Pipes {
    NorthSouth { up: (i32, i32), down: (i32, i32) },
    EastWest { left: (i32, i32), right: (i32, i32) },
    NorthEast { up: (i32, i32), right: (i32, i32) },
    NorthWest { up: (i32, i32), left: (i32, i32) },
    SouthWest { down: (i32, i32), left: (i32, i32) },
    SouthEast { down: (i32, i32), right: (i32, i32) },
    Source
}

impl Pipes {
    fn new(data: char, pos: (i32, i32)) -> Self {
        let up = (pos.0 - 1, pos.1);
        let down = (pos.0 + 1, pos.1);
        let left = (pos.0, pos.1 - 1);
        let right = (pos.0, pos.1 + 1);

        match data {
            '|' => {
                Self::NorthSouth { up, down }
            }
            '-' => {
                Self::EastWest { left, right }
            }
            'L' => {
                Self::NorthEast { up, right }
            }
            'J' => {
                Self::NorthWest { up, left }
            }
            '7' => {
                Self::SouthWest { down, left }
            }
            'F' => {
                Self::SouthEast { down, right }
            }
            'S' => {
                Self::Source 
            }
            _ => unreachable!()
        }
    }
}

struct SketchGraph {
    arena: Vec<Pipes>,
    nodes: HashMap<(i32, i32), usize>,
    source: ((i32, i32), usize)
}

impl SketchGraph {
    fn new(input: &String) -> Self{
        Self { arena: vec![], nodes: HashMap::new(), source: ((0, 0), 0)}.populate(input)
    }

    fn populate(mut self, input: &String) -> Self {
        let mut source_pos = (0 , 0);
        for (i, line) in (0..).zip(input.lines()) {
            for (j, char) in (0..).zip(line.chars()) {
                if char == '.' {
                    continue;
                }
                if char == 'S' {
                    self.source = ((i, j), self.arena.len());
                    source_pos = (i, j);
                }
                self.nodes.insert((i, j), self.arena.len());
                self.arena.push(Pipes::new(char, (i, j)));
            }
        }
        let pos = [
            (source_pos.0 - 1, source_pos.1),
            (source_pos.0 + 1, source_pos.1),
            (source_pos.0, source_pos.1 - 1),
            (source_pos.0, source_pos.1 + 1)
        ];
        let mut ends = (false, false, false, false);
        for (idx, pos) in pos.iter().enumerate() {
            if let Some(x) = self.nodes.get(pos) {
                let element = &self.arena[*x];
                match (element, idx) {
                    (Pipes::NorthSouth { down, .. } 
                     | Pipes::SouthEast { down, .. } 
                     | Pipes::SouthWest { down, .. }, 0) => {
                        if *down == source_pos {
                            ends.0 = true;
                        }
                    }
                    (Pipes::NorthSouth { up, .. }
                     | Pipes::NorthEast { up, .. }
                     | Pipes::NorthWest { up,.. }, 1) => {
                        if *up == source_pos {
                            ends.1 = true;
                        }
                    }                
                    (Pipes::NorthEast { right, .. }
                     | Pipes::EastWest { right, .. }
                     | Pipes::SouthEast { right, .. }, 2) => {
                        if *right == source_pos {
                            ends.2 = true;
                        }
                    }
                    (Pipes::NorthWest { left, .. }
                     | Pipes::EastWest { left, .. }
                     | Pipes::SouthWest { left, .. }, 3) => {
                        if *left == source_pos {
                            ends.3 = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        let source_type = match ends {
            (true, true, _, _) => {
                Pipes::NorthSouth { up: pos[0], down: pos[1] }
            }
            (true, _, true, _) => {
                Pipes::NorthWest { up: pos[0], left: pos[2] }
            }
            (true, _, _, true) => {
                Pipes::NorthEast { up: pos[0], right: pos[3] }
            }
            (_, true, true, _) => {
                Pipes::SouthWest { down: pos[1], left: pos[2] }
            }
            (_, true, _, true) => {
                Pipes::SouthEast { down: pos[1], right: pos[3] }
            }
            (_, _, true, true) => {
                Pipes::EastWest { left: pos[2], right: pos[3] }
            }
            _ => unreachable!()
        };
        self.arena[self.source.1] = source_type;
        self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let output = part1(&input);
    println!("part1: {}, part2: {}", output.0, output.1);
    Ok(())
}

fn part1(input: &String) -> (u32, i32) {
    let graph = SketchGraph::new(input);
    let get_index = |pos: (i32, i32)| -> usize { *graph.nodes.get(&pos).unwrap() };
    let source_pos = graph.source.0;
    let mut first = &graph.arena[get_index(source_pos)];
    let mut second = &graph.arena[get_index(source_pos)];
    let mut point_list = vec![];
    let mut next_first = source_pos;
    let mut next_second = source_pos;
    let mut prev_first = source_pos;
    let mut prev_second = source_pos;
    let checker = |old_pos: (i32, i32), x: (i32, i32), y: (i32, i32)| -> (i32, i32) {
        if x == old_pos {
            y
        } else if y == old_pos {
            x
        } else {
            println!("x: {:?}, y: {:?}, old_pos: {:?}", x, y, old_pos);
            unreachable!()
        }
    };
    let mut first_stack = vec![];
    let mut second_stack = vec![];
    first_stack.push(source_pos);
    second_stack.push(source_pos);
    let mut count = 0;
    loop {
        if count < 1 {
            (next_first, next_second) = match first {
                Pipes::NorthSouth { up, down } => (*up, *down),
                Pipes::EastWest { left, right } => (*left, *right),
                Pipes::NorthEast { up, right } => (*up, *right),
                Pipes::NorthWest { up, left } => (*up, *left),
                Pipes::SouthWest { down, left } => (*down, *left),
                Pipes::SouthEast { down, right } => (*down, *right),
                Pipes::Source => unreachable!(),
            };
        } else {
            first = &graph.arena[get_index(next_first)];
            second = &graph.arena[get_index(next_second)];
            (next_first, prev_first) = match first {
                Pipes::NorthSouth { up, down } => (checker(prev_first, *up, *down), next_first),
                Pipes::EastWest { left, right } => (checker(prev_first, *left, *right), next_first),
                Pipes::NorthEast { up, right } => (checker(prev_first, *up, *right), next_first),
                Pipes::NorthWest { up, left } => (checker(prev_first, *up, *left), next_first),
                Pipes::SouthWest { down, left } => (checker(prev_first, *down, *left), next_first),
                Pipes::SouthEast { down, right } => (checker(prev_first, *down, *right), next_first),
                _ => unreachable!()
            };
            (next_second, prev_second) = match second {
                Pipes::NorthSouth { up, down } => (checker(prev_second, *up, *down), next_second),
                Pipes::EastWest { left, right } => (checker(prev_second, *left, *right), next_second),
                Pipes::NorthEast { up, right } => (checker(prev_second, *up, *right), next_second),
                Pipes::NorthWest { up, left } => (checker(prev_second, *up, *left), next_second),
                Pipes::SouthWest { down, left } => (checker(prev_second, *down, *left), next_second),
                Pipes::SouthEast { down, right } => (checker(prev_second, *down, *right), next_second),
                _ => unreachable!()
            };        
        }
        count += 1;
        if next_first == next_second {
            first_stack.push(next_first);
            second_stack.reverse();
            point_list.append(&mut first_stack);
            point_list.append(&mut second_stack);
            break;
        }
        first_stack.push(next_first);
        second_stack.push(next_second);
    }
    (count, part2(&point_list))
}

fn part2(point_list: &Vec<(i32, i32)>) -> i32 {
    let determinant = |p1: &(i32, i32), p2: &(i32, i32)| {
        (p1.0 * p2.1) - (p1.1 * p2.0)
    };
    let mut shoelace = 0;
    for i in 1..point_list.len() {
        shoelace += determinant(&point_list[i - 1], &point_list[i]);
    }
    shoelace = shoelace.abs() / 2;
    let internal = shoelace + 1 - point_list.len() as i32 / 2;
    internal
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let test_string = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...".to_string();
        let test_string2 = ".....
.S-7.
.|.|.
.L-J.
.....
".to_string();
//        assert_eq!(8_u32, part1(&test_string, None));
//        assert_eq!(4_u32, part1(&test_string2, None));
    }

    #[test]
    fn part2_test() {
        let test_string = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".to_string();
        let test_string2 = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...".to_string();
        let test_string3 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".to_string();
        assert_eq!(4, part1(&test_string).1);
        assert_eq!(8, part1(&test_string2).1);
        assert_eq!(10, part1(&test_string3).1);
    }
}

//struct SketchArena {
//    arena: Vec<Node>,
//    nodes: HashMap<(i32, i32), usize>,
//    source: usize
//}
//
//impl SketchArena {
//    fn new() -> SketchArena {
//        SketchArena { arena: vec![], nodes: HashMap::new(), source: 0 }
//    }
//    fn make_graph(mut self, input: &String) -> SketchArena {
//        for (i, line) in (0..).zip(input.lines()) {
//            for (j, c) in (0..).zip(line.chars()) {
//                if c == '.' {
//                    continue;
//                }
//                self.nodes.insert((i, j), self.arena.len());
//                if c == 'S' {
//                    self.source = self.arena.len();
//                }
//                self.arena.push(Node::new((i, j), c));
//            };
//        };
//        self
//    }
//    fn connect_source(mut self) {
//        let source = self.source;
//        let source_node = &mut self.arena[source];
//        let source_pos = source_node.pos;
//        let mut temp = vec![];
//        for pos in source_node.ends.iter() {
//            if let Some(x) = self.nodes.get(pos) {
//                temp.push(*x);
//            }
//        }
//        for i in temp.into_iter() {
//        }
//    }
//}
//
//struct Node {
//    pos: (i32, i32),
//    data: char,
//    links: Vec<usize>,
//    ends: Vec<(i32, i32)>
//}
//
//impl Node {
//    fn new(pos: (i32, i32), data: char) -> Node {
//        Node { pos, data, links: vec![], ends: vec![] }.make_ends()
//    }
//    fn make_ends(mut self) -> Node {
//        self.ends = match self.data {
//            '|' => {
//                vec![(self.pos.0 + 1, self.pos.1), (self.pos.0 - 1, self.pos.1)]
//            }
//            '-' => {
//                vec![(self.pos.0, self.pos.1 + 1), (self.pos.0, self.pos.1 - 1)]
//            }
//            'L' => {
//                vec![(self.pos.0 - 1, self.pos.1), (self.pos.0, self.pos.1 + 1)]
//            }
//            'J' => {
//                vec![(self.pos.0 - 1, self.pos.1), (self.pos.0, self.pos.1 - 1)]
//            }
//            '7' => {
//                vec![(self.pos.0, self.pos.1 - 1), (self.pos.0 + 1, self.pos.1)]
//            }
//            'F' => {
//                vec![(self.pos.0, self.pos.1 + 1), (self.pos.0 + 1, self.pos.1)]
//            }
//            'S' => {
//                vec![
//                    (self.pos.0 + 1, self.pos.1), (self.pos.0 - 1, self.pos.1),
//                    (self.pos.0, self.pos.1 + 1), (self.pos.0, self.pos.1 + 1)
//                ]
//            }
//            s => { println!("{}", s); unreachable!() }
//        };
//        self
//    }
//}
