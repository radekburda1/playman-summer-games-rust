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
    let image = atlas::image();
    for source in 612..616 {
        let mapped = brother.sprite_for_scene(0, source);
        let (shoulder, hip) = custom::torso_joints(source - 612);
        let (dx, dy) = (hip.0 - shoulder.0, hip.1 - shoulder.1);
        let thickness = |[x, y, w, h]: [f32; 4]| {
            let (mut low, mut high) = (f32::INFINITY, f32::NEG_INFINITY);
            for yy in 0..h as usize {
                for xx in 0..w as usize {
                    let p = ((y as usize + yy) * image.width as usize + x as usize + xx) * 4;
                    if image.bytes[p + 3] > 32 {
                        let across = xx as f32 * dy - yy as f32 * dx;
                        low = low.min(across);
                        high = high.max(across);
                    }
                }
            }
            high - low
        };
        let old = thickness(original_sprite_rect(source).unwrap());
        let new = thickness(custom::rect(mapped, 0).unwrap());
        assert!(
            new > old * 1.2 && new <= old * 3.2,
            "torso {source}: thickness {new}, original {old}"
        );
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
        .chain(custom::BURDAS..custom::BURDAS + 6)
        .chain(custom::TEREZA..custom::TEREZA + 6)
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
        assert_eq!(
            kubak.head_sprite(794 + view),
            custom::KUBAK + [0, 1, 2, 1][view]
        );
    }
    assert_eq!(kubak.height_percent(), 100);
}

#[test]
fn torso_views_have_no_neighbor_strips_separated_by_empty_columns() {
    let image = atlas::image();
    for index in 27..31 {
        for transform in 0..8 {
            let [x, y, w, h] = custom::rect(custom::BASE + index, transform).unwrap();
            for row in 0..h as usize {
                for column in 0..w as usize {
                    if row == 0 || column == 0 || row + 1 == h as usize || column + 1 == w as usize
                    {
                        let p =
                            ((y as usize + row) * image.width as usize + x as usize + column) * 4;
                        assert!(
                            image.bytes[p + 3] <= 32,
                            "torso {index}/{transform} clips its cell"
                        );
                    }
                }
            }
        }
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
fn torso_registration_preserves_both_attachment_points() {
    for slot in 0..4 {
        let (shoulder, hip) = custom::torso_joints(slot);
        let (w, h) = (400., 560.);
        let actual_shoulder = custom::torso_sample(slot, shoulder.0, shoulder.1, w, h);
        let actual_hip = custom::torso_sample(slot, hip.0, hip.1, w, h);
        assert!((actual_shoulder.0 - w * 0.18).abs() < 0.001);
        assert!((actual_shoulder.1 - h * 0.17).abs() < 0.001);
        assert!((actual_hip.0 - w * 0.47).abs() < 0.001);
        assert!((actual_hip.1 - h * 0.94).abs() < 0.001);
        for transform in 0..8 {
            assert_eq!(
                custom::anchor_offset(custom::BASE + 27 + slot, transform),
                (0., 0.)
            );
        }
    }
}

#[test]
fn radek_cycles_with_independent_heads_and_standard_body() {
    let face = Face::RADEK;
    assert_eq!(face.next(), Face::BURDAS);
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
        assert_eq!(
            face.head_sprite(794 + view),
            custom::RADEK + [0, 1, 2, 1][view]
        );
    }
    assert_eq!(face.height_percent(), 100);
}

#[test]
fn bent_torso_slots_rotate_the_waist_toward_the_hips() {
    let image = atlas::image();
    // Original 612..615 are successive lean angles, not front/rear views.
    // A front/rear replacement leaves the dark waistband below the shoulders
    // even in the nearly horizontal start/dive slot.
    for (index, horizontal) in [(27, false), (30, true)] {
        let [x, y, w, h] = custom::rect(custom::BASE + index, 0).unwrap();
        let (mut count, mut sum_x, mut sum_y) = (0., 0., 0.);
        for yy in 0..h as usize {
            for xx in 0..w as usize {
                let p = ((y as usize + yy) * image.width as usize + x as usize + xx) * 4;
                let [r, g, b, a]: [u8; 4] = image.bytes[p..p + 4].try_into().unwrap();
                if a > 128 && (25..90).contains(&r) && r.abs_diff(g) < 8 && g.abs_diff(b) < 8 {
                    count += 1.;
                    sum_x += xx as f32;
                    sum_y += yy as f32;
                }
            }
        }
        // A thin hem can reduce to only a few gray pixels at these lean angles.
        assert!(count > 0., "missing waistband in slot {index}");
        if horizontal {
            assert!(
                sum_x / count < w * 0.5 - 8.,
                "waist must face left in bent torso"
            );
        } else {
            assert!(
                sum_y / count > h * 0.5 + 10.,
                "waist must face down in upright torso"
            );
        }
    }
}

#[test]
fn torso_hem_ends_at_the_leg_attachment_in_each_lean_pose() {
    let image = atlas::image();
    for slot in 0..4 {
        let (shoulder, hip) = custom::torso_joints(slot);
        let (dx, dy) = (hip.0 - shoulder.0, hip.1 - shoulder.1);
        let length_squared = dx * dx + dy * dy;
        let [x, y, w, h] = custom::rect(custom::BASE + 27 + slot, 0).unwrap();
        for row in 0..h as usize {
            for column in 0..w as usize {
                let p = ((y as usize + row) * image.width as usize + x as usize + column) * 4;
                if image.bytes[p + 3] > 32 {
                    let px = column as f32 + 0.5 - w / 2. - shoulder.0;
                    let py = row as f32 + 0.5 - h / 2. - shoulder.1;
                    let along = (px * dx + py * dy) / length_squared;
                    assert!(
                        along <= 1.15,
                        "torso {slot} hangs below the hip toward the knees: {along}"
                    );
                }
            }
        }
    }
}

#[test]
fn torso_shoulder_socket_leaves_room_for_the_arm_joint() {
    let image = atlas::image();
    for slot in 0..4 {
        let (shoulder, _) = custom::torso_joints(slot);
        let [x, y, w, h] = custom::rect(custom::BASE + 27 + slot, 0).unwrap();
        let px = (x + w / 2. + shoulder.0) as usize;
        let py = (y + h / 2. + shoulder.1) as usize;
        assert!(
            image.bytes[(py * image.width as usize + px) * 4 + 3] <= 32,
            "torso {slot} puts fabric across the arm socket center"
        );
    }
}

#[test]
fn custom_head_turns_keep_faces_visible_through_all_animation_poses() {
    let art = Artwork::new();
    for (face, base) in [
        (Face::BROTHER, custom::BASE),
        (Face::KUBAK, custom::KUBAK),
        (Face::RADEK, custom::RADEK),
        (Face::BURDAS, custom::BURDAS),
        (Face::TEREZA, custom::TEREZA),
    ] {
        assert_eq!(face.head_sprite(797), base + 1);
        assert_eq!(face.sprite_for_scene(0, 797), base + 1);
        assert_eq!(face.sprite_for_scene(1, 793), base + 1);
        for pose in 0..318 {
            for command in art.frame_with_face(0, pose, pose, 0, 120., 230., face) {
                if let playman_rust::assets::artwork::Command::Image { source, .. } = command {
                    for transform in 0..8 {
                        assert_ne!(
                            Some(source),
                            custom::rect(base + 3, transform),
                            "rear head in {face:?} pose {pose}"
                        );
                    }
                }
            }
        }
    }
    assert_eq!(Face::default().head_sprite(797), 797);
}

#[test]
fn brother_shorts_keep_original_width_in_bent_leg_poses() {
    for slot in [16, 23, 24, 25, 26] {
        let [_, _, w, h] = original_sprite_rect(custom::original_sprite(slot)).unwrap();
        assert_eq!(custom::dimensions(slot), (w as usize, h as usize));
    }
    let [_, _, arm_width, _] = original_sprite_rect(custom::original_sprite(8)).unwrap();
    assert!(custom::dimensions(8).0 as f32 > arm_width);
}

#[test]
fn burdas_cycles_with_transparent_portraits_and_standard_body() {
    let face = Face::BURDAS;
    assert!(face.is_burdas());
    assert_eq!(face.previous(), Face::RADEK);
    assert_eq!(face.next(), Face::TEREZA);
    assert_eq!(face.portrait(false), custom::BURDAS + 4);
    assert_eq!(face.portrait(true), custom::BURDAS + 5);
    assert_ne!(face.portrait(true), Face::RADEK.portrait(true));
    for sprite in 612..616 {
        assert_eq!(face.sprite_for_scene(0, sprite), sprite);
    }
    for slot in 6..27 {
        let original = custom::original_sprite(slot);
        assert_eq!(face.sprite_for_scene(0, original), original);
    }
    for (view, mapped) in [0, 1, 2, 1].into_iter().enumerate() {
        assert_eq!(face.head_sprite(794 + view), custom::BURDAS + mapped);
    }
    assert_eq!(face.height_percent(), 100);
}

#[test]
fn tereza_uses_the_original_female_bank_and_compact_head_views() {
    let face = Face::TEREZA;
    assert!(face.is_tereza());
    assert_eq!(face.previous(), Face::BURDAS);
    assert_eq!(face.next().index(), 0);
    assert_eq!(face.scene(), 1);
    assert_eq!(face.height_percent(), 100);
    assert_eq!(face.portrait(false), custom::TEREZA + 4);
    assert_eq!(face.portrait(true), custom::TEREZA + 5);
    let table = include_bytes!("../assets/tables.bin");
    for slot in 0..21 {
        let offset = 514 + slot * 4;
        let source = i32::from_be_bytes(table[offset..offset + 4].try_into().unwrap()) as usize;
        assert_eq!(face.sprite_for_scene(1, source), source);
    }
    for source in 624..628 {
        assert_eq!(face.sprite_for_scene(1, source), source);
    }
    for (view, mapped) in [0, 1, 2, 1].into_iter().enumerate() {
        assert_eq!(
            face.sprite_for_scene(1, 790 + view),
            custom::TEREZA + mapped
        );
    }
    for view in 0..4 {
        let (w, h) = custom::dimensions(custom::TEREZA - custom::BASE + view);
        assert!(
            w <= 40 && h <= 38,
            "hair must stay in the original female head envelope"
        );
    }
}

#[test]
fn custom_heads_reveal_the_existing_body_neck_through_transforms() {
    let offsets = [
        (0., -3.),
        (3., 0.),
        (0., 3.),
        (-3., 0.),
        (0., -3.),
        (0., 3.),
        (-3., 0.),
        (3., 0.),
    ];
    let art = Artwork::new();
    for id in [
        custom::BASE,
        custom::KUBAK,
        custom::RADEK,
        custom::BURDAS,
        custom::TEREZA,
    ]
    .into_iter()
    .flat_map(|base| base..base + 4)
    {
        for (transform, expected) in offsets.into_iter().enumerate() {
            assert_eq!(custom::anchor_offset(id, transform as u8), expected);
            if let Some(playman_rust::assets::artwork::Command::Image { source, dest }) =
                art.sprite(id, transform as u8, 100., 100., true)
            {
                assert_eq!(
                    dest,
                    [
                        100. - (source[2] as i32 / 2) as f32 + expected.0,
                        100. - (source[3] as i32 / 2) as f32 + expected.1
                    ]
                );
            } else {
                panic!("missing custom head {id}");
            }
        }
    }
    for id in [
        custom::BASE,
        custom::KUBAK,
        custom::RADEK,
        custom::BURDAS,
        custom::TEREZA,
    ]
    .into_iter()
    .flat_map(|base| base + 4..base + 6)
    {
        for transform in 0..8 {
            assert_eq!(custom::anchor_offset(id, transform), (0., 0.));
        }
    }
}
