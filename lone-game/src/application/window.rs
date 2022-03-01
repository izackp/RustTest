
extern crate sdl2;
use std::marker::PhantomData;

use sdl2::{render::{Texture, TextureCreator}, pixels::{PixelFormatEnum, Color}, video::WindowContext};
use crate::geometry::Size;


pub struct Window<'a> {
    pub _canvas: sdl2::render::WindowCanvas,
    pub creator: TextureCreator<WindowContext>,
    pub texture: Option<Texture<'a>>,
    _marker: PhantomData<&'a ()>
}

impl<'a> Window<'a> {
    pub fn new_with_size(video_subsystem: &sdl2::VideoSubsystem, size: Size<u32>) -> Result<Window<'a>, String> {
        
        let window = video_subsystem
            .window("Lone-Game", size.width, size.height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string())?;

        let w = size.width;
        let h = size.height;
        let texture_creator = canvas.texture_creator();
    
        //setup the background image, light grey with a blue diagonal line from upper right to lower left
        /*
        let texture:Texture = {
            let mut bg_texture = texture_creator
                .create_texture_target(PixelFormatEnum::RGBA8888, w, h)
                .map_err(|e| e.to_string())?;
        
            canvas.with_texture_canvas(&mut bg_texture, |texture_canvas| {
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
            bg_texture
        };

        Ok(Window { _canvas: canvas, creator:texture_creator, texture: texture})*/
        Ok(Window { _canvas: canvas, creator:texture_creator, texture:None, _marker:PhantomData})
    }

    pub fn addTexture(&'a mut self) -> Result<(), String> {
        let (w, h) = self._canvas.window().size();
        let mut bg_texture = self.creator
            .create_texture_target(PixelFormatEnum::RGBA8888, w, h)
            .map_err(|e| e.to_string())?;
        
        self._canvas.with_texture_canvas(&mut bg_texture, |texture_canvas| {
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
        self.texture = Some(bg_texture);
        return Ok(());
    }

    pub fn new(video_subsystem: &sdl2::VideoSubsystem) -> Result<Window, String> {
        let size = Size { width: 640, height: 480 };
        Window::new_with_size(video_subsystem, size)
    }

    //render_target_supported before providing canvas
}