

pub mod window;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::Texture;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::convert::TryInto;
use std::marker::PhantomData;
use std::vec::Vec;
use crate::option_ext::OptionExt;

use self::window::ResourceManager;
use self::window::WindowA;

use self::window::Window2;

pub struct World {
    test: bool
}

pub struct Application {
    sdl_context: sdl2::Sdl,
    video_subsystem: Option<sdl2::VideoSubsystem>
}
/*
pub struct Loop<'a> {
    app: &'a Application,
    list_window: [Window<'a>; 1],
}*/

impl Application {
    pub fn new() -> Result<Application, String> {
        let context = sdl2::init()?;
        Ok(Application { 
            sdl_context:context, 
            video_subsystem:None
        })
    }

    //TODO: Use 'if let' when new polonius borrow checker gets implemented
    fn video(&mut self) -> Result<&VideoSubsystem, String> {
        if self.video_subsystem.is_none() {
            self.video_subsystem = Some(self.sdl_context.video()?);
        }
        let video = self.video_subsystem.as_ref().unwrap();
        return Ok(video);
        //let result = self.video_subsystem.get_or_assign_result(|| self.sdl_context.video());
        //return result;
    }
/*
    fn videoLet(&mut self) -> Result<&VideoSubsystem, String> {
        if let Some(cached) = self.video_subsystem.as_ref() {
            return Ok(cached);
        }
        let system = self.sdl_context.video()?;
        self.video_subsystem = Some(system);
        let result = self.video_subsystem.as_ref().expect("Unexpected nil");
        return Ok(result);
    } */

    fn videoNoMut(&self) -> Result<&VideoSubsystem, String> {
        if self.video_subsystem.is_some() {
            return Ok(self.video_subsystem.as_ref().unwrap())
        }
        return Err("idk".to_string());
    }

    pub fn init<'b>(&mut self) -> Result<bool, String> {
        #[cfg(target_os = "emscripten")]
        let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");
        
        if self.video_subsystem.is_none() {
            self.video_subsystem = Some(self.sdl_context.video()?);
        }
        let video = self.video_subsystem.as_ref().unwrap();
        return Ok(true);
    }

    pub fn run(&mut self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let video = &self.video()?;

        let windowa = WindowA::new(video)?; //Step 1
        let rm = ResourceManager::new(&windowa.creator, &windowa.canvas)?; //Step 2
        //windowa.rm = Some(rm);

        let window1 = Window2 {canvas: &windowa.canvas, creator: &windowa.creator, resource_manager:&rm};//Bundled
        let list_window = [window1];
        'mainloop: loop {
            let event = event_pump.wait_event(); //blocking wait for events
            
            for each_window in &list_window {
                let canvas = &mut each_window.canvas.borrow_mut();
                canvas.copy(&each_window.resource_manager.texture, None, None).unwrap();
 
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