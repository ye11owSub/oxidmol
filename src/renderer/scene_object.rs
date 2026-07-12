use crate::core::{Mesh, Molecule};
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
    pub fn from_molecule(device: &wgpu::Device, name: String, mol: &Molecule) -> Self {
        let sphere_instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Atom instances"),
            contents: bytemuck::cast_slice(&mol.atoms_flat),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Self {
            name,
            visible: true,
            sphere_instances,
            sphere_count: mol.atoms_flat.len() as u32,
            bond_instances: None,
            bond_count: 0,
        }
    }

    pub fn draw<'a>(&'a self, rp: &mut wgpu::RenderPass<'a>, mesh: &'a Mesh) {
        rp.set_vertex_buffer(1, self.sphere_instances.slice(..));
        rp.draw_indexed(0..mesh.index_count, 0, 0..self.sphere_count);
    }
}
