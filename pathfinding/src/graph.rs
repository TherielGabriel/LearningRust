use crossterm::{cursor, terminal, QueueableCommand};
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::io::{stdout, Stdout, Write};
use std::{thread, time};

#[derive(Clone, PartialEq)]
enum Tile {
    Start,
    End,
    Wall,
    Searched,
    Path,
    Empty,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Start => write!(f, "S "),
            Tile::End => write!(f, "E "),
            Tile::Wall => write!(f, "# "),
            Tile::Searched => write!(f, ". "),
            Tile::Path => write!(f, "X "),
            Tile::Empty => write!(f, "  "),
        }
    }
}

pub struct Graph {
    graph: Vec<Vec<Tile>>,
    stdout: Stdout,
}

impl Graph {
    // While rust doesn't have any explicit constructors, best practice is
    //  to create a function called "new" that returns an object of the class
    pub fn new() -> Self {
        Self::from(11, 11)
    }

    pub fn from(width: usize, height: usize) -> Self {
        let mut graph = Vec::new();
        for y in 0..height {
            graph.push(vec![Tile::Empty; width]);
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
                graph[y][x] = tile;
            }
        }
        let mut rng = rand::thread_rng();
        graph[0][rng.gen_range(0..width / 2) * 2 + 1] = Tile::Start;
        graph[height - 1][rng.gen_range(0..width / 2) * 2 + 1] = Tile::End;
        let stdout = stdout();
        Self { graph, stdout }
    }

    pub fn run_bfs(
        &mut self,
    ) -> Result<HashMap<(usize, usize), (usize, usize)>, Box<dyn std::error::Error>> {
        // HashMap <Destination, Source>
        let mut paths: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        // Frontier: (y, x)
        let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
        let start: (usize, usize) = self.get_start_end().0;
        paths.insert((start.0 + 1, start.1), start);
        frontier.push_back((start.0 + 1, start.1));
        while !frontier.is_empty() {
            let target = frontier.pop_front().unwrap();
            self.graph[target.0][target.1] = Tile::Searched;
            self.print()?;
            thread::sleep(time::Duration::from_millis(5));
            for neighbor in self.neighbors_of(target) {
                if self.graph[neighbor.0][neighbor.1] == Tile::Empty {
                    frontier.push_back(neighbor);
                    paths.insert(neighbor, target);
                } else if self.graph[neighbor.0][neighbor.1] == Tile::End {
                    paths.insert(neighbor, target);
                    return Ok(paths);
                }
            }
        }
        Err("Path not found".into())
    }

    fn neighbors_of(&self, (y, x): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (h, w) = (self.graph.len(), self.graph[0].len());
        let neighbors = [
            (y + 1, x),
            (y.wrapping_sub(1), x),
            (y, x + 1),
            (y, x.wrapping_sub(1)),
        ];
        neighbors.into_iter().filter(move |(i, j)| {
            (0..w).contains(j)
                && (0..h).contains(i)
                && matches!(self.graph[*i][*j], Tile::Empty | Tile::End)
        })
    }

    fn get_start_end(&self) -> ((usize, usize), (usize, usize)) {
        let mut start = (0, 0);
        for (i, tile) in self.graph[0].iter().enumerate() {
            if *tile == Tile::Start {
                start = (0, i.try_into().unwrap());
                break;
            }
        }
        let mut end = (0, 0);
        for (i, tile) in self.graph[self.graph.len() - 1].iter().enumerate() {
            if *tile == Tile::End {
                end = (self.graph.len() - 1, i.try_into().unwrap());
                break;
            }
        }
        (start, end)
    }

    fn print(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Drawing graph on top of previous output
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;
        self.stdout
            .write_all(format!("{}", self.as_str()).as_bytes())?;
        self.stdout.flush().unwrap();
        Ok(())
    }

    pub fn print_path(
        &mut self,
        paths: HashMap<(usize, usize), (usize, usize)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (start, end) = self.get_start_end();
        let mut current = end;
        while current != start {
            self.graph[current.0][current.1] = Tile::Path;
            current = *paths.get(&current).unwrap();

            // Drawing graph on top of previous output
            self.print()?;
            thread::sleep(time::Duration::from_millis(100));
        }
        Ok(())
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
