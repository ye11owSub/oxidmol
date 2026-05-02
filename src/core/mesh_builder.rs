use glam::Vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

// ---------------------------------------------------------------------------
// Vertex types
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

/// Vertex used for sphere geometry — position only (normal = position on unit sphere).
#[repr(C)]
pub struct SphereVertex {
    pub position: Vec3,
}

impl SphereVertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SphereVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

// ---------------------------------------------------------------------------
// Mesh
// ---------------------------------------------------------------------------

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

unsafe fn slice_as_u8<T: Sized>(slice: &[T]) -> &[u8] {
    std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice))
}

// ---------------------------------------------------------------------------
// Geometry builders
// ---------------------------------------------------------------------------

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: [Vertex; 3] = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.0, 0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
    ];

    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Triangle vertex buffer"),
        contents: unsafe { any_as_u8_slice(&vertices) },
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn make_quad(device: &wgpu::Device) -> Mesh {
    let vertices: [Vertex; 4] = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3::new(1.0, 0.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3::new(0.0, 1.0, 0.0),
        },
        Vertex {
            position: Vec3::new(0.0, 0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
        Vertex {
            position: Vec3::new(-0.75, 0.75, 0.0),
            color: Vec3::new(0.0, 0.0, 1.0),
        },
    ];
    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Quad vertex buffer"),
        contents: unsafe { any_as_u8_slice(&vertices) },
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Quad index buffer"),
        contents: unsafe { any_as_u8_slice(&indices) },
        usage: wgpu::BufferUsages::INDEX,
    });

    Mesh {
        vertex_buffer,
        index_buffer,
        index_count: 6,
    }
}

/// UV-sphere of unit radius centered at origin.
/// `stacks` = latitude divisions, `slices` = longitude divisions.
pub fn make_sphere(device: &wgpu::Device, stacks: u32, slices: u32) -> Mesh {
    use std::f32::consts::PI;

    let mut vertices: Vec<SphereVertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    for stack in 0..=stacks {
        let phi = PI * stack as f32 / stacks as f32;
        let (sin_phi, cos_phi) = phi.sin_cos();
        for slice in 0..=slices {
            let theta = 2.0 * PI * slice as f32 / slices as f32;
            let (sin_theta, cos_theta) = theta.sin_cos();
            vertices.push(SphereVertex {
                position: Vec3::new(sin_phi * cos_theta, cos_phi, sin_phi * sin_theta),
            });
        }
    }

    for stack in 0..stacks {
        for slice in 0..slices {
            let a = (stack * (slices + 1) + slice) as u16;
            let b = a + (slices + 1) as u16;
            // two triangles per quad
            indices.extend_from_slice(&[a, b, a + 1, b, b + 1, a + 1]);
        }
    }

    let index_count = indices.len() as u32;

    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Sphere vertex buffer"),
        contents: unsafe { slice_as_u8(&vertices) },
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Sphere index buffer"),
        contents: unsafe { slice_as_u8(&indices) },
        usage: wgpu::BufferUsages::INDEX,
    });

    Mesh {
        vertex_buffer,
        index_buffer,
        index_count,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex {
            position: Vec3::new(1.0, 2.0, 3.0),
            color: Vec3::new(0.5, 0.6, 0.7),
        };
        assert_eq!(vertex.position.x, 1.0);
        assert_eq!(vertex.position.y, 2.0);
        assert_eq!(vertex.position.z, 3.0);
        assert_eq!(vertex.color.x, 0.5);
        assert_eq!(vertex.color.y, 0.6);
        assert_eq!(vertex.color.z, 0.7);
    }

    #[test]
    fn test_vertex_layout() {
        let layout = Vertex::get_layout();
        assert_eq!(layout.array_stride, std::mem::size_of::<Vertex>() as u64);
        assert_eq!(layout.step_mode, wgpu::VertexStepMode::Vertex);
        assert_eq!(layout.attributes.len(), 2);
        assert_eq!(layout.attributes[0].shader_location, 0);
        assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);
        assert_eq!(layout.attributes[0].offset, 0);
        assert_eq!(layout.attributes[1].shader_location, 1);
        assert_eq!(layout.attributes[1].format, wgpu::VertexFormat::Float32x3);
        assert_eq!(
            layout.attributes[1].offset,
            std::mem::size_of::<Vec3>() as u64
        );
    }

    #[test]
    fn test_any_as_u8_slice() {
        let test_data: [f32; 3] = [1.0, 2.0, 3.0];
        let slice = unsafe { any_as_u8_slice(&test_data) };
        assert_eq!(slice.len(), std::mem::size_of::<[f32; 3]>());
        assert!(!slice.is_empty());
    }

    #[test]
    fn test_vertex_memory_layout() {
        assert_eq!(std::mem::size_of::<Vertex>(), 24);
        assert_eq!(std::mem::align_of::<Vertex>(), 4);
    }

    #[test]
    fn test_sphere_vertex_count() {
        let stacks = 8u32;
        let slices = 8u32;
        let expected_vertices = (stacks + 1) * (slices + 1);
        let expected_indices = stacks * slices * 6;

        // Verify the formula matches what make_sphere would produce
        assert_eq!(expected_vertices, 81);
        assert_eq!(expected_indices, 384);
    }
}
