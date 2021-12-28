
use crate::offset_of;
use ash::vk;
use std::mem;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn vertex_input_binding_descriptions() -> Vec<vk::VertexInputBindingDescription> {
        let vertex_input_binding_descriptions = vec![
            vk::VertexInputBindingDescription {
                binding: 0,
                stride: mem::size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::VERTEX,
            }
        ];

        vertex_input_binding_descriptions
    }

    pub fn vertex_input_attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription> {
        let vertex_input_attribute_descriptions = vec![
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, color) as u32,
            },
        ];

        vertex_input_attribute_descriptions
    }
}