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

use lone_game::application::window::{Window, WindowDelegate};
use lone_game::application::{Application, AppDelegate};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::WindowPos;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct TestGameAppDelegate {
    idk:u32
}

struct TestWindowDelegate {
    //app:&'a Application<'a>
}

impl WindowDelegate for TestWindowDelegate {
    fn on_event(&self, window:& Window, app:& mut Application, event:&Event) -> Result<(), String> {
        let c = window.borrow_canvas();
        let canvas = &mut c.borrow_mut();
        canvas.copy(&window.borrow_rm().texture, None, None).unwrap();

        match event {
            Event::KeyDown {keycode: Some(Keycode::F),..} => { 
                //"F" -> full screen mode
                canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True)?;
            }
            Event::KeyDown {keycode: Some(Keycode::Q),..} => {
                app.remove_window(&window)?;
                //list_windows.to_remove.push(i);
            }
            Event::KeyDown {keycode: Some(Keycode::W),..} => {
                app.add_window(Box::new(TestWindowDelegate {}))?;
                //let another = Window::new2(&video)?;
                //list_windows.to_add.push(another);
            }
            Event::MouseMotion {x, y, .. } => {
                //draw a red line from the upper left corner to the current mouse position
                canvas.set_draw_color(Color::RGBA(255,0,0,255));
                canvas.draw_line(Point::new(0,0), Point::new(*x, *y)).unwrap();
                ()
            }
            _ => {
                println!("{:?}",event); //Print out other events to the "console"
                ()
            }
        }
        
        canvas.present();
        Ok(())
    }
}

impl AppDelegate for TestGameAppDelegate {
    fn init<'a>(&self, app:&'a mut Application) -> Result<(), String> {
        let x = WindowPos::from(0);
        /* 
        let window = app.add_window()?;
        window.with_delegate_mut(|delegate| *delegate = Some(Box::new(TestWindowDelegate{ app: app})));
        
        window.with_canvas(|canvas| canvas.borrow_mut().window_mut().set_position(x, x));*/
        
        let window2 = app.add_window(Box::new(TestWindowDelegate {}))?;
        let x2 = WindowPos::from(100);
        window2.with_canvas(|canvas| canvas.borrow_mut().window_mut().set_position(x2, x));
        return Ok(())
    }

    fn run(&self, app:& mut Application) -> Result<(), String> {
        
        return Ok(())
    }

}


fn main() -> Result<(), String> {

    test_file_read();
    let delegate = TestGameAppDelegate { idk:0 };
 
    let mut app = Application::new(&delegate)?;
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
