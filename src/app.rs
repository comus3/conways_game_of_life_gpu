use core::num;

use winit::event::WindowEvent;

use wgpu::util::DeviceExt;

use crate::runner::Context;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

const GRID_SIZE: u32 = 4;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.8, -0.8],
    },
    Vertex {
        position: [0.8, -0.8],
    },
    Vertex {
        position: [0.8, 0.8],
    },
    Vertex {
        position: [-0.8, -0.8],
    },
    Vertex {
        position: [0.8, 0.8],
    },
    Vertex {
        position: [-0.8, 0.8],
    },
];


pub struct App {
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    num_vertices: u32,
    bind_group: wgpu::BindGroup,
}

impl App {
    pub fn new(context: &mut Context) -> Self {
        let vertex_buffer =
        context
        .device()
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
            };
        let shader = context
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });
            let render_pipeline =
            context
              .device()
              .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: None, //Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                  module: &shader,
                  entry_point: "vertexMain",
                  buffers: &[vertex_buffer_layout],
                  compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                  module: &shader,
                  entry_point: "fragmentMain",
                  targets: &[Some(wgpu::ColorTargetState {
                    format: context.config().format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                  })],
                  compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                  topology: wgpu::PrimitiveTopology::TriangleList,
                  strip_index_format: None,
                  front_face: wgpu::FrontFace::Ccw,
                  cull_mode: Some(wgpu::Face::Back),
                  // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                  polygon_mode: wgpu::PolygonMode::Fill,
                  // Requires Features::DEPTH_CLIP_CONTROL
                  unclipped_depth: false,
                  // Requires Features::CONSERVATIVE_RASTERIZATION
                  conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                  count: 1,
                  mask: !0,
                  alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
              });

        let num_vertices = VERTICES.len() as u32;

        let uniform_array = [GRID_SIZE as f32, GRID_SIZE as f32];

        let uniform_buffer =
        context
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&uniform_array),
            usage: wgpu::BufferUsages::UNIFORM,
            });

        let bind_group = context
        .device()
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &render_pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            vertex_buffer,
            render_pipeline,
            num_vertices,
            bind_group,
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        return false; // means that the event must be handeld in the start() function
    }

    pub fn update(&mut self, context: &mut Context) {}

    pub fn render(&mut self, context: &mut Context) -> Result<(), wgpu::SurfaceError> {
        let output = context.surface().get_current_texture()?;
      
        let view = output
          .texture
          .create_view(&wgpu::TextureViewDescriptor::default());
      
        let mut encoder =
          context
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
              label: Some("Render Encoder"),
            });
      
        {
          let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
              view: &view,
              resolve_target: None,
              ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                  r: 0.0,
                  g: 0.4,
                  b: 0.0,
                  a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
              },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
          });
          render_pass.set_pipeline(&self.render_pipeline);
          render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
          render_pass.set_bind_group(0, &self.bind_group, &[]);
          render_pass.draw(0..self.num_vertices, 0..GRID_SIZE*GRID_SIZE);
        }
      
        // submit will accept anything that implements IntoIter
        context.queue().submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
      }
}