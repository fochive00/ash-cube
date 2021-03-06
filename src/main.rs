
mod config;
mod app;
mod buffers;
mod pipelines;
mod helpers;
mod entities;
mod cameras;
mod fps_calculator;

use fps_calculator::FPScalculator;
use cameras::CameraTrait;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop}
};

fn main() {
    let event_loop = EventLoop::new();

    let mut application = app::App::new(&event_loop);

    let fps_calculator = Arc::new(Mutex::new(FPScalculator::new()));
    let fps_calculator_clone = Arc::clone(&fps_calculator);
    
    thread::spawn(move || {
        loop {
            let fps = fps_calculator_clone.lock().unwrap().fps();
            println!("fps: {}", fps);
    
            let one_second = time::Duration::from_secs(2);
            thread::sleep(one_second);
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        application.camera().handle_event(&event);

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
    
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = winit::event_loop::ControlFlow::Exit,
    
                    winit::event::WindowEvent::Resized(dims) => {
                        println!("resized to {:?}", dims);
                        application.update_surface_resolution(ash::vk::Extent2D {
                            width: dims.width,
                            height: dims.height,
                        });
                        application.recreate_swapchain();
                    }
    
                    _ => {}
                }
            },
            Event::RedrawEventsCleared => {
                application.render();
                fps_calculator.lock().unwrap().count_one_frame();
            }
            _ => {}
        }
    });
}
