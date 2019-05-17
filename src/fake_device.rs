use pathfinder_geometry::basic::point::Point2DI32;
use pathfinder_geometry::basic::rect::RectI32;
use rustache::HashBuilder;
use std::time::Duration;
use pathfinder_gpu::{
    Device,
    TextureFormat,
    ShaderKind,
    VertexAttrType,
    UniformData,
    BufferData,
    BufferTarget,
    BufferUploadMode,
    ClearParams,
    Primitive,
    RenderState,
};

static mut FRAMEBUFFER_TEXTURE: Option<FakeTexture> = None;

pub struct FakeTexture {
    pub size: Point2DI32,
}

pub struct FakeDevice {
}

impl Device for FakeDevice {
    type Buffer = i64;
    type Framebuffer = i64;
    type Program = i64;
    type Shader = i64;
    type Texture = FakeTexture;
    type TimerQuery = i64;
    type Uniform = i64;
    type VertexArray = i64;
    type VertexAttr = i64;

    fn create_texture(&self, _format: TextureFormat, size: Point2DI32) -> Self::Texture {
        FakeTexture { size }
    }
    fn create_texture_from_data(&self, size: Point2DI32, _data: &[u8]) -> Self::Texture {
        println!("create_texture_from_data({}x{})", size.x(), size.y());
        FakeTexture { size }
    }
    fn create_shader_from_source(
        &self,
        name: &str,
        _source: &[u8],
        _kind: ShaderKind,
        _template_input: HashBuilder,
    ) -> Self::Shader {
        println!("create_shader_from_source({:?})", name);
        0
    }
    fn create_vertex_array(&self) -> Self::VertexArray {
        println!("create_vertex_array()");
        6
    }
    fn create_program_from_shaders(
        &self,
        name: &str,
        _vertex_shader: Self::Shader,
        _fragment_shader: Self::Shader,
    ) -> Self::Program {
        println!("create_program_from_shaders({:?})", name);
        1
    }
    fn get_vertex_attr(&self, _program: &Self::Program, name: &str) -> Self::VertexAttr {
        println!("get_vertex_attr({:?})", name);
        7
    }
    fn get_uniform(&self, _program: &Self::Program, name: &str) -> Self::Uniform {
        println!("get_uniform({:?})", name);
        2
    }
    fn use_program(&self, _program: &Self::Program) {
    }
    fn configure_float_vertex_attr(
        &self,
        _attr: &Self::VertexAttr,
        _size: usize,
        _attr_type: VertexAttrType,
        _normalized: bool,
        _stride: usize,
        _offset: usize,
        _divisor: u32,
    ) {
    }
    fn configure_int_vertex_attr(
        &self,
        _attr: &Self::VertexAttr,
        _size: usize,
        _attr_type: VertexAttrType,
        _stride: usize,
        _offset: usize,
        _divisor: u32,
    ) {
    }
    fn set_uniform(&self, _uniform: &Self::Uniform, _data: UniformData) {
    }
    fn create_framebuffer(&self, _texture: Self::Texture) -> Self::Framebuffer {
        10
    }
    fn create_buffer(&self) -> Self::Buffer {
        println!("create_buffer()");
        4
    }
    fn allocate_buffer<T>(
        &self,
        _buffer: &Self::Buffer,
        _data: BufferData<T>,
        _target: BufferTarget,
        _mode: BufferUploadMode,
    ) {
        println!("allocate_buffer()");
    }
    fn framebuffer_texture<'f>(&self, _framebuffer: &'f Self::Framebuffer) -> &'f Self::Texture {
        unsafe {
            if FRAMEBUFFER_TEXTURE.is_none() {
                    FRAMEBUFFER_TEXTURE = Some(FakeTexture {
                        size: Point2DI32::new(640, 480)
                    });
            }
            match FRAMEBUFFER_TEXTURE {
                Some(ref thing) => thing,
                None => panic!()
            }
        }
    }
    fn texture_size(&self, texture: &Self::Texture) -> Point2DI32 {
        texture.size
    }
    fn upload_to_texture(&self, _texture: &Self::Texture, _size: Point2DI32, _data: &[u8]) {
    }
    fn read_pixels_from_default_framebuffer(&self, _size: Point2DI32) -> Vec<u8> {
        unimplemented!();
    }
    fn clear(&self, _params: &ClearParams) {
    }
    fn draw_arrays(&self, _primitive: Primitive, _index_count: u32, _render_state: &RenderState) {
    }
    fn draw_elements(&self, _primitive: Primitive, _index_count: u32, _render_state: &RenderState) {
    }
    fn draw_arrays_instanced(
        &self,
        _primitive: Primitive,
        _index_count: u32,
        _instance_count: u32,
        _render_state: &RenderState,
    ) {
    }
    fn create_timer_query(&self) -> Self::TimerQuery {
        12
    }
    fn begin_timer_query(&self, _query: &Self::TimerQuery) {
    }
    fn end_timer_query(&self, _query: &Self::TimerQuery) {
    }
    fn timer_query_is_available(&self, _query: &Self::TimerQuery) -> bool {
        true
    }
    fn get_timer_query(&self, _query: &Self::TimerQuery) -> Duration {
        Duration::from_secs(0)
    }

    // TODO(pcwalton): Go bindless...
    fn bind_vertex_array(&self, _vertex_array: &Self::VertexArray) {
    }
    fn bind_buffer(&self, _buffer: &Self::Buffer, _target: BufferTarget) {
    }
    fn bind_default_framebuffer(&self, _viewport: RectI32) {
    }
    fn bind_framebuffer(&self, _framebuffer: &Self::Framebuffer) {
    }
    fn bind_texture(&self, _texture: &Self::Texture, _unit: u32) {
    }
}
