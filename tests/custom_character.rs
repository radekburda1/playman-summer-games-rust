use playman_rust::{
    assets::{
        artwork::{Artwork, original_sprite_rect},
        character::Face,
        custom_character as custom,
    },
    rendering::atlas,
};
#[test]
fn brother_is_selectable_and_has_fuller_body_sprites() {
    let brother = Face::BROTHER;
    assert!(brother.is_brother());
    assert_eq!(brother.next(), Face::KUBAK);
    let art = Artwork::new();
    for id in custom::BASE..custom::BASE + custom::COUNT {
        for transform in 0..8 {
            let [_, _, w, h] = custom::rect(id, transform).unwrap();
            assert!(w > 0. && h > 0. && w <= custom::CELL as f32 && h <= custom::CELL as f32);
        }
        assert!(art.sprite(id, 0, 0., 0., true).is_some());
    }
    for source in 612..616 {
        let mapped = brother.sprite_for_scene(0, source);
        let [_, _, old_width, _] = original_sprite_rect(source).unwrap();
        let [_, _, width, _] = custom::rect(mapped, 0).unwrap();
        assert!(width > old_width * 1.2 && width <= old_width * 2.0);
    }
    assert_eq!(brother.scene(), Face::default().scene());
}
#[test]
fn composed_atlas_preserves_original_pixels_and_custom_transparency() {
    let combined = atlas::image();
    let original = macroquad::prelude::Image::from_file_with_format(
        include_bytes!("../assets/atlas.png"),
        None,
    )
    .unwrap();
    assert_eq!(
        &combined.bytes[..original.bytes.len()],
        original.bytes.as_slice()
    );
    for id in (custom::BASE..custom::BASE + 6)
        .chain(custom::KUBAK..custom::KUBAK + 6)
        .chain(custom::RADEK..custom::RADEK + 6)
    {
        let [x, y, w, h] = custom::rect(id, 0).unwrap();
        let mut opaque = 0;
        let mut clear = 0;
        for yy in y as usize..(y + h) as usize {
            for xx in x as usize..(x + w) as usize {
                let alpha = combined.bytes[(yy * combined.width as usize + xx) * 4 + 3];
                if alpha > 128 {
                    opaque += 1;
                } else {
                    clear += 1;
                }
            }
        }
        assert!(opaque > 100 && clear > 0);
    }
}

#[test]
fn kubak_uses_standard_body_and_distinct_custom_portrait_and_heads() {
    let kubak = Face::KUBAK;
    assert_eq!(kubak.next(), Face::RADEK);
    assert_eq!(kubak.previous(), Face::BROTHER);
    assert_ne!(kubak.portrait(true), Face::BROTHER.portrait(true));
    let table = include_bytes!("../assets/tables.bin");
    for slot in 0..21 {
        let offset = 346 + slot * 4;
        let sprite = i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap()) as usize;
        assert_eq!(kubak.sprite_for_scene(0, sprite), sprite);
    }
    for sprite in 612..616 {
        assert_eq!(kubak.sprite_for_scene(0, sprite), sprite);
    }
    for view in 0..4 {
        assert_eq!(kubak.head_sprite(794 + view), custom::KUBAK + view);
    }
    assert_eq!(kubak.height_percent(), 100);
}

#[test]
fn torso_views_have_no_neighbor_strips_separated_by_empty_columns() {
    let image = atlas::image();
    for index in 27..31 {
        let [x, y, w, h] = custom::rect(custom::BASE + index, 0).unwrap();
        let occupied: Vec<bool> = (0..w as usize)
            .map(|column| {
                (0..h as usize).any(|row| {
                    image.bytes
                        [((y as usize + row) * image.width as usize + x as usize + column) * 4 + 3]
                        > 32
                })
            })
            .collect();
        let first = occupied.iter().position(|v| *v).unwrap();
        let last = occupied.iter().rposition(|v| *v).unwrap();
        assert!(
            occupied[first..=last].iter().all(|v| *v),
            "torso {index} has an empty column before a neighboring strip"
        );
    }
}

#[test]
fn torso_anchor_offsets_follow_rotation_and_mirroring() {
    let id = custom::BASE + 27;
    assert_eq!(custom::anchor_offset(id, 0), (6., 2.));
    assert_eq!(custom::anchor_offset(id, 4), (-6., 2.));
    assert_eq!(custom::anchor_offset(id, 1), (-2., 6.));
    assert_eq!(custom::anchor_offset(custom::KUBAK, 0), (0., 0.));
}

#[test]
fn radek_cycles_with_independent_heads_and_standard_body() {
    let face = Face::RADEK;
    assert_eq!(face.next().index(), 0);
    assert_eq!(face.previous(), Face::KUBAK);
    assert_ne!(face.portrait(true), Face::KUBAK.portrait(true));
    for sprite in 612..616 {
        assert_eq!(face.sprite_for_scene(0, sprite), sprite);
    }
    let table = include_bytes!("../assets/tables.bin");
    for slot in 0..21 {
        let offset = 346 + slot * 4;
        let sprite = i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap()) as usize;
        assert_eq!(face.sprite_for_scene(0, sprite), sprite);
    }
    for view in 0..4 {
        assert_eq!(face.head_sprite(794 + view), custom::RADEK + view);
    }
    assert_eq!(face.height_percent(), 100);
}
