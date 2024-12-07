use freetype::Library;
use gl::types::*;
use std::collections::HashMap;

pub async fn init_freetype() -> HashMap<char, Character> {
    let lib = Library::init().unwrap();
    let face = lib.new_face("./a.ttf", 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let mut characters = HashMap::new();
    let c = 'A';
    face.load_char(c as usize, freetype::face::LoadFlag::RENDER)
        .unwrap();
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            bitmap.width(),
            bitmap.rows(),
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            bitmap.buffer().as_ptr() as *const _,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        characters.insert(
            c,
            Character {
                texture_id: texture,
                size: (bitmap.width(), bitmap.rows()),
                bearing: (glyph.bitmap_left(), glyph.bitmap_top()),
                advance: glyph.advance().x as i32,
            },
        );
    }

    characters
}