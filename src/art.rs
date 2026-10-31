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
    pub fn right_center_sprite(&self, id: usize, x: f32, y: f32) {
        if let Some(Command::Image {
            source: [sx, sy, w, h],
            ..
        }) = self.artwork.sprite(id, 0, 0., 0., false)
        {
            self.draw(Command::Image {
                source: [sx, sy, w, h],
                dest: [x - w, y - (h as i32 / 2) as f32],
            });
        }
    }
    pub fn bottom_center_sprite(&self, id: usize, x: f32, y: f32) {
        if let Some(Command::Image {
            source: [sx, sy, w, h],
            ..
        }) = self.artwork.sprite(id, 0, 0., 0., false)
        {
            self.draw(Command::Image {
                source: [sx, sy, w, h],
                dest: [x - (w as i32 / 2) as f32, y - h],
            });
        }
    }
    pub fn bottom_right_sprite(&self, id: usize, x: f32, y: f32) {
        if let Some(Command::Image {
            source: [sx, sy, w, h],
            ..
        }) = self.artwork.sprite(id, 0, 0., 0., false)
        {
            self.draw(Command::Image {
                source: [sx, sy, w, h],
                dest: [x - w, y - h],
            });
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
    pub fn rounded_panel(&self, x: f32, y: f32, w: f32, h: f32) {
        draw_rectangle(x + 8., y, w - 16., 8., WHITE);
        draw_rectangle(x, y + 8., w, h - 16., WHITE);
        draw_rectangle(x + 8., y + h - 8., w - 16., 8., WHITE);
        for (a, b, c, d) in [
            (x + 8., y, x + w - 9., y),
            (x + w - 1., y + 8., x + w - 1., y + h - 9.),
            (x, y + 8., x, y + h - 9.),
            (x + 8., y + h - 1., x + w - 9., y + h - 1.),
            (x + 1., y + 8., x + 1., y + h - 9.),
            (x + 8., y + h - 2., x + w - 9., y + h - 2.),
        ] {
            draw_line(a, b, c, d, 1., BLACK);
        }
        for id in 37..=40 {
            if let Some(Command::Image {
                source: [sx, sy, cw, ch],
                ..
            }) = self.artwork.sprite(id, 0, 0., 0., false)
            {
                let dx = if id == 38 || id == 39 { x + w - cw } else { x };
                let dy = if id == 39 || id == 40 { y + h - ch } else { y };
                self.draw(Command::Image {
                    source: [sx, sy, cw, ch],
                    dest: [dx, dy],
                });
            }
        }
    }
    pub fn result_panel(&self, x: f32, y: f32, width: f32) {
        // Original panel: left cap 32, tiled center 31, right cap 33.
        for id in [32, 31, 33] {
            if let Some(Command::Image {
                source: [sx, sy, w, h],
                ..
            }) = self.artwork.sprite(id, 0, 0., 0., false)
            {
                if id == 31 {
                    let mut offset = 0.;
                    while offset < width {
                        let visible = w.min(width - offset);
                        self.draw(Command::Image {
                            source: [sx, sy, visible, h],
                            dest: [x + offset, y],
                        });
                        offset += w;
                    }
                } else {
                    self.draw(Command::Image {
                        source: [sx, sy, w, h],
                        dest: [if id == 32 { x - w } else { x + width }, y],
                    });
                }
            }
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
    pub fn animation_ground_clip(&self, animation: Animation, x: f32, y: f32, ground: f32) {
        let (a, b, f) = animation.frames(include_bytes!("../assets/tables.bin"));
        for mut command in self.artwork.frame(0, a, b, f, x, y) {
            match &mut command {
                Command::Image { source, dest } => {
                    source[3] = source[3].min(ground - dest[1]);
                    if source[3] <= 0. {
                        continue;
                    }
                }
                Command::Rect { bounds, .. } => {
                    bounds[3] = bounds[3].min(ground - bounds[1]);
                    if bounds[3] <= 0. {
                        continue;
                    }
                }
                _ => {}
            }
            self.draw(command);
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
    pub fn scene_tween(&self, id: usize, frame: usize, next: usize, fraction: i32, x: f32, y: f32) {
        for c in self.artwork.frame(id, frame, next, fraction, x, y) {
            self.draw(c)
        }
    }
}
