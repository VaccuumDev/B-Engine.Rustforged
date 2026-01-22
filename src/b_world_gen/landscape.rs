/*=========================================|
                                           |
        DO NOT INCLUDE THIS FILE           |
                                           |
===========================================|

    this file contains EXPERIMENTAL and UNSTABLE GPU-computations code with compute shader. Including this file may damage your GPU

*/
use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::texture::ImageSampler;
use bytemuck::{Pod, Zeroable};
use std::num::NonZeroU32;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Params {
    width: u32,
    height: u32,
    octaves: u32,
    seed: u32,
    lacunarity: f32,
    gain: f32,
    scale: f32,
    _pad: f32,
}

fn setup(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut pipelines: ResMut<Assets<ComputePipelineDescriptor>>,
) {
    unimplemented!();

    let width = 128u32;
    let height = 128u32;
    let params = Params {
        width,
        height,
        octaves: 5,
        seed: 12345,
        lacunarity: 2.0,
        gain: 0.5,
        scale: 8.0,
        _pad: 0.0,
    };

    let shader_src = include_str!("../assets/shaders/plasma.wgsl");
    let shader_module = render_device.create_shader_module(ShaderModuleDescriptor {
        label: Some("plasma"),
        source: ShaderSource::Wgsl(shader_src.into()),
    });

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let texture = render_device.create_texture(&TextureDescriptor {
        label: Some("plasma_texture"),
        size,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        mip_level_count: 1,
        sample_count: 1,
        usage: TextureUsages::STORAGE_BINDING
            | TextureUsages::COPY_SRC
            | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&TextureViewDescriptor::default());

    let params_buf = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("plasma_params"),
        contents: bytemuck::bytes_of(&params),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("plasma_bgl"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(std::mem::size_of::<Params>() as u64),
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::StorageTexture {
                    access: StorageTextureAccess::WriteOnly,
                    format: TextureFormat::Rgba8Unorm,
                    view_dimension: TextureViewDimension::D2,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("plasma_pl"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = render_device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("plasma_cp"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "main",
    });

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("plasma_bg"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: params_buf.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&texture_view),
            },
        ],
    });

    let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("plasma_enc"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("plasma_pass"),
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        let groups_x = ((width + 7) / 8) as u32;
        let groups_y = ((height + 7) / 8) as u32;
        compute_pass.dispatch_workgroups(groups_x, groups_y, 1);
    }

    render_queue.submit(Some(encoder.finish()));

    let bytes_per_texel = 4u32; // rgba8
    let padded_bytes_per_row = ((width * bytes_per_texel + 255) / 256) * 256;
    let buffer_size = padded_bytes_per_row as u64 * height as u64;

    let read_buf = render_device.create_buffer(&BufferDescriptor {
        label: Some("read_buf"),
        size: buffer_size,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder2 = render_device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("copy_enc"),
    });
    encoder2.copy_texture_to_buffer(
        ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        ImageCopyBuffer {
            buffer: &read_buf,
            layout: ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(padded_bytes_per_row),
                rows_per_image: NonZeroU32::new(height),
            },
        },
        size,
    );
    render_queue.submit(Some(encoder2.finish()));

    let buffer_slice = read_buf.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |res| {
        sender.send(res).unwrap();
    });
    render_device.poll(wgpu::Maintain::Wait);
    futures_lite::future::block_on(async {
        receiver.receive().await;
    });

    let data = buffer_slice.get_mapped_range().to_vec();
    read_buf.unmap();

    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        let row_start = (y as u64 * padded_bytes_per_row as u64) as usize;
        let row = &data[row_start..row_start + (width * bytes_per_texel) as usize];
        pixels.extend_from_slice(row);
    }

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8Unorm,
    );
    image.sampler_descriptor = ImageSampler::nearest(); // no filtering for heightmap
    let handle = images.add(image);

    info!(
        "Plasma generated and uploaded to Bevy Image handle: {:?}",
        handle
    );
}
