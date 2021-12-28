
mod camera;
pub use camera::Camera;

extern crate nalgebra as na;

pub trait CameraTrait {
    type Transform;
    fn transform(&self, model: &na::Matrix4<f32>) -> Self::Transform;
    fn update_view(&mut self);
    fn handle_event<T>(&mut self, event: &winit::event::Event<T>);
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct UniformBufferObject {
    transform: na::Matrix4<f32>,
}