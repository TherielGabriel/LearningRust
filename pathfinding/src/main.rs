// https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
mod graph;
use graph::Graph;

use std::{thread, time};
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

fn main() {

    let (width, height) = (21, 21);

    let mut graph = Graph::new();
    graph.build(21, 21);

    graph.run_bfs();
    // // println!("{}", graph.as_str());

    // let mut stdout = stdout();

    // // stdout.execute(cursor::Hide).unwrap();
    // println!("Running algoritm");
    // // stdout.execute(cursor::MoveTo(0, 1)).unwrap();
    // stdout.execute(cursor::SavePosition).unwrap();
    // for i in (1..30).rev() {
    //     //stdout.execute(cursor::RestorePosition).unwrap();
    //     // stdout.execute(cursor::RestorePosition).unwrap();
    //     stdout.execute(cursor::MoveTo(0, 1)).unwrap();
    //     stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    //     // stdout.flush().unwrap();

    //     graph.build(21, 21);
    //     stdout.write_all(format!("{}", graph.as_str()).as_bytes()).unwrap();
    //     stdout.flush().unwrap();

    //     thread::sleep(time::Duration::from_millis(100));
    // }
    // // stdout.execute(cursor::Show).unwrap();

}
