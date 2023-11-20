
// lib.rs
use winit::window::Window;
use winit:: event::*;
use wgpu::util::DeviceExt;



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}


// lib.rs


// lib.rs
impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}



// lib.rs
const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];


pub struct State
{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
    
    render_pipeline : wgpu::RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    // temp field color
    pub color: wgpu::Color,
}






// impl State
// {



impl State 
{







    // ... Other methods, like render ...
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self
    {
        let size = window.inner_size();
        let num_vertices = VERTICES.len() as u32;
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
            }
        );
        
        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: 
                    if cfg!(target_arch = "wasm32") 
                    {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    }
                    else
                    {
                        wgpu::Limits::default()
                    },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();


        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration 
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let _modes = &surface_caps.present_modes;

        // let shader = device.create_shader_module(
        //     wgpu::ShaderModuleDescriptor 
        //     {
        //         label: Some("Shader"),
        //         source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        //     }
        // );

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor 
            {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }
        );







        let pipeline1 = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor 
            {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState 
                {
                    module: &shader,
                    entry_point: "vs_main", // 1.
                    buffers: &[Vertex::desc(),], // 2.
                },
                fragment: Some(
                    wgpu::FragmentState 
                    { // 3.
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(
                            wgpu::ColorTargetState 
                            { // 4.
                                format: config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            }
                        )],
                    }
                ),

                primitive: wgpu::PrimitiveState 
                {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },

                depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState 
                {
                    count: 1, // 2.
                    mask: !0, // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
            }
        );



        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Self
        {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline : pipeline1,
            vertex_buffer,
            num_vertices,
            color: wgpu::Color::BLACK,
        }
    }


    pub fn window(&self) -> &Window
    {
        &self.window
    }

    // impl State
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    {
        if new_size.width > 0 && new_size.height > 0 
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }



// impl State
    pub fn window_input(&mut self, event: &WindowEvent) -> bool
    {
        match event 
        {
            WindowEvent::Touch(..) =>
            {
                // toggle pipeline
                self.color = wgpu::Color 
                {
                    r: 0.1,
                    g: 0.1,
                    b: 1.0,
                    a: 1.0,
                };
                true
            },
            WindowEvent::KeyboardInput 
            {
                input : KeyboardInput 
                {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                },
                ..
            } => 
            {
                // toggle pipeline
                self.color = wgpu::Color 
                {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                };
                true
            },
            _ => false,
        }
    }



    pub fn device_input(&mut self, event : &DeviceEvent) -> bool
    {
        match event 
        {
            DeviceEvent::MouseMotion { .. } =>
            {
                // toggle pipeline
                self.color = wgpu::Color 
                {
                    r: 0.1,
                    g: 0.1,
                    b: 1.0,
                    a: 1.0,
                };
                true
            },
            _ => false,
        }
    }







    pub fn update(&mut self) 
    {
        // todo!()
    }


// impl State

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> 
    {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor 
            {
                label: Some("Render Encoder"),
            }
        );
        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor 
                {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment 
                        {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations 
                            {
                                load: wgpu::LoadOp::Clear(
                                    wgpu::Color 
                                    {
                                        r: self.color.r as f64,
                                        g: self.color.g as f64,
                                        b: self.color.b as f64,
                                        a: 1.0,
                                    }   
                                    // self.color
                                ),
                                store: true,
                            },
                            }
                    )],
                    depth_stencil_attachment: None,
                }
            );

// render()
            render_pass.set_pipeline(&self.render_pipeline);
            // NEW!
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }        // submit will accept anything that implements IntoIter
        //
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }


}