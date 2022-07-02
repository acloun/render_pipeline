#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);

pub fn create_triangle() -> Vec<Vertex> {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];
    return shape;
}
