use glium::{Display, Program};

pub fn create_shader_program(display: &Display) -> Program {
    let vertex_shader = std::fs::read_to_string("./glsl/vertex_shader.vs").unwrap();
    let fragment_shader = std::fs::read_to_string("./glsl/fragment_shader.fs").unwrap();
    let program = Program::from_source(
        display,
        vertex_shader.as_str(),
        fragment_shader.as_str(),
        None,
    )
    .unwrap();
    return program;
}
