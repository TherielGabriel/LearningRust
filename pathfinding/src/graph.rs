use rand::Rng;
use queue::Queue;
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};
use std::{thread, time};
use std::io::{Write, stdout};

#[derive(Clone, PartialEq)]
enum Tile {
    Start, End, Wall, Searched, Empty
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "S "),
            Tile::End => write!(f, "E "),
            Tile::Wall => write!(f, "# "),
            Tile::Searched => write!(f, ". "),
            Tile::Empty => write!(f, "  "),            
        }
    }
}

pub struct Graph {
    graph: Vec<Vec<Tile>>,
}

impl Graph {
    // While rust doesn't have any explicit constructors, best practice is 
    //  to create a function called "new" that returns an object of the class
    pub fn new() -> Self {
        let graph = Vec::new();
        Self { graph }
    }

    pub fn build(&mut self, width: usize, height: usize) {
        self.graph = Vec::new();
        for y in 0..height {
            self.graph.push(vec![Tile::Empty; width]);
            for x in 0..width {
                let mut tile = Tile::Empty;
                // Outer Walls
                if x == 0 || x == width - 1 {
                    tile = Tile::Wall;
                } else if y == 0 || y == height - 1 {
                    tile = Tile::Wall;
                } else if x % 2 == 0 && y % 2 == 0 {
                    tile = Tile::Wall;
                } else if (x % 2 == 0 && y % 2 == 1) || (x % 2 == 1 && y % 2 == 0) {
                    if rand::random::<f64>() > 0.7 {
                        tile = Tile::Wall;
                    }
                }
                self.graph[y][x] = tile;
            }
        }
        let mut rng = rand::thread_rng();
        self.graph[0][rng.gen_range(0..width/2) * 2 + 1] = Tile::Start;
        self.graph[height-1][rng.gen_range(0..width/2) * 2 + 1] = Tile::End;
    }

    pub fn run_bfs(&mut self) {
        let mut stdout = stdout();

        let mut frontier: Queue<(usize, usize)> = Queue::new();
        for (i, tile) in self.graph[0].iter().enumerate() {
            if *tile == Tile::Start {
                frontier.queue((0, i.try_into().unwrap())).unwrap();
            }
        }
        while !frontier.is_empty() {
            let target = frontier.dequeue().unwrap();
            self.graph[target.0][target.1] = Tile::Searched;

            stdout.execute(cursor::MoveTo(0, 1)).unwrap();
            stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
            stdout.write_all(format!("{}", self.as_str()).as_bytes()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(time::Duration::from_millis(100));

            for neighbor in self.neighbors_of(target).iter() {
                if self.graph[neighbor.0][neighbor.1] == Tile::Empty {
                    frontier.queue(*neighbor).unwrap();
                } else if self.graph[neighbor.0][neighbor.1] == Tile::End {
                    return;
                }
            }
        }
    }

    fn neighbors_of(&self, target: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        let (h, w) = (self.graph.len(), self.graph[0].len());
        match target {
            (a, b) if (0..h).contains(&(&a + 1)) && (0..w).contains(&b) 
                && self.graph[a + 1][b] == Tile::Empty => neighbors.push((target.0 + 1, target.1)),
            (a, b) if (0..h).contains(&(&a - 1)) && (0..w).contains(&b)
                && self.graph[a - 1][b] == Tile::Empty => neighbors.push((target.0 - 1, target.1)),
            (a, b) if (0..h).contains(&a) && (0..w).contains(&(&b + 1))
                && self.graph[a][b + 1] == Tile::Empty => neighbors.push((target.0, target.1 + 1)),
            (a, b) if (0..h).contains(&a) && (0..w).contains(&(&b - 1))
                && self.graph[a][b - 1] == Tile::Empty => neighbors.push((target.0, target.1 - 1)),
            _ => ()
        }
        neighbors
    }

    pub fn as_str(&self) -> String {
        let mut display = String::new();
        for row in self.graph.iter() {
            for tile in row.iter() {
                display.push_str(&tile.to_string());
            }
            display.push_str("\n");
        }
        display
    }
}