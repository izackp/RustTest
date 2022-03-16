/* Minimal example to showcase the Rust SDL2 bindings working with an emscripten target (asmjs,wasm32).
 * Install one or both of the following Rust target triples:
 *   rustup target add wasm32-unknown-emscripten
 *   rustup target add asmjs-unknown-emscripten
 *
 * Build:
 *   source emsdk/emsdk_env.sh 
 *   cd src/
 *   em++ -c gxx_personality_v0_stub.cpp
 *   cargo build --target=asmjs-unknown-emscripten --release
 *   cargo build --target=wasm32-unknown-emscripten --release
 *
 * Start a web server and run the example:
 *   emrun index-asmjs.html 
 *   (or emrun index-wasm.html)
 */
extern crate lone_game;

use lone_game::application::{Application};

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() -> Result<(), String> {

    test_file_read();
 
    let mut app = Application::new()?;
    app.init()?;
    app.run()?;

    Ok(())
}

fn test_file_read() {
    let mut file = File::open("data.txt").expect("Cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Cannot read file");

    let mut counter = HashMap::new();
    for line in contents.lines() {
        for word in line.trim().split(" ") {
            if word == "" {
                continue;
            }
            let counter = counter.entry(word).or_insert(0);
            *counter += 1;
        }
    }

    println!("Counter example:");
    for (word, n) in &counter {
        println!("{}: {}", word, n);
    }
}
