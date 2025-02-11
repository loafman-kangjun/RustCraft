use image::{ImageBuffer, Luma};
use noise::{NoiseFn, Perlin};

pub fn gn() {
    let width = 256;
    let height = 256;

    // 1. 生成 Perlin 噪声
    let perlin = Perlin::new(81565128); // 使用相同的种子值
    let mut noise_map = vec![vec![0.0; height]; width];

    for x in 0..width {
        for y in 0..height {
            let value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);
            noise_map[x][y] = value;
        }
    }

    // 2. 应用高斯模糊
    let mut blurred_map = vec![vec![0.0; height]; width]; // 注意类型匹配
    let kernel = [[1.0, 2.0, 1.0], [2.0, 4.0, 2.0], [1.0, 2.0, 1.0]];
    let kernel_sum = 16.0;

    for x in 1..(width as u32 - 1) {
        for y in 1..(height as u32 - 1) {
            let mut sum = 0.0;
            for i in 0..3 {
                for j in 0..3 {
                    sum += noise_map[x as usize - 1 + i][y as usize - 1 + j] * kernel[i][j];
                }
            }
            blurred_map[x as usize][y as usize] = sum / kernel_sum;
        }
    }


    // 3. 保存为图像
    let mut img = ImageBuffer::new(width as u32, height as u32);
    for x in 0..width as u32 {
        for y in 0..height as u32 {
            let value = ((blurred_map[x as usize][y as usize] + 1.0) / 3.0 * 255.0) as u8; // 映射到 0-255
            img.put_pixel(x, y, Luma([value]));
        }
    }

    img.save("blurred_noise.png").unwrap();
}