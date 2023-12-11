use std::f32::consts::PI;

use bevy::prelude::{Transform, Vec2, Vec3};

pub fn calculate_rotated_bounds(
    transform: &Transform,
    half_width: f32,
    half_height: f32,
) -> [Vec2; 4] {
    let transform_matrix = transform.compute_matrix();
    let rotated_corner_1 =
        transform_matrix.transform_point3(Vec3::new(-half_width, -half_height, 0.0));
    let rotated_corner_2 =
        transform_matrix.transform_point3(Vec3::new(half_width, -half_height, 0.0));
    let rotated_corner_3 =
        transform_matrix.transform_point3(Vec3::new(half_width, half_height, 0.0));
    let rotated_corner_4 =
        transform_matrix.transform_point3(Vec3::new(-half_width, half_height, 0.0));

    [
        Vec2::new(rotated_corner_1.x, rotated_corner_1.y),
        Vec2::new(rotated_corner_2.x, rotated_corner_2.y),
        Vec2::new(rotated_corner_3.x, rotated_corner_3.y),
        Vec2::new(rotated_corner_4.x, rotated_corner_4.y),
    ]
}

pub fn point_in_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    for i in 0..n {
        let j = (i + 1) % n;

        let intersect = ((polygon[i].y > point.y) != (polygon[j].y > point.y))
            && (point.x
                < (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y)
                    / (polygon[j].y - polygon[i].y)
                    + polygon[i].x);

        if intersect {
            inside = !inside;
        }
    }

    inside
}

pub fn regular_polygon_vertices(sides: usize, radius: f32) -> Vec<Vec2> {
    let angle_increment = 2.0 * PI / sides as f32;

    (0..sides)
        .map(|i| {
            let angle = i as f32 * angle_increment;
            // NOTE:
            // x and y are flipped so that it starts from the top (a vertices will always exist at
            // the top of the polygon)
            let x = radius * angle.sin();
            let y = radius * angle.cos();
            Vec2::new(x, y)
        })
        .collect()
}
pub fn point_in_board(x: f32, y: f32, size: Vec2, center: Vec2) -> bool {
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;
    let min_x = center.x - half_width;
    let max_x = center.x + half_width;
    let min_y = center.y - half_height;
    let max_y = center.y + half_height;

    min_x <= x && x <= max_x && min_y <= y && y <= max_y
}
