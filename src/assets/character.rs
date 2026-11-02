//! Original character resource mappings, independent of mechanics and graphics devices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Face(u8);
impl Default for Face {
    fn default() -> Self {
        Self(1)
    }
}
impl Face {
    pub const COUNT: usize = 15;
    pub const BROTHER: Self = Self(12);
    pub const KUBAK: Self = Self(13);
    pub const RADEK: Self = Self(14);
    pub fn is_radek(self) -> bool {
        self == Self::RADEK
    }
    pub fn is_kubak(self) -> bool {
        self == Self::KUBAK
    }
    pub fn is_brother(self) -> bool {
        self.0 == 12
    }
    pub fn index(self) -> usize {
        self.0 as usize
    }
    pub fn next(self) -> Self {
        Self((self.0 + 1) % Self::COUNT as u8)
    }
    pub fn previous(self) -> Self {
        Self((self.0 + Self::COUNT as u8 - 1) % Self::COUNT as u8)
    }
    pub fn portrait(self, large: bool) -> usize {
        if self.is_radek() {
            super::custom_character::RADEK + if large { 5 } else { 4 }
        } else if self.is_brother() {
            super::custom_character::BASE + if large { 5 } else { 4 }
        } else if self.is_kubak() {
            super::custom_character::KUBAK + if large { 5 } else { 4 }
        } else {
            (if large { 53 } else { 41 }) + self.index()
        }
    }
    fn table_value(self, column: usize) -> i32 {
        let table = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tables.bin"));
        let offset = 646 + (if self.index() >= 12 { 1 } else { self.index() }) * 20 + column * 4;
        i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap())
    }
    pub fn scene(self) -> usize {
        self.table_value(0) as usize
    }
    pub fn height_percent(self) -> i32 {
        // Preserve the established default athlete/pixel fixtures. Other athletes
        // use the original character-specific vertical anchor adjustment.
        if self == Self::default() || self.index() >= 12 {
            100
        } else {
            self.table_value(2)
        }
    }
    /// Reverse the resolved default bank to its original mapping slots, then
    /// apply d.a(body, skin, head, uniform) using this character's metadata.
    pub fn sprite_for_scene(self, scene: usize, sprite: usize) -> usize {
        let table = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tables.bin"));
        let read = |offset: usize| {
            i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap()) as usize
        };
        let default_body = if scene == 0 { 346 } else { 514 };
        if let Some(slot) = (0..21).find(|&slot| read(default_body + slot * 4) == sprite) {
            if self.is_brother() {
                return super::custom_character::BASE + 6 + slot;
            }
            let selected = 262 + (self.scene() * 2 + self.table_value(1) as usize) * 84;
            return read(selected + slot * 4);
        }
        let default_head = if scene == 0 { 794 } else { 790 };
        if (default_head..default_head + 4).contains(&sprite) {
            return self
                .custom_head(sprite - default_head)
                .unwrap_or_else(|| self.table_value(3) as usize + sprite - default_head);
        }
        if self.is_brother() && (612..616).contains(&sprite) {
            return super::custom_character::BASE + 27 + sprite - 612;
        }
        sprite
    }
    // Keep custom faces visible during the original rear-facing head turns.
    fn custom_head(self, view: usize) -> Option<usize> {
        let base = if self.is_brother() {
            super::custom_character::BASE
        } else if self.is_kubak() {
            super::custom_character::KUBAK
        } else if self.is_radek() {
            super::custom_character::RADEK
        } else {
            return None;
        };
        Some(base + if view == 3 { 1 } else { view })
    }
    pub fn head_sprite(self, sprite: usize) -> usize {
        if (794..798).contains(&sprite) {
            self.custom_head(sprite - 794)
                .unwrap_or_else(|| self.table_value(3) as usize + sprite - 794)
        } else {
            sprite
        }
    }
}
/// Visual configuration stays separate from simulation and can grow independently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Appearance {
    pub face: Face,
}
