use crate::renderloop::structs::Character;
use freetype::Library;
use std::collections::HashMap;

pub async fn init_freetype() -> HashMap<char, Character> {
    let lib = Library::init().unwrap();
    let face = lib.new_face("./a.ttf", 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let mut characters = HashMap::new();
    
    // 定义需要加载的字符
    let chars_to_load: Vec<char> = (0..=9)  // 数字 0-9
        .map(|n| n.to_string().chars().next().unwrap())
        .chain('A'..='Z')  // 大写字母 A-Z
        .chain('a'..='z')  // 小写字母 a-z
        .collect();

    // 遍历加载每个字符
    for c in chars_to_load {
        // 加载字形
        face.load_char(c as usize, freetype::face::LoadFlag::RENDER)
            .unwrap_or_else(|_| panic!("Failed to load character {}", c));
            
        let glyph = face.glyph();
        let bitmap = glyph.bitmap();

        let mut texture = 0;
        unsafe {
            // 生成并配置纹理
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            // 创建纹理
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

            // 设置纹理参数
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // 存储字符信息
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
    }

    characters
}

// 添加一个辅助函数来清理资源
pub unsafe fn cleanup_characters(characters: &HashMap<char, Character>) {
    for character in characters.values() {
        gl::DeleteTextures(1, &character.texture_id);
    }
}
