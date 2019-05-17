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

pub struct FakeDevice {
}

impl Device for FakeDevice {
    type Buffer = i64;
    type Framebuffer = i64;
    type Program = i64;
    type Shader = i64;
    type Texture = i64;
    type TimerQuery = i64;
    type Uniform = i64;
    type VertexArray = i64;
    type VertexAttr = i64;

    fn create_texture(&self, _format: TextureFormat, _size: Point2DI32) -> Self::Texture {
        unimplemented!();
    }
    fn create_texture_from_data(&self, _size: Point2DI32, _data: &[u8]) -> Self::Texture {
        unimplemented!();
    }
    fn create_shader_from_source(
        &self,
        _name: &str,
        _source: &[u8],
        _kind: ShaderKind,
        _template_input: HashBuilder,
    ) -> Self::Shader {
        unimplemented!();
    }
    fn create_vertex_array(&self) -> Self::VertexArray {
        unimplemented!();
    }
    fn create_program_from_shaders(
        &self,
        _name: &str,
        _vertex_shader: Self::Shader,
        _fragment_shader: Self::Shader,
    ) -> Self::Program {
        unimplemented!();
    }
    fn get_vertex_attr(&self, _program: &Self::Program, _name: &str) -> Self::VertexAttr {
        unimplemented!();
    }
    fn get_uniform(&self, _program: &Self::Program, _name: &str) -> Self::Uniform {
        unimplemented!();
    }
    fn use_program(&self, _program: &Self::Program) {
        unimplemented!();
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
        unimplemented!();
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
        unimplemented!();
    }
    fn set_uniform(&self, _uniform: &Self::Uniform, _data: UniformData) {
        unimplemented!();
    }
    fn create_framebuffer(&self, _texture: Self::Texture) -> Self::Framebuffer {
        unimplemented!();
    }
    fn create_buffer(&self) -> Self::Buffer {
        unimplemented!();
    }
    fn allocate_buffer<T>(
        &self,
        _buffer: &Self::Buffer,
        _data: BufferData<T>,
        _target: BufferTarget,
        _mode: BufferUploadMode,
    ) {
        unimplemented!();
    }
    fn framebuffer_texture<'f>(&self, _framebuffer: &'f Self::Framebuffer) -> &'f Self::Texture {
        unimplemented!();
    }
    fn texture_size(&self, _texture: &Self::Texture) -> Point2DI32 {
        unimplemented!();
    }
    fn upload_to_texture(&self, _texture: &Self::Texture, _size: Point2DI32, _data: &[u8]) {
        unimplemented!();
    }
    fn read_pixels_from_default_framebuffer(&self, _size: Point2DI32) -> Vec<u8> {
        unimplemented!();
    }
    fn clear(&self, _params: &ClearParams) {
         unimplemented!();
    }
    fn draw_arrays(&self, _primitive: Primitive, _index_count: u32, _render_state: &RenderState) {
        unimplemented!();
    }
    fn draw_elements(&self, _primitive: Primitive, _index_count: u32, _render_state: &RenderState) {
        unimplemented!();
    }
    fn draw_arrays_instanced(
        &self,
        _primitive: Primitive,
        _index_count: u32,
        _instance_count: u32,
        _render_state: &RenderState,
    ) {
        unimplemented!();
    }
    fn create_timer_query(&self) -> Self::TimerQuery {
        unimplemented!();
    }
    fn begin_timer_query(&self, _query: &Self::TimerQuery) {
         unimplemented!();
    }
    fn end_timer_query(&self, _query: &Self::TimerQuery) {
        unimplemented!();
    }
    fn timer_query_is_available(&self, _query: &Self::TimerQuery) -> bool {
        unimplemented!();
    }
    fn get_timer_query(&self, _query: &Self::TimerQuery) -> Duration {
        unimplemented!();
    }

    // TODO(pcwalton): Go bindless...
    fn bind_vertex_array(&self, _vertex_array: &Self::VertexArray) {
        unimplemented!();
    }
    fn bind_buffer(&self, _buffer: &Self::Buffer, _target: BufferTarget) {
        unimplemented!();
    }
    fn bind_default_framebuffer(&self, _viewport: RectI32) {
        unimplemented!();
    }
    fn bind_framebuffer(&self, _framebuffer: &Self::Framebuffer) {
        unimplemented!();
    }
    fn bind_texture(&self, _texture: &Self::Texture, _unit: u32) {
        unimplemented!();
    }
}
