use glam::Vec3;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

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

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: [Vertex; 3] = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        },
        Vertex {
            position: Vec3::new(0.0, 0.75, 0.0),
            color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ];

    let bytes: &[u8] = unsafe { any_as_u8_slice(&vertices) };

    let buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Tringle vertex buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    });

    buffer
}

pub fn make_quad(device: &wgpu::Device) -> Mesh {
    let vertices: [Vertex; 4] = [
        Vertex {
            position: Vec3::new(-0.75, -0.75, 0.0),
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        Vertex {
            position: Vec3::new(0.75, -0.75, 0.0),
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        },
        Vertex {
            position: Vec3::new(0.0, 0.75, 0.0),
            color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
        Vertex {
            position: Vec3::new(-0.75, 0.75, 0.0),
            color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        },
    ];
    let mut bytes: &[u8] = unsafe { any_as_u8_slice(&vertices) };

    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Quad vertex buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    });

    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
    bytes = unsafe { any_as_u8_slice(&indices) };
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Quad index buffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::INDEX,
    });

    Mesh {
        vertex_buffer,
        index_buffer,
    }
}

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
}
