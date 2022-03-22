
use std::{cell::RefCell};

use sdl2::{render::{Texture, TextureCreator}, pixels::{PixelFormatEnum, Color}, video::WindowContext};
use sdl2::event::Event;
use crate::geometry::Size;
use crate::application::Application;
use ouroboros::self_referencing;

pub trait WindowDelegate {
    fn on_event(&self, window:& Window, app:& mut Application<'_>, event:&Event) -> Result<(), String>;
}

#[self_referencing]
pub struct Window {
    pub identifier: u32,
    pub canvas: RefCell<sdl2::render::WindowCanvas>,
    pub creator: TextureCreator<WindowContext>,
    #[borrows(canvas, creator)]#[covariant]
    pub rm: ResourceManager<'this>
}

pub struct ResourceManager<'a> {
    pub creator: &'a TextureCreator<WindowContext>,
    pub texture: Texture<'a>
}

impl<'a> ResourceManager<'a> {
    pub fn new<'b:'a>(creator_ref: &'b TextureCreator<WindowContext>, cell: &'b RefCell<sdl2::render::WindowCanvas>) -> Result<ResourceManager<'a>, String> {
        let mut b_canvas = cell.borrow_mut();
        let (w, h) = b_canvas.window().size();
        //let creator = &self.creator;

        let result = creator_ref.create_texture_target(PixelFormatEnum::RGBA8888, w, h);
        let result_mapped = result.map_err(|e| e.to_string());
        let mut bg_texture = result_mapped?;
        
        b_canvas.with_texture_canvas(&mut bg_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGBA(230,230,230,255));
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(0,0,255,255));
            { 
                let w:i32 = w.try_into().unwrap();
                let h:i32 = h.try_into().unwrap();
                texture_canvas.draw_line(sdl2::rect::Point::new(w-1,0),
                sdl2::rect::Point::new(0,h-1)).unwrap();
            }
        }).map_err(|e| e.to_string())?;
        let manager = ResourceManager { creator: creator_ref, texture: bg_texture };
        return Ok(manager);
    }
}

impl Window {

    pub fn new_with_size(video_subsystem: &sdl2::VideoSubsystem, size: Size<u32>) -> Result<Window, String> {
        
        let window = video_subsystem
            .window("Lone-Game", size.width, size.height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
        
        let creator = canvas.texture_creator();
        let canvas_cell = RefCell::new(canvas);

        let window_id = canvas_cell.borrow().window().id();

        let result = WindowTryBuilder {
            identifier:window_id,
            canvas: canvas_cell,
            creator: creator,
            rm_builder: |cell: &RefCell<sdl2::render::WindowCanvas>, creator: &_| ResourceManager::new(creator, cell)
        }.try_build()?;
    
        Ok(result)
    }

    pub fn build(video_subsystem: &sdl2::VideoSubsystem) -> Result<Window, String> {
        let size = Size { width: 640, height: 480 };
        Window::new_with_size(video_subsystem, size)
    }
    //TODO: render_target_supported before providing canvas

    pub fn id(&self) -> u32 {
        return self.borrow_identifier().clone();
    }


    pub fn window_flags(&self) -> u32 {
        return self.borrow_canvas().borrow().window().window_flags()
    }

    pub fn is_window_flag_set(&self, flag:sdl2::sys::SDL_WindowFlags) -> bool {
        let value = flag as u32;
        self.window_flags() & value == value
    }

    pub fn is_fullscreen(&self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN)
    }

    pub fn is_fullscreen_desktop(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_FULLSCREEN_DESKTOP)
    }

    pub fn is_opengl(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_OPENGL)
    }

    pub fn is_vulkan(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_VULKAN)
    }

    pub fn is_shown(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_SHOWN)
    }

    pub fn is_hidden(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_HIDDEN)
    }

    pub fn is_borderless(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_BORDERLESS)
    }

    pub fn is_resizable(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_RESIZABLE)
    }

    pub fn is_minimized(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_MINIMIZED)
    }

    pub fn is_maximized(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_MAXIMIZED)
    }

    pub fn is_input_grabbed(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_INPUT_GRABBED)
    }

    pub fn has_input_focus(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_INPUT_FOCUS)
    }

    pub fn has_mouse_focus(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_MOUSE_FOCUS)
    }

    pub fn is_foreign(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_FOREIGN)
    }

    pub fn has_mouse_capture(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_MOUSE_CAPTURE)
    }

    pub fn allows_highdpi(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI)
    }

    //X-11 Only:

    pub fn is_always_on_top(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_ALWAYS_ON_TOP)
    }

    pub fn skips_taskbar(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_SKIP_TASKBAR)
    }

    pub fn is_utility(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_UTILITY)
    }

    pub fn is_tooltip(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_TOOLTIP)
    }

    pub fn is_popup(&mut self) -> bool {
        self.is_window_flag_set(sdl2::sys::SDL_WindowFlags::SDL_WINDOW_POPUP_MENU)
    }
}