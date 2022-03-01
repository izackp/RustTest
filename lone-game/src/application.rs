

extern crate sdl2;
pub mod window;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::Texture;
use std::convert::TryInto;
use std::vec::Vec;
use crate::application::window::Window;
use crate::option_ext::OptionExt;

pub struct World {
    test: bool
}

pub struct Application<'a> {
    sdl_context: sdl2::Sdl,
    video_subsystem: Option<sdl2::VideoSubsystem>,
    list_window: Vec<Window<'a>>
}

impl<'a> Application<'a> {
    pub fn new() -> Result<Application<'a>, String> {
        let context = sdl2::init()?;
        Ok(Application { 
            sdl_context:context, 
            video_subsystem:None,
            list_window:Vec::new()
        })
    }

    //TODO: Use 'if let' when new polonius borrow checker gets implemented
    fn video(&mut self) -> Result<&VideoSubsystem, String> {
        let result = self.video_subsystem.get_or_assign_result(|| self.sdl_context.video());
        return result;
    }

    fn videoNoMut(&self) -> Result<&VideoSubsystem, String> {
        if self.video_subsystem.is_some() {
            return Ok(self.video_subsystem.as_ref().unwrap())
        }
        return Err("idk".to_string());
    }

    pub fn init( &'a mut self) -> Result<bool, String> {
        #[cfg(target_os = "emscripten")]
        let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");
        if self.video_subsystem.is_none() {
            self.video_subsystem = Some(self.sdl_context.video()?);
        }
        let video = self.video_subsystem.as_ref().unwrap();
        let root_window = Window::new(video)?;
        self.list_window.push(root_window);
        let test = &mut self.list_window[0];
        test.addTexture()?;
        return Ok(true);
    }

    pub fn run(&mut self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'mainloop: loop {
            let event = event_pump.wait_event(); //blocking wait for events
            
            for each_window in &mut self.list_window {
                let canvas = &mut each_window._canvas;
                canvas.copy(&each_window.texture.as_ref().unwrap(), None, None).unwrap();
 
                match event {
                    Event::KeyDown {keycode: Some(Keycode::Escape),..} | Event::Quit { .. } 
                        => { break 'mainloop; }
                    Event::KeyDown {keycode: Some(Keycode::F),..} => { 
                        //"F" -> full screen mode
                        canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True)?;
                    }
                    Event::MouseMotion {x, y, .. } => {
                        //draw a red line from the upper left corner to the current mouse position
                        canvas.set_draw_color(Color::RGBA(255,0,0,255));
                        canvas.draw_line(Point::new(0,0), Point::new(x,y)).unwrap();
                        ()
                    }
                    _ => {
                        println!("{:?}",event); //Print out other events to the "console"
                        ()
                    }
                }
                canvas.present();
            }
        };
        return Ok(true);
    }
}