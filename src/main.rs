use std::time::Instant;

use poople_solver::{app::PoopleSolverApp, metric::hamming_metric, words::PoopleGraph};
// fn main() {
//     let app = PoopleSolverApp::new().unwrap();
//     for i in &app.graph.words {
//         if let Some(path) = app.find(&i, "poop") {
//             println!("Solution for {} length {}", i, i.len());
//         } else {
//             println!("No path found for {}", i);
//         }
//     }
// }
fn main() {
    let mut arguments = std::env::args();
    let _ = arguments.next();
    let next = arguments.next().unwrap();
    let next1 = arguments.next().unwrap_or("poop".to_string());
    if next.len() > 4 {
        println!("poople words must be of length 4");
        return;
    }
    if next1.len() > 4 {
        println!("poople words must be of length 4");
        return;
    }
    let app = PoopleSolverApp::new().unwrap();
    if let Some(path) = app.find(&next, &next1) {
        for (i, word) in path.words.iter().rev().enumerate() {
            println!("[{i}] {word}");
        }
    } else {
        println!("No solution found. The word is either not in the poople dictionary or doesn't have a solution");
    }
}
