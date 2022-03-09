
use std::{marker::PhantomData, rc::Rc, option, cell::RefCell};

use sdl2::{render::{Texture, TextureCreator, Canvas, WindowCanvas, TextureValueError}, pixels::{PixelFormatEnum, Color}, video::WindowContext};
use crate::geometry::Size;

pub struct WindowA {
    pub canvas: RefCell<sdl2::render::WindowCanvas>,
    pub creator: TextureCreator<WindowContext>
}
/*
pub struct Window<'a> {
    pub windowA:&'a RefCell<WindowA>,
    pub resource_manager: ResourceManager<'a>
} */

pub struct Window2<'a, 'b> {
    pub canvas: &'a RefCell<sdl2::render::WindowCanvas>,
    pub creator: &'a TextureCreator<WindowContext>,
    pub resource_manager: &'b ResourceManager<'a>
}

pub struct ResourceManager<'a> {
    pub creator: &'a TextureCreator<WindowContext>,
    pub texture: Texture<'a>
}

impl<'a> ResourceManager<'a> {
    pub fn new<'b:'a>(creatorRef: &'b TextureCreator<WindowContext>, cell: &'b RefCell<sdl2::render::WindowCanvas>) -> Result<ResourceManager<'a>, String> {
        let mut b_canvas = cell.borrow_mut();
        let (w, h) = b_canvas.window().size();
        //let creator = &self.creator;

        let result = creatorRef.create_texture_target(PixelFormatEnum::RGBA8888, w, h);
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
        let manager = ResourceManager { creator: creatorRef, texture: bg_texture };
        return Ok(manager);
    }
}

//impl<'a> Window<'a> {
    /*
    pub fn new(windowA:&'a RefCell<WindowA>, texture_creator:&'a TextureCreator<WindowContext>) -> Result<Window<'a>, String> {
        //let rm = windowA.buildRM()?;
        let mut option:Option<ResourceManager<'a>> = None;
        {
            let rm = ResourceManager::new(texture_creator, windowA)?;
            option = Some(rm);
        }
        return Ok(Window { windowA: windowA, resource_manager:option.unwrap() });
    } */
//}

impl WindowA {


    pub fn new_with_size(video_subsystem: &sdl2::VideoSubsystem, size: Size<u32>) -> Result<WindowA, String> {
        
        let window = video_subsystem
            .window("Lone-Game", size.width, size.height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;

        let w = size.width;
        let h = size.height;

        let creator = canvas.texture_creator();

        let mut window = WindowA { canvas: RefCell::new(canvas), creator:creator};
        //let resource_manager:ResourceManager<'a> = ResourceManager::new(&window.creator, & mut canvas)?;
        //window.resource_manager = Some(resource_manager);
    
        Ok(window)
    }

    pub fn new(video_subsystem: &sdl2::VideoSubsystem) -> Result<WindowA, String> {
        let size = Size { width: 640, height: 480 };
        WindowA::new_with_size(video_subsystem, size)
    }

    //TODO: render_target_supported before providing canvas
}
