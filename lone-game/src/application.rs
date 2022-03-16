

pub mod window;

use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use self::window::WindowA;


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

    pub fn init<'b>(&mut self) -> Result<bool, String> {
        #[cfg(target_os = "emscripten")]
        let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");
        
        if self.video_subsystem.is_none() {
            self.video_subsystem = Some(self.sdl_context.video()?);
        }
        return Ok(true);
    }

    pub fn run(&mut self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let video = &self.video()?;

        let window = WindowA::new2(video)?;
        let mut list_windows:Vec<WindowA> = vec![window];
        let mut windows_to_add:Vec<WindowA> = Vec::new();
        let mut windows_to_remove:Vec<u32> = Vec::new();
        'mainloop: loop {
            let event = event_pump.wait_event(); //blocking wait for events

            while let Some(window_id) = windows_to_remove.pop() {
                let index = list_windows.iter().position(|item| item.id() == window_id);
                if let Some(index) = index {
                    list_windows.swap_remove(index);
                }
            }

            while let Some(each_window) = windows_to_add.pop() {
                list_windows.push(each_window);
            }

            if list_windows.len() == 0 {
                break 'mainloop;
            }
            
            for each_window in &list_windows {
                let c = each_window.borrow_canvas();
                let canvas = &mut c.borrow_mut();
                canvas.copy(&each_window.borrow_rm().texture, None, None).unwrap();
                let window_id = each_window.id();
                
                if let Some(event_window_id) = event.get_window_id() {
                    if event_window_id == window_id { //TODO: https://rust-lang.github.io/rfcs/2497-if-let-chains.html
                        match event {
                            Event::KeyDown {keycode: Some(Keycode::F),..} => { 
                                //"F" -> full screen mode
                                canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True)?;
                            }
                            Event::KeyDown {keycode: Some(Keycode::Q),..} => {
                                windows_to_remove.push(window_id);
                            }
                            Event::KeyDown {keycode: Some(Keycode::W),..} => {
                                let another = WindowA::new2(video)?;
                                windows_to_add.push(another);
                            }
                            _ => {
                                println!("{:?}",event); //Print out other events to the "console"
                                ()
                            }
                        }
                    }
                }
                match event {
                    Event::KeyDown {keycode: Some(Keycode::Escape),..} | Event::Quit { .. } 
                        => { break 'mainloop; }
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