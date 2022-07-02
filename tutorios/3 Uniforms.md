动画我们的三角形
Animating our triangle

现在我们有了一个三角形，我们将尝试对其进行动画。 请记住，OpenGL就像绘图软件。 如果我们想在屏幕上进行更改，我们必须绘制现有内容以替换已经存在的内容。 幸运的是，我们已经有一个循环连续绘制在窗口上，因此我们的更改几乎会立即反映在窗口上。
Now that we have a triangle, we are going to try animating it. Remember that OpenGL is like a drawing software. If we want to make a change on the screen, we have to draw over the existing content to replace what is already there. Fortunately we already have a loop that continuously draws on the window, so our changes will almost instantly be reflected on the window.

天真的方法
The naive approach
我们的第一种方法是创建一个名为t的变量，该变量代表动画中的步骤。 我们在每个循环处更新T的值，并将其添加到每个帧的三角形的坐标中：
Our first approach will be to create a variable named t which represents the step in the animation. We update the value of t at each loop, and add it to the coordinates of our triangle at each frame:

```rust
let mut t: f32 = -0.5;
event_loop.run(move |event, _, control_flow| {

    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }

    let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    // we update `t`
    t += 0.0002;
    if t > 0.5 {
        t = -0.5;
    }
    
    let vertex1 = Vertex { position: [-0.5 + t, -0.5] };
    let vertex2 = Vertex { position: [ 0.0 + t,  0.5] };
    let vertex3 = Vertex { position: [ 0.5 + t, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
    target.finish().unwrap();
});
```

如果运行此代码，则应该看到三角形从屏幕的左侧到屏幕右侧，然后跳回左侧！
If you run this code, you should see your triangle going from the left to the right of the screen, then jumping back to the left!

这种方法大约是游戏程序员在1990年代所做的。 当您具有小的形状（如单个三角形）时，这很好，但是当您用数千个多边形操纵模型时，它的效率很高。 有两个原因：
This method is approximately what game programmers were doing in the 1990s. This works perfectly fine when you have small shapes (like a single triangle), but it is highly inefficient when you manipulate models with thousands of polygons. There are two reasons for this:

CPU每次绘制时都会花费大量时间来计算坐标（每个模型的每个顶点都有一个操作，最后您可以达到数十万操作）。
The CPU would spend a lot of time calculating the coordinates every time you draw (with one operation for each vertex for each model, at the end you reach hundreds of thousands of operations).

将我们的形状从RAM上传到视频内存需要一些时间。 这次完全浪费了，因为GPU必须等到转移完成工作才能开始工作。
It takes some time to upload our shape from the RAM to the video memory. This time is totally wasted as the GPU has to wait until the transfer is finished to start its work.

## Uniforms
你还记得顶峰着色器吗？ 我们的顶点着色器将每个顶点的属性作为输入，并输出其在窗口上的位置。 我们将要求GPU执行此操作，而不是在程序中进行添加并上传结果。
Do you remember vertex shaders? Our vertex shader takes as input the attributes of each vertex, and outputs its position on the window. Instead of doing the addition in our program and upload the result, we are going to ask the GPU to do this operation.

让我们将我们的程序重置为第一个教程结束时的内容，但请保持t
Let's reset our program to what it was at the end of the first tutorial, but keep t:

``` rust
let vertex1 = Vertex { position: [-0.5, -0.5] };
let vertex2 = Vertex { position: [ 0.0,  0.5] };
let vertex3 = Vertex { position: [ 0.5, -0.25] };
let shape = vec![vertex1, vertex2, vertex3];

let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
let mut t: f32 = -0.5;
event_loop.run(move |event, _, control_flow| {

    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }

    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    // we update `t`
    t += 0.0002;
    if t > 0.5 {
        t = -0.5;
    }

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
    target.finish().unwrap();
});
```

相反，我们将在顶点着色器中进行一些更改
And instead we are going to do a small change in our vertex shader:

```rust
let vertex_shader_src = r#"

    in vec2 position;

    uniform float t;

    void main() {
        vec2 pos = position;
        pos.x += t;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;
```

您可能会注意到，这正是我们上面正在做的操作，除了这次是在GPU方面完成的。 我们在着色器中添加了一个可变t，该t被声明为统一。 统一是一个全局变量，当我们通过将其值传递给绘制函数时绘制其值时，其值是设置的。 最简单的方法是使用制服！ 宏
You may notice that this is exactly the operation that we've been doing above, except that this time it is done on the GPU side. We have added a variable t in our shader, which is declared as a uniform. A uniform is a global variable whose value is set when we draw by passing its value to the draw function. The easiest way to do so is to use the uniform! macro:

```rust
target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t },
            &Default::default()).unwrap();
```

使用统一变量解决了我们上面的两个问题。 CPU不必进行任何计算，其上传的只是t（单个浮点）而不是整个形状的值。
Using uniform variables solves our two problems above. The CPU doesn't have to do any calculation, and all that it uploaded is the value of t (a single float) instead of the whole shape.