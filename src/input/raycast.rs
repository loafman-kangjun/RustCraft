use crate::Ground;
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
        let mut t_far = f32::INFINITY;

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
                t_far = t_far.min(t2);
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

pub fn raycast_system(windows: Query<&Window>, camera_query: Query<(&Camera, &GlobalTransform)>) {
    // 取主窗口和摄像机（这里假设只有一个窗口和一个摄像机）
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    // 这里我们用窗口中心作为射线的起始点（你可以换成鼠标位置）
    let cursor_pos = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    // 通过 camera.viewport_to_world 生成射线
    if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
        // 定义两个立方体的 AABB

        // 立方体 1：位于原点，大小为 2.0，其 AABB 为 [(-1,-1,-1), (1,1,1)]
        let cube1_aabb = Aabb {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        };

        // 立方体 2：示例中放在不同位置，比如中心为 (3, 0, 0)，大小同样为 2.0，其 AABB 为 [(2,-1,-1), (4,1,1)]
        let cube2_aabb = Aabb {
            min: Vec3::new(2.0, -1.0, -1.0),
            max: Vec3::new(4.0, 1.0, 1.0),
        };

        // 用一个 Vec 收集所有相交的立方体及其交点参数
        let mut hits = Vec::new();

        if let Some(t_hit) = cube1_aabb.intersect_ray(&ray, 1000.0) {
            hits.push(("Cube 1", t_hit));
        }
        if let Some(t_hit) = cube2_aabb.intersect_ray(&ray, 1000.0) {
            hits.push(("Cube 2", t_hit));
        }

        // 根据检测结果输出信息
        if hits.is_empty() {
            info!("射线与所有立方体都没有相交");
        } else {
            // 如果有多个立方体相交，还可以按 t_hit 排序，找到最近的相交点
            hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            for (cube_name, t_hit) in hits {
                let hit_point = ray.origin + ray.direction * t_hit;
                info!("射线与 {} 相交，交点为： {:?}", cube_name, hit_point);
            }
        }
    }
}

pub fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    let cursor_position = Vec2::new(windows.width() / 2.0, windows.height() / 2.0);

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
        ),
        0.2,
        Color::WHITE,
    );
}
