use macroquad::prelude::*;
use playman_rust::{
    Game,
    artwork::{Artwork, Command},
    presentation::Animation,
};
pub struct Art {
    atlas: Texture2D,
    artwork: Artwork,
}
impl Art {
    pub fn new() -> Self {
        let atlas = Texture2D::from_file_with_format(
            include_bytes!("../assets/atlas.png"),
            Some(ImageFormat::Png),
        );
        atlas.set_filter(FilterMode::Nearest);
        Self {
            atlas,
            artwork: Artwork::new(),
        }
    }
    fn draw(&self, command: Command) {
        match command {
            Command::Hook { .. } => {}
            Command::Image {
                source: [x, y, w, h],
                dest: [dx, dy],
            } => draw_texture_ex(
                &self.atlas,
                dx,
                dy,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(x, y, w, h)),
                    ..Default::default()
                },
            ),
            Command::Rect {
                bounds: [x, y, w, h],
                rgb,
            } => draw_rectangle(
                x,
                y,
                w,
                h,
                Color::from_rgba((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8, 255),
            ),
        }
    }
    pub fn sprite(&self, id: usize, x: f32, y: f32, center: bool) {
        if let Some(c) = self.artwork.sprite(id, 0, x, y, center) {
            self.draw(c)
        }
    }
    pub fn scaled_sprite(&self, id: usize, x: f32, y: f32, scale: f32) {
        if let Some(Command::Image {
            source: [sx, sy, w, h],
            ..
        }) = self.artwork.sprite(id, 0, 0., 0., false)
        {
            draw_texture_ex(
                &self.atlas,
                x - w * scale / 2.,
                y - h * scale / 2.,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(sx, sy, w, h)),
                    dest_size: Some(vec2(w * scale, h * scale)),
                    ..Default::default()
                },
            )
        }
    }
    pub fn label(&self, font: usize, text: &str, mut x: f32, y: f32, align: u8) {
        let font = playman_rust::bitmap_font::BitmapFont::get(font);
        let width = font.width(text) as f32;
        if align == 1 {
            x -= width / 2.
        } else if align == 2 {
            x -= width;
        }
        for c in text.chars() {
            let g = font.glyph(c);
            if g.sprite >= 0 {
                self.sprite(g.sprite as usize, x + g.x as f32, y + g.y as f32, false)
            }
            x += g.advance as f32;
        }
    }
    pub fn athlete(&self, g: &Game, x: f32, y: f32) {
        self.animation(Animation::for_game(g), x, y);
    }
    pub fn animation(&self, animation: Animation, x: f32, y: f32) {
        let (a, b, f) = animation.frames(include_bytes!("../assets/tables.bin"));
        for c in self.artwork.frame(0, a, b, f, x, y) {
            self.draw(c)
        }
    }
    pub fn athlete_hook(&self, g: &Game, x: f32, y: f32) -> Option<(Vec2, Vec2)> {
        let (a, b, f) = Animation::for_game(g).frames(include_bytes!("../assets/tables.bin"));
        self.artwork
            .frame(0, a, b, f, x, y)
            .into_iter()
            .find_map(|command| {
                if let Command::Hook {
                    bounds: [px, py, dx, dy],
                } = command
                {
                    let center = vec2(px, py + dy - 1.);
                    let direction = vec2(dx, -dy);
                    if (center.x - x).abs() < 200.
                        && (center.y - y).abs() < 200.
                        && direction.length_squared() > 0.
                    {
                        Some((center, direction.normalize()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
    }
    pub fn scene_width(&self, id: usize) -> i32 {
        self.artwork.width(id)
    }
    pub fn scene(&self, id: usize, frame: usize, x: f32, y: f32) {
        for c in self.artwork.frame(id, frame, frame, 0, x, y) {
            self.draw(c)
        }
    }
}
