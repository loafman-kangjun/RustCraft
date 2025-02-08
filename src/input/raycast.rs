use bevy::prelude::*;

#[derive(Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn intersect_ray(&self, ray: &Ray3d, max_t: f32) -> Option<f32> {
        let epsilon = 1e-6; // 浮点容差
        let mut t_near = -f32::INFINITY;
        let mut t_far  = f32::INFINITY;

        // 对三个轴分别计算
        for i in 0..3 {
            let origin = ray.origin[i];
            let direction = ray.direction[i];
            let min_val = self.min[i];
            let max_val = self.max[i];

            if direction.abs() < epsilon {
                // 如果射线与该轴平行，检查起点是否在 slab 内
                if origin < min_val - epsilon || origin > max_val + epsilon {
                    return None;
                }
            } else {
                let inv_d = 1.0 / direction;
                let mut t1 = (min_val - origin) * inv_d;
                let mut t2 = (max_val - origin) * inv_d;
                if t1 > t2 {
                    std::mem::swap(&mut t1, &mut t2);
                }
                t_near = t_near.max(t1);
                t_far  = t_far.min(t2);
                if t_near - t_far > epsilon {
                    return None;
                }
            }
        }

        // 如果 t_far 小于零，则 AABB 完全在射线后面
        if t_far < 0.0 {
            return None;
        }
        // 如果 t_near 超出射线长度，则认为不相交
        if t_near > max_t {
            return None;
        }

        // 如果射线起点在内部，可以根据需求返回0或 t_near
        let t_hit = if t_near < 0.0 { 0.0 } else { t_near };

        Some(t_hit)
    }
}

pub fn raycast_system(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // 取主窗口和摄像机
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    let cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    // 获取当前鼠标在窗口中的位置

    // 通过 camera.viewport_to_world 生成射线
    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
        // 此处示例立方体位于原点，大小为 2.0，其 AABB 为 [(-1,-1,-1), (1,1,1)]
        let cube_aabb = Aabb {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        };

        if let Some(t_hit) = cube_aabb.intersect_ray(&ray,1000.0) {
            let _hit_point = ray.origin + ray.direction * t_hit;
            info!("射线与立方体相交");
        } else {
            info!("射线与立方体没有相交");
        }
    }
}
