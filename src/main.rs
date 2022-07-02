use glium::glutin::{event, event_loop, window, ContextBuilder};
use glium::{index, uniform, Display, Surface, VertexBuffer};
use std::time::{Duration, Instant};

mod shader;
mod vertex;
mod texture;

fn main() {
    let event_loop = event_loop::EventLoop::new();
    let wb = window::WindowBuilder::new();
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    let program = shader::create_shader_program(&display);
    let shape = vertex::create_triangle();

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let mut t: f32 = -0.5;

    event_loop.run(move |event, _, control_flow| {
        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::CloseRequested => {
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            event::Event::NewEvents(cause) => match cause {
                event::StartCause::ResumeTimeReached { .. } => (),
                event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);

        // we update `t`
        t += 0.002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                // &glium::uniforms::EmptyUniforms,
                &uniform! {
                    matrix: [
                    [1.0,0.0,0.0,0.0],
                    [0.0,1.0,0.0,0.0],
                    [0.0,0.0,1.0,0.0],
                    [t,0.0,0.0,1.0f32],
                ]},
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        #[derive(Debug)]
        struct a {
            aa: i8,
        }
        impl Default for a {
            fn default() -> Self {
                a { aa: 10 }
            }
        }
        let c: a = Default::default();
        println!("{:?}", c);
    }
}
