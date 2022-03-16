
use std::{cell::RefCell};

use sdl2::{render::{Texture, TextureCreator}, pixels::{PixelFormatEnum, Color}, video::WindowContext};
use crate::geometry::Size;
use ouroboros::self_referencing;

#[self_referencing]
pub struct WindowA {
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

impl WindowA {

    pub fn new_with_size(video_subsystem: &sdl2::VideoSubsystem, size: Size<u32>) -> Result<WindowA, String> {
        
        let window = video_subsystem
            .window("Lone-Game", size.width, size.height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;
        

        let w = size.width;
        let h = size.height;

        let creator = canvas.texture_creator();
        let canvas_cell = RefCell::new(canvas);

        let window_id = canvas_cell.borrow().window().id();

        let result = WindowATryBuilder {
            identifier:window_id,
            canvas: canvas_cell,
            creator: creator,
            rm_builder: |cell: &RefCell<sdl2::render::WindowCanvas>, creator: &_| ResourceManager::new(creator, cell)
        }.try_build()?;
    
        Ok(result)
    }

    pub fn new2(video_subsystem: &sdl2::VideoSubsystem) -> Result<WindowA, String> {
        let size = Size { width: 640, height: 480 };
        WindowA::new_with_size(video_subsystem, size)
    }

    pub fn id(&self) -> u32 {
        return self.borrow_identifier().clone();
    }

    //TODO: render_target_supported before providing canvas
}
