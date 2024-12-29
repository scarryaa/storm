use metal::MTLPixelFormat;

const SHADERS_METALLIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shaders.metallib"));

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Quad {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub color: [f32; 4],
}

pub struct MetalRenderer {
    device: metal::Device,
    pub layer: metal::MetalLayer,
    command_queue: metal::CommandQueue,
    pipeline_state: metal::RenderPipelineState,
    vertex_buffer: metal::Buffer,
}

impl MetalRenderer {
    pub fn new() -> Self {
        let layer = metal::MetalLayer::new();
        let device = metal::Device::system_default().expect("Cannot get device");
        let command_queue = device.new_command_queue();

        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_opaque(false);
        layer.set_maximum_drawable_count(3);

        let library = device
            .new_library_with_data(SHADERS_METALLIB)
            .expect("Error building metal library");

        let quads_pipeline_state = Self::build_pipeline_state(
            &device,
            &library,
            "quads",
            "quad_vertex",
            "quad_fragment",
            MTLPixelFormat::BGRA8Unorm,
        );

        let vertices: [f32; 8] = [
            0.0, 0.0, // Bottom-left
            1.0, 0.0, // Bottom-right
            0.0, 1.0, // Top-left
            1.0, 1.0, // Top-right
        ];

        let vertex_buffer = device.new_buffer_with_data(
            vertices.as_ptr() as *const _,
            (vertices.len() * std::mem::size_of::<f32>()) as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );

        MetalRenderer {
            layer,
            device,
            command_queue,
            pipeline_state: quads_pipeline_state,
            vertex_buffer,
        }
    }

    fn build_pipeline_state(
        device: &metal::DeviceRef,
        library: &metal::LibraryRef,
        label: &str,
        vertex_fn_name: &str,
        fragment_fn_name: &str,
        pixel_format: metal::MTLPixelFormat,
    ) -> metal::RenderPipelineState {
        let vertex_fn = library
            .get_function(vertex_fn_name, None)
            .expect("Error locating vertex function");
        let fragment_fn = library
            .get_function(fragment_fn_name, None)
            .expect("Error locating fragment function");

        let vertex_descriptor = metal::VertexDescriptor::new();

        let descriptor = metal::RenderPipelineDescriptor::new();
        descriptor.set_label(label);
        descriptor.set_vertex_function(Some(vertex_fn.as_ref()));
        descriptor.set_fragment_function(Some(fragment_fn.as_ref()));

        // Buffer 0: Vertices
        let position_desc = vertex_descriptor.attributes().object_at(0).unwrap();
        position_desc.set_format(metal::MTLVertexFormat::Float2);
        position_desc.set_offset(0);
        position_desc.set_buffer_index(0);

        let vertex_layout = vertex_descriptor.layouts().object_at(0).unwrap();
        vertex_layout.set_stride(8); // 2 floats * 4 bytes

        descriptor.set_vertex_descriptor(Some(vertex_descriptor));

        let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
        color_attachment.set_pixel_format(pixel_format);
        color_attachment.set_blending_enabled(true);
        color_attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
        color_attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
        color_attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
        color_attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::One);
        color_attachment
            .set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
        color_attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::One);

        device
            .new_render_pipeline_state(&descriptor)
            .expect("Could not create render pipeline state")
    }

    pub fn draw(&mut self, quads: &[Quad], viewport_size: [f32; 2]) {
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };

        // Create quad buffer
        let quad_buffer = self.device.new_buffer_with_data(
            quads.as_ptr() as *const _,
            std::mem::size_of_val(quads) as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );

        // Create viewport size buffer
        let viewport_buffer = self.device.new_buffer_with_data(
            viewport_size.as_ptr() as *const _,
            std::mem::size_of::<[f32; 2]>() as u64,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        );

        let command_buffer = self.command_queue.new_command_buffer();
        let render_pass_descriptor = metal::RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();

        color_attachment.set_texture(Some(drawable.texture()));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_store_action(metal::MTLStoreAction::Store);

        let command_encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
        command_encoder.set_render_pipeline_state(&self.pipeline_state);

        // Set buffers
        command_encoder.set_vertex_buffer(0, Some(&self.vertex_buffer), 0);
        command_encoder.set_vertex_buffer(1, Some(&quad_buffer), 0);
        command_encoder.set_vertex_buffer(2, Some(&viewport_buffer), 0);

        // Draw quads using instancing
        command_encoder.draw_primitives_instanced(
            metal::MTLPrimitiveType::TriangleStrip,
            0,
            4,                  // 4 vertices per quad
            quads.len() as u64, // Number of instances
        );

        command_encoder.end_encoding();
        command_buffer.present_drawable(drawable);
        command_buffer.commit();
    }
}

impl Default for MetalRenderer {
    fn default() -> Self {
        Self::new()
    }
}
