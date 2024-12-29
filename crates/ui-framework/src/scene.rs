use crate::{
    AtlasTextureId, AtlasTile, Background, Bounds, ContentMask, DevicePixels, MonochromeSprite,
    PaintSurface, Path, PathId, PolychromeSprite, Quad, ScaledPixels, Shadow, Underline,
};

pub struct Scene {
    pub paths: Vec<Path<ScaledPixels>>,
    pub shadows: Vec<Shadow>,
    pub quads: Vec<Quad>,
    pub underlines: Vec<Underline>,
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
    pub surfaces: Vec<PaintSurface>,
}

pub enum PrimitiveBatch<'a> {
    Shadows(&'a [Shadow]),
    Quads(&'a [Quad]),
    Paths(&'a [Path<ScaledPixels>]),
    Underlines(&'a [Underline]),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [MonochromeSprite],
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        sprites: &'a [PolychromeSprite],
    },
    Surfaces(&'a [PaintSurface]),
}

impl Scene {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            shadows: Vec::new(),
            quads: Vec::new(),
            underlines: Vec::new(),
            monochrome_sprites: Vec::new(),
            polychrome_sprites: Vec::new(),
            surfaces: Vec::new(),
        }
    }

    pub fn paths(&self) -> &[Path<ScaledPixels>] {
        &self.paths
    }

    pub fn batches(&self) -> Vec<PrimitiveBatch> {
        Vec::new()
    }
}
