#![allow(unused_imports)]
#![allow(dead_code)]

use std::path::Path;
use std::collections::HashMap;
use std::cell::RefCell;

use sdl2::rect::{Rect};
use sdl2::surface::{Surface};
use sdl2::render::{Texture, Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::image::{LoadSurface};
use sdl2::pixels::{PixelFormatEnum};

use crate::robot::RobotId;


pub struct DrawContext<'d> {
    pub canvas: &'d mut Canvas<Window>,
    pub tm: RefCell<TextureManager<'d>>,
}


pub struct TextureManager<'t> {
    creator: &'t TextureCreator<WindowContext>,
    surfaces: Vec<Surface<'t>>,
    textures: Vec<Texture<'t>>,
    sprites: HashMap<SpriteId, Sprite>,
}


#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    // Board management
    BoardCell,
    SizedBoard { width: u32, height: u32 },
    DefaultBoard,
    //
    Robot(RobotId),
}


#[derive(Clone)]
pub struct Sprite {
    pub texture_index: usize,
    pub geom: Rect
}


impl<'d> DrawContext<'d> {
    pub fn new(
        canvas: &'d mut Canvas<Window>,
        creator: &'d TextureCreator<WindowContext>,
        ) -> DrawContext<'d> {
        DrawContext {
            canvas,
            tm: RefCell::new(TextureManager::new(creator)),
        }
    }

    pub fn draw(&mut self, id: &SpriteId, area: Rect) -> Result<(), String> {
        let tm = self.tm.borrow();
        let sprite = tm.get_sprite(id)?;
        let texture = tm.get_texture(sprite)?;
        self.canvas.copy(texture, sprite.geom, area)
    }

    pub fn create_texture<F, D>(
        &mut self, 
        id: SpriteId,
        format: F, 
        width: u32, 
        height: u32,
        draw: D
        ) 
        -> Result<Sprite, String> 
        where F: Into<Option<PixelFormatEnum>>,
              D: FnOnce(&mut Canvas<Window>, &TextureManager) -> Result<(), String>,
    {
        let mut tm = self.tm.borrow_mut();
        
        let mut texture = tm.create_texture(format, width, height)?;
        
        let mut draw_result = Ok(());
        self.canvas.with_texture_canvas(
            &mut texture,
            |texture_canvas| { 
                draw_result = draw(texture_canvas, &tm);
            })
            .map_err(|err| format!("{:?}", err))
            .and(draw_result)
            .map(|_| tm.add_sprite_from_texture(texture, id))
    }

}


impl<'t> TextureManager<'t> {
    pub fn new(
        creator: &'t TextureCreator<WindowContext>,
        ) -> TextureManager<'t> {
        TextureManager {
            creator,
            surfaces: Vec::new(),
            textures: Vec::new(),
            sprites: HashMap::new(),
        }
    }

    pub fn load_static(&mut self) -> Result<(), String> {
        self.surfaces = vec![
            Surface::from_file(&Path::new("assets/all.svg"))?,
        ];
        
        self.textures = vec![
            self.creator.create_texture_from_surface(&self.surfaces[0])
                .map_err(|e| format!("{:?}", e))?,
        ];

        let side = self.textures[0].query().height;
        
        self.sprites.insert(
            SpriteId::BoardCell,
            Sprite { texture_index: 0, geom: Rect::new(0, 0, side, side) });
        
        // Robots
        self.sprites.insert(
            SpriteId::Robot(RobotId::Blue),
            Sprite { texture_index: 0, geom: Rect::new(1 * side as i32, 0, side, side) });
        self.sprites.insert(
            SpriteId::Robot(RobotId::Green),
            Sprite { texture_index: 0, geom: Rect::new(2 * side as i32, 0, side, side) });
        self.sprites.insert(
            SpriteId::Robot(RobotId::Yellow),
            Sprite { texture_index: 0, geom: Rect::new(3 * side as i32, 0, side, side) });
        self.sprites.insert(
            SpriteId::Robot(RobotId::Red),
            Sprite { texture_index: 0, geom: Rect::new(4 * side as i32, 0, side, side) });
        
        Ok(())
    }

    // Texture management below
    
    pub fn add_texture(&mut self, texture: Texture<'t>) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn get_texture(&self, sprite: &Sprite) -> Result<&Texture<'t>, String> {
        self.textures.get(sprite.texture_index)
            .ok_or_else(|| format!("missing texture"))
    }

    pub fn create_texture<F>(&mut self, format: F, width: u32, height: u32) 
        -> Result<Texture<'t>, String> 
        where F: Into<Option<PixelFormatEnum>>
    {
        self.creator
            .create_texture_target(format, width, height)
            .map_err(|err| format!("{:?}", err))
    }

    // Sprite management
    
    pub fn get_sprite(&self, id: &SpriteId) -> Result<&Sprite, String> {
        self.sprites.get(id)
            .ok_or_else(|| format!("missing sprite"))
    }

    pub fn sprite_exists(&self, id: &SpriteId) -> bool {
        self.sprites.contains_key(id)
    }
    
    pub fn set_sprite(&mut self, id: SpriteId, sprite: Sprite) {
        self.sprites.insert(id, sprite);
    }
    
    pub fn add_sprite_from_texture(&mut self, texture: Texture<'t>, id: SpriteId) -> Sprite {
        let info = texture.query();
        let geom = Rect::new(0, 0, info.width, info.height);
        
        let texture_index = self.add_texture(texture);
        
        let sprite = Sprite { texture_index, geom };
        self.sprites.insert(id, sprite.clone());
        
        sprite
    }

}
