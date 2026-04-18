use glam::{Mat4, Quat, Vec3};

pub fn create_view_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    Mat4::look_at_rh(eye, target, up)
}

pub fn create_projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    Mat4::perspective_rh(fov, aspect_ratio, near, far)
}

pub fn create_transform_matrix(translation: Vec3, rotation: Quat, scale: Vec3) -> Mat4 {
    Mat4::from_translation(translation) * Mat4::from_quat(rotation) * Mat4::from_scale(scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_create_view_matrix() {
        let eye = Vec3::new(0.0, 0.0, 5.0);
        let target = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        let view_matrix = create_view_matrix(eye, target, up);

        assert_ne!(view_matrix, Mat4::IDENTITY);

        let transformed_origin = view_matrix * Vec3::ZERO.extend(1.0);
        assert_abs_diff_eq!(transformed_origin.z, -5.0, epsilon = 1e-5);
    }

    #[test]
    fn test_create_projection_matrix() {
        let fov = std::f32::consts::PI / 4.0;
        let aspect_ratio = 16.0 / 9.0;
        let near = 0.1;
        let far = 100.0;

        let proj_matrix = create_projection_matrix(fov, aspect_ratio, near, far);

        assert_ne!(proj_matrix, Mat4::IDENTITY);

        assert!(proj_matrix.w_axis.z < 0.0);
    }

    #[test]
    fn test_create_transform_matrix_identity() {
        let translation = Vec3::ZERO;
        let rotation = Quat::IDENTITY;
        let scale = Vec3::ONE;

        let transform = create_transform_matrix(translation, rotation, scale);

        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_abs_diff_eq!(transform.col(i)[j], expected, epsilon = 1e-5);
            }
        }
    }

    #[test]
    fn test_create_transform_matrix_translation() {
        let translation = Vec3::new(1.0, 2.0, 3.0);
        let rotation = Quat::IDENTITY;
        let scale = Vec3::ONE;

        let transform = create_transform_matrix(translation, rotation, scale);

        assert_abs_diff_eq!(transform.w_axis.x, 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(transform.w_axis.y, 2.0, epsilon = 1e-5);
        assert_abs_diff_eq!(transform.w_axis.z, 3.0, epsilon = 1e-5);
    }

    #[test]
    fn test_create_transform_matrix_scale() {
        let translation = Vec3::ZERO;
        let rotation = Quat::IDENTITY;
        let scale = Vec3::new(2.0, 3.0, 4.0);

        let transform = create_transform_matrix(translation, rotation, scale);

        assert_abs_diff_eq!(transform.x_axis.x, 2.0, epsilon = 1e-5);
        assert_abs_diff_eq!(transform.y_axis.y, 3.0, epsilon = 1e-5);
        assert_abs_diff_eq!(transform.z_axis.z, 4.0, epsilon = 1e-5);
    }

    #[test]
    fn test_create_transform_matrix_rotation() {
        let translation = Vec3::ZERO;
        let rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        let scale = Vec3::ONE;

        let transform = create_transform_matrix(translation, rotation, scale);

        let rotated_x = transform * Vec3::X.extend(1.0);
        assert_abs_diff_eq!(rotated_x.x, 0.0, epsilon = 1e-5);
        assert_abs_diff_eq!(rotated_x.z, -1.0, epsilon = 1e-5);
    }
}
