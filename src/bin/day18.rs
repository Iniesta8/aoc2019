use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, io};

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapObject {
    Entrance,
    Empty,
    Wall,
    Key(char),
    Door(char),
}

impl From<char> for MapObject {
    fn from(c: char) -> Self {
        match c {
            '@' => MapObject::Entrance,
            '.' => MapObject::Empty,
            '#' => MapObject::Wall,
            'a'..='z' => MapObject::Key(c),
            'A'..='Z' => MapObject::Door(c),
            _ => panic!("unknown map object: {}", c),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct KeyRing {
    keys: [bool; 26],
    len: usize,
}

impl KeyRing {
    fn index_of(key: char) -> usize {
        (key.to_ascii_lowercase() as u32 - u32::from(b'a')) as usize
    }

    fn new() -> Self {
        Self {
            keys: [false; 26],
            len: 0,
        }
    }

    fn contains(self, key: char) -> bool {
        self.keys[Self::index_of(key)]
    }

    fn insert(&mut self, key: char) {
        if !self.keys[Self::index_of(key)] {
            self.keys[Self::index_of(key)] = true;
            self.len += 1;
        }
    }
}

fn parse_map(input: &str) -> Vec<Vec<MapObject>> {
    input
        .lines()
        .map(|l| l.trim().chars().map(MapObject::from).collect())
        .collect()
}

fn find_entrances(input: &str) -> Vec<(usize, usize)> {
    let mut entrances: Vec<(usize, usize)> = vec![];

    for (j, line) in input.lines().enumerate() {
        if line.contains('@') {
            for (i, c) in line.chars().enumerate() {
                if c == '@' {
                    entrances.push((i, j));
                }
            }
        }
    }

    entrances
}

fn get_possible_moves(
    map: &[Vec<MapObject>],
    from: (usize, usize),
    keys: KeyRing,
) -> HashMap<char, ((usize, usize), usize)> {
    let mut possible_moves = HashMap::new();
    let mut pos_to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    pos_to_visit.push_back(((from.0, from.1), 0));
    while let Some(pos) = pos_to_visit.pop_front() {
        let ((x, y), done_steps) = pos;
        for target in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
            let (tx, ty) = *target;
            let object = map[ty][tx];
            if visited.insert(*target) {
                match object {
                    MapObject::Empty | MapObject::Entrance => {
                        pos_to_visit.push_back((*target, done_steps + 1));
                    }
                    MapObject::Wall => {}
                    MapObject::Door(c) => {
                        if keys.contains(c.to_ascii_lowercase()) {
                            pos_to_visit.push_back((*target, done_steps + 1));
                        }
                    }
                    MapObject::Key(c) => {
                        if keys.contains(c.to_ascii_lowercase()) {
                            pos_to_visit.push_back((*target, done_steps + 1));
                        } else {
                            let entry =
                                possible_moves.entry(c).or_insert((*target, done_steps + 1));
                            if entry.1 > done_steps + 1 {
                                *entry = (entry.0, done_steps + 1);
                            }
                        }
                    }
                }
            }
        }
    }
    possible_moves
}

fn collect_keys(map: &[Vec<MapObject>], from: (usize, usize)) -> usize {
    let mut pos_to_visit = VecDeque::new();
    let mut visited = HashMap::new();
    let mut sum_steps = std::usize::MAX;

    pos_to_visit.push_back((from, 0, KeyRing::new()));
    while let Some((pos, steps, keys)) = pos_to_visit.pop_front() {
        let possible_moves = get_possible_moves(map, pos, keys);
        if possible_moves.is_empty() && steps < sum_steps {
            sum_steps = steps;
        }
        for (key, (pos, steps_to_pos)) in &possible_moves {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) = visited.get_mut(&(new_keys, *pos)) {
                if steps + *steps_to_pos < *did_visit {
                    pos_to_visit.push_back((*pos, steps + *steps_to_pos, new_keys));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert((new_keys, *pos), steps + *steps_to_pos);
                pos_to_visit.push_back((*pos, steps + *steps_to_pos, new_keys));
            }
        }
    }
    sum_steps
}

fn collect_keys_with_4_robots(map: &[Vec<MapObject>], from: Vec<(usize, usize)>) -> usize {
    let mut pos_to_visit = VecDeque::new();
    let mut visited = HashMap::new();
    let mut sum_steps = std::usize::MAX;

    pos_to_visit.push_back((from, 0, KeyRing::new()));
    while let Some((pos, steps, keys)) = pos_to_visit.pop_front() {
        let possible_moves0 = get_possible_moves(map, pos[0], keys);
        let possible_moves1 = get_possible_moves(map, pos[1], keys);
        let possible_moves2 = get_possible_moves(map, pos[2], keys);
        let possible_moves3 = get_possible_moves(map, pos[3], keys);
        if possible_moves0.is_empty()
            && possible_moves1.is_empty()
            && possible_moves2.is_empty()
            && possible_moves3.is_empty()
            && steps < sum_steps
        {
            sum_steps = steps;
        }

        for (key, (new_pos, steps_to_pos)) in &possible_moves0 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) =
                visited.get_mut(&(new_keys, vec![*new_pos, pos[1], pos[2], pos[3]]))
            {
                if steps + *steps_to_pos < *did_visit {
                    pos_to_visit.push_back((
                        vec![*new_pos, pos[1], pos[2], pos[3]],
                        steps + *steps_to_pos,
                        new_keys,
                    ));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert(
                    (new_keys, vec![*new_pos, pos[1], pos[2], pos[3]]),
                    steps + *steps_to_pos,
                );
                pos_to_visit.push_back((
                    vec![*new_pos, pos[1], pos[2], pos[3]],
                    steps + *steps_to_pos,
                    new_keys,
                ));
            }
        }
        for (key, (new_pos, steps_to_pos)) in &possible_moves1 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) =
                visited.get_mut(&(new_keys, vec![pos[0], *new_pos, pos[2], pos[3]]))
            {
                if steps + *steps_to_pos < *did_visit {
                    pos_to_visit.push_back((
                        vec![pos[0], *new_pos, pos[2], pos[3]],
                        steps + *steps_to_pos,
                        new_keys,
                    ));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert(
                    (new_keys, vec![pos[0], *new_pos, pos[2], pos[3]]),
                    steps + *steps_to_pos,
                );
                pos_to_visit.push_back((
                    vec![pos[0], *new_pos, pos[2], pos[3]],
                    steps + *steps_to_pos,
                    new_keys,
                ));
            }
        }
        for (key, (new_pos, steps_to_pos)) in &possible_moves2 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) =
                visited.get_mut(&(new_keys, vec![pos[0], pos[1], *new_pos, pos[3]]))
            {
                if steps + *steps_to_pos < *did_visit {
                    pos_to_visit.push_back((
                        vec![pos[0], pos[1], *new_pos, pos[3]],
                        steps + *steps_to_pos,
                        new_keys,
                    ));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert(
                    (new_keys, vec![pos[0], pos[1], *new_pos, pos[3]]),
                    steps + *steps_to_pos,
                );
                pos_to_visit.push_back((
                    vec![pos[0], pos[1], *new_pos, pos[3]],
                    steps + *steps_to_pos,
                    new_keys,
                ));
            }
        }
        for (key, (new_pos, steps_to_pos)) in &possible_moves3 {
            let mut new_keys = keys;
            new_keys.insert(*key);
            if let Some(did_visit) =
                visited.get_mut(&(new_keys, vec![pos[0], pos[1], pos[2], *new_pos]))
            {
                if steps + *steps_to_pos < *did_visit {
                    pos_to_visit.push_back((
                        vec![pos[0], pos[1], pos[2], *new_pos],
                        steps + *steps_to_pos,
                        new_keys,
                    ));
                    *did_visit = steps + *steps_to_pos;
                }
            } else {
                visited.insert(
                    (new_keys, vec![pos[0], pos[1], pos[2], *new_pos]),
                    steps + *steps_to_pos,
                );
                pos_to_visit.push_back((
                    vec![pos[0], pos[1], pos[2], *new_pos],
                    steps + *steps_to_pos,
                    new_keys,
                ));
            }
        }
    }
    sum_steps
}

fn manipulate_map(input: &String) -> String {
    let entrance = find_entrances(input);
    assert!(entrance.len() == 1);
    let (ex, ey) = *entrance.first().unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    map[ex][ey] = '#';
    map[ex - 1][ey] = '#';
    map[ex + 1][ey] = '#';
    map[ex][ey + 1] = '#';
    map[ex][ey - 1] = '#';
    map[ex - 1][ey - 1] = '@';
    map[ex + 1][ey - 1] = '@';
    map[ex - 1][ey + 1] = '@';
    map[ex + 1][ey + 1] = '@';

    let mut s = String::new();
    for line in map.iter() {
        for c in line.iter() {
            s.push(*c);
        }
        s.push('\n');
    }
    s
}

fn main() -> io::Result<()> {
    let input_a = fs::read_to_string("./input/day18.in")?;

    let map_a = parse_map(&input_a);
    let entrance_a = find_entrances(&input_a);
    assert!(entrance_a.len() == 1, "p1: no entrance found");
    println!("p1: {}", collect_keys(&map_a, *entrance_a.first().unwrap()));

    let input_b = manipulate_map(&input_a);
    let map_b = parse_map(&input_b);
    let entrances_b = find_entrances(&input_b);
    assert!(
        entrances_b.len() == 4,
        "p2: map should contain 4 entrances, but there is/are {}",
        entrances_b.len()
    );
    println!("p2: {}", collect_keys_with_4_robots(&map_b, entrances_b));

    Ok(())
}
