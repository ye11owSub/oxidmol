//! Helpers for tests that need a real GPU but no window.

use std::{future::Future, sync::mpsc};

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 64;
/// Not sRGB, so byte values read back are exactly the colors that were written.
pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub fn block_on<F: Future>(fut: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("tokio runtime")
        .block_on(fut)
}

/// A device on whatever adapter the machine has (a software one is accepted too), without a surface.
///
/// Machines with no adapter (typical CI runners) make the test skip; set `OXIDMOL_REQUIRE_GPU=1`
/// to turn a missing adapter into a failure instead.
pub fn gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());

    let adapter = [false, true]
        .into_iter()
        .find_map(|force_fallback_adapter| {
            block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                force_fallback_adapter,
                ..Default::default()
            }))
            .ok()
        });

    let Some(adapter) = adapter else {
        assert!(
            std::env::var_os("OXIDMOL_REQUIRE_GPU").is_none(),
            "OXIDMOL_REQUIRE_GPU is set but no GPU adapter was found"
        );
        eprintln!("skipping: no GPU adapter available");
        return None;
    };

    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
        .expect("adapter was found, so the device must be creatable");
    Some((device, queue))
}

/// Runs `f` and panics if wgpu reported a validation error while it ran.
pub fn assert_no_validation_error<T>(device: &wgpu::Device, f: impl FnOnce() -> T) -> T {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let result = f();
    if let Some(error) = block_on(scope.pop()) {
        panic!("wgpu validation error: {error}");
    }
    result
}

pub fn target_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Test render target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}

/// Pixels of an `Rgba8` texture, row by row, without row padding.
pub struct Image {
    pub width: u32,
    pub height: u32,
    rgba: Vec<u8>,
}

impl Image {
    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * self.width + x) * 4) as usize;
        [
            self.rgba[i],
            self.rgba[i + 1],
            self.rgba[i + 2],
            self.rgba[i + 3],
        ]
    }

    pub fn count_where(&self, predicate: impl Fn([u8; 4]) -> bool) -> usize {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|&(x, y)| predicate(self.pixel(x, y)))
            .count()
    }
}

/// Copies the texture to a CPU-visible buffer and waits for the GPU to finish.
pub fn read_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
) -> Image {
    // Rows in a texture-to-buffer copy must start at multiples of 256 bytes.
    let unpadded = width * 4;
    let padded = unpadded.next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Test readback buffer"),
        size: u64::from(padded * height),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(encoder.finish()));

    let (tx, rx) = mpsc::channel();
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |r| {
        tx.send(r).expect("receiver is alive until recv returns");
    });
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("GPU poll");
    rx.recv().expect("map callback ran").expect("buffer mapped");

    let mapped = buffer.slice(..).get_mapped_range().expect("mapped range");
    let mut rgba = Vec::with_capacity((unpadded * height) as usize);
    for row in mapped.chunks(padded as usize) {
        rgba.extend_from_slice(&row[..unpadded as usize]);
    }
    drop(mapped);
    buffer.unmap();

    Image {
        width,
        height,
        rgba,
    }
}
