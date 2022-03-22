

pub mod window;

use std::cell::RefCell;

use sdl2::VideoSubsystem;
use crate::defered_vec::DeferedVec;
use crate::option_ext::OptionExt;

use self::window::{Window, WindowDelegate};

pub trait AppDelegate {
    fn init(&self, app:& mut Application<'_>) -> Result<(), String>;
    fn run(&self, app:& mut Application<'_>) -> Result<(), String>;
}

pub struct WindowPair {
    window: Window,
    delegate: Box<dyn WindowDelegate> //TODO: Move into window?
}

pub struct Application<'a> {
    sdl_context: sdl2::Sdl,
    video_subsystem: Option<sdl2::VideoSubsystem>,
    list_windows: DeferedVec<WindowPair>,
    delegate: &'a dyn AppDelegate
}
/*
pub struct Loop<'a> {
    app: &'a Application,
    list_window: [Window<'a>; 1],
}*/

impl Application<'_> {
    pub fn new(delegate:&dyn AppDelegate) -> Result<Application<'_>, String> {
        let context = sdl2::init()?;
        Ok(Application { 
            sdl_context:context, 
            video_subsystem:None,
            list_windows:DeferedVec::new(),
            delegate:delegate
        })
    }

    //TODO: Use 'if let' when new polonius borrow checker gets implemented
    fn video(&mut self) -> Result<&VideoSubsystem, String> {
        let result = self.video_subsystem.get_or_assign_result(|| self.sdl_context.video());
        return result;
    }

    pub fn add_window<'a, 'b:'a>(&'b mut self, delegate:Box<dyn WindowDelegate>) -> Result<&'a mut Window, String> {
        let video = (*self.video()?).clone();
        let list_windows = &mut self.list_windows;
        let another = Window::build(&video)?;
        list_windows.to_add.push(WindowPair {window: another, delegate: delegate});
        let count = list_windows.to_add.len();
        return Ok(& mut list_windows.to_add[count - 1].window);
    }

    pub fn remove_window(&mut self, window:&Window) -> Result<(), String> {
        let list_windows = &mut self.list_windows;
        let pos = list_windows.iter().position(|it| it.window.id() == window.id());
        if pos.is_none() {
            return Err("Window doesn't exisit".to_string());
        }
        list_windows.to_remove.push(pos.unwrap());
        return Ok(())
    }

    pub fn init<'b>(&mut self) -> Result<bool, String> {
        #[cfg(target_os = "emscripten")]
        let _ = sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY","1");
        
        self.delegate.init(self)?;

        return Ok(true);
    }

    pub fn run(&mut self) -> Result<bool, String> {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'mainloop: loop {
            let event = event_pump.wait_event(); //blocking wait for events
            {
                let list_windows = &mut self.list_windows;
                list_windows.process_changes();

                if list_windows.len() == 0 {
                    break 'mainloop;
                }
            }

            for each_pair in & self.list_windows.container {
                let window_id = each_pair.window.id();
                if let Some(event_window_id) = event.get_window_id() {
                    if event_window_id == window_id { //TODO: https://rust-lang.github.io/rfcs/2497-if-let-chains.html
                        each_pair.delegate.on_event(& each_pair.window, self, &event)?;
                    }
                } else {
                    each_pair.delegate.on_event(& each_pair.window, self, &event)?;
                }
            }
        };
        return Ok(true);
    }
}