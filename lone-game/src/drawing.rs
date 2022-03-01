
use crate::geometry::Size;
use crate::geometry::Point;
use sdl2::render::RenderTarget;
use sdl2::render::Texture;
use sdl2::render::Canvas;

/*
pub struct ImmediateModeCanvas {
    size: Size<u32>,
    canvas: Canvas<RenderTarget>
}

type SDLPoint = sdl2::rect::Point;

impl ImmediateModeCanvas {
    pub fn draw_point(&mut self, point: Point<i32>) -> Result<(), String> {
        //let sdlPoint = SDLPoint::from(point);
        return self.canvas.draw_point(point);
    }
 */
    /*
    /// Draws multiple points on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawPoints")]
    pub fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<(), String> {
        let points = points.into();
        let result = unsafe {
            sys::SDL_RenderDrawPoints(
                self.context.raw,
                Point::raw_slice(points),
                points.len() as c_int,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a line on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawLine")]
    pub fn draw_line<P1: Into<Point>, P2: Into<Point>>(
        &mut self,
        start: P1,
        end: P2,
    ) -> Result<(), String> {
        let start = start.into();
        let end = end.into();
        let result = unsafe {
            sys::SDL_RenderDrawLine(self.context.raw, start.x(), start.y(), end.x(), end.y())
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a series of connected lines on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawLines")]
    pub fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<(), String> {
        let points = points.into();
        let result = unsafe {
            sys::SDL_RenderDrawLines(
                self.context.raw,
                Point::raw_slice(points),
                points.len() as c_int,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws a rectangle on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawRect")]
    pub fn draw_rect(&mut self, rect: Rect) -> Result<(), String> {
        let result = unsafe { sys::SDL_RenderDrawRect(self.context.raw, rect.raw()) };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Draws some number of rectangles on the current rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderDrawRects")]
    pub fn draw_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_RenderDrawRects(
                self.context.raw,
                Rect::raw_slice(rects),
                rects.len() as c_int,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Fills a rectangle on the current rendering target with the drawing
    /// color.
    /// Passing None will fill the entire rendering target.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderFillRect")]
    pub fn fill_rect<R: Into<Option<Rect>>>(&mut self, rect: R) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_RenderFillRect(
                self.context.raw,
                rect.into().as_ref().map(|r| r.raw()).unwrap_or(ptr::null()),
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Fills some number of rectangles on the current rendering target with
    /// the drawing color.
    /// Errors if drawing fails for any reason (e.g. driver failure)
    #[doc(alias = "SDL_RenderFillRects")]
    pub fn fill_rects(&mut self, rects: &[Rect]) -> Result<(), String> {
        let result = unsafe {
            sys::SDL_RenderFillRects(
                self.context.raw,
                Rect::raw_slice(rects),
                rects.len() as c_int,
            )
        };
        if result != 0 {
            Err(get_error())
        } else {
            Ok(())
        }
    }

    /// Copies a portion of the texture to the current rendering target.
    ///
    /// * If `src` is `None`, the entire texture is copied.
    /// * If `dst` is `None`, the texture will be stretched to fill the given
    ///   rectangle.
    ///
    /// Errors if drawing fails for any reason (e.g. driver failure),
    /// or if the provided texture does not belong to the renderer.
    #[doc(alias = "SDL_RenderCopy")]
    pub fn copy<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), String>
     */
//}