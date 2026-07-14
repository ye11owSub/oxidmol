use crate::core::{FlatAtom, Mesh};
use wgpu::util::DeviceExt;

pub struct SceneObject {
    pub name: String,
    pub visible: bool,
    sphere_instances: wgpu::Buffer,
    sphere_count: u32,
    bond_instances: Option<wgpu::Buffer>,
    bond_count: u32,
}

impl SceneObject {
    pub fn from_bytes(device: &wgpu::Device, name: String, bytes: &[u8]) -> Self {
        let sphere_instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Atom instances"),
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        Self {
            name,
            visible: true,
            sphere_instances,
            sphere_count: (bytes.len() / std::mem::size_of::<FlatAtom>()) as u32,
            bond_instances: None,
            bond_count: 0,
        }
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>, mesh: &'a Mesh) {
        rp.set_vertex_buffer(1, self.sphere_instances.slice(..));
        rp.draw_indexed(0..mesh.index_count, 0, 0..self.sphere_count);
    }
}
