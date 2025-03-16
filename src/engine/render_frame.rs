use std::time::{Duration, Instant};

use wgpu::Operations;

use super::Engine;


pub struct RenderFrame<'a> {
    pub engine: &'a Engine<'a>,
    pub output: wgpu::SurfaceTexture,
    pub output_view: wgpu::TextureView,
}

pub struct RenderEncoder<'a, 'b> {
    pub frame: &'a RenderFrame<'b>,
    pub encoder: &'a mut wgpu::CommandEncoder,
}

pub struct RenderFrameResult<T> {
    pub result: T,
    pub duration: Duration,
}

impl<'a> RenderFrame<'a> {
    pub fn render<T, R, F: FnOnce(&RenderEncoder, T) -> R>(mut self, arg: T, render: F) -> RenderFrameResult<R> {
        let mut encoder = self.engine.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("RenderFrame CommandEncoder"),
            }
        );
        let render_encoder = RenderEncoder { frame: &self, encoder: &mut encoder };
        let start_time = Instant::now();
        let result = render(&render_encoder, arg);
        self.engine.queue.submit(std::iter::once(encoder.finish()));
        let duration = start_time.elapsed();
        self.output.present();
        RenderFrameResult { result, duration }
    }
}

pub struct RenderPassDescriptor<'a, 'tex> {
    pub label: Option<&'a str>,
    pub view: Option<&'a wgpu::TextureView>,
    pub resolve_target: Option<&'a wgpu::TextureView>,
    pub load: wgpu::LoadOp<wgpu::Color>,
    pub store: wgpu::StoreOp,
    pub depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'tex>>,
}

impl<'a, 'b> RenderEncoder<'a, 'b> {
    pub fn begin_render_pass(
        &'a mut self,
        descriptor: &RenderPassDescriptor,
    ) -> wgpu::RenderPass<'a> {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: descriptor.label,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: descriptor.view.unwrap_or_else(|| &self.frame.output_view),
                resolve_target: descriptor.resolve_target,
                ops: Operations {
                    load: descriptor.load,
                    store: descriptor.store,
                },
            })],
            depth_stencil_attachment: descriptor.depth_stencil_attachment.clone(),
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }
}