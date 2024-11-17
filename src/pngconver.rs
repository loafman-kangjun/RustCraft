use glium::texture::RawImage2d;

pub fn image() -> RawImage2d<'static, u8> {
    let image = image::load(
        std::io::Cursor::new(&include_bytes!("./opengl.png")),
        image::ImageFormat::Png,
    ).unwrap()
        .to_rgba8();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    return image;
}