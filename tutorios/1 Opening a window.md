Creating a project
To start this tutorial, we will create a new project from scratch. Even though it's highly recommended to be familiar with Rust and Cargo before starting, some little reminders are always good. Let's start by running:

> cargo new --bin my_project
> cd my_project

The directory you have just created should contain a Cargo.toml file which contains our project's metadata, plus a src/main.rs file which contains the Rust source code. If you have src/lib.rs file instead, that means that you forgot the --bin flag ; just rename the file.

In order to use the glium library, we need to add them as dependencies in our Cargo.toml file:

[dependencies]
glium = "*"

Before we can use them, we also need to import this library in our src/main.rs file, like this:

#[macro_use]
extern crate glium;

fn main() {
}

It is now time to start filling the main function!

Creating a window
The first step when creating a graphical application is to create a window. If you have ever worked with OpenGL before, you know how hard it is to do this correctly. Both window creation and context creation are platform-specific, and they are sometimes weird and tedious. Fortunately, this is where the glutin library shines.

Initializing an OpenGL window with glutin can be done using the following steps:

Creating an EventLoop for handling window and device events.
Specify Window parameters using glium::glutin::WindowBuilder::new(). These are window-specific attributes that have nothing to do with OpenGL.
Specify Context parameters using glium::glutin::ContextBuilder::new(). Here we specify OpenGL-specific attributes like multisampling or vsync.
Create the OpenGL window (in glium, this is the Display): glium::Display::new(window, context, &event_loop).unwrap(). This builds a Display using the given window and context attributes, and registers the window with the given event_loop.

```rust
fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
}
```

But there is a problem: as soon as the window has been created, our main function exits and display's destructor closes the window. To prevent this, we need to loop forever until we detect that a CloseRequested event has been received. We do so by calling event_loop.run:

```rust

event_loop.run(move |ev, _, control_flow| {
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    match ev {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            _ => return,
        },
        _ => (),
    }
});
```
您现在可以执行货物运行。 几分钟后，货物下载并汇编了Grium及其依赖性，您应该看到一个不错的小窗户。
You can now execute cargo run. After a few minutes during which Cargo downloads and compiles glium and its dependencies, you should see a nice little window.

#### Clearing the color
但是，窗口的内容不是很吸引人。 根据您的系统，它可以显得黑色，显示随机图像或仅下雪。 期望我们在窗口上绘制，因此该系统不会打扰将其颜色初始化为特定值。
The content of the window, however, is not not very appealing. Depending on your system, it can appear black, show a random image, or just some snow. We are expected to draw on the window, so the system doesn't bother initializing its color to a specific value.

Glium和OpenGL API类似于绘制Windows油漆或GIMP等绘制软件。 我们从一个空图像开始，然后在其上绘制一个对象，然后在另一个对象上绘制一个对象，然后是另一个对象等。直到我们对结果感到满意。 但是与绘图软件相反，您不希望用户看到中间步骤。 仅应显示最终结果。
Glium and the OpenGL API work similarly to drawing software like Windows' Paint or The GIMP. We start with an empty image, then draw an object on it, then another object, then another object, etc. until we are satisfied with the result. But contrary to drawing software, you don't want your users to see the intermediate steps. Only the final result should be shown.

为了处理这一点，OpenGL使用所谓的双缓冲。 我们没有直接绘制到窗口，而是绘制到存储在内存中的图像。 一旦完成绘图，该图像就会复制到窗口。 这是框架对象以敏锐的形式表示的。 当您想开始在窗口上绘制某些内容时，必须首先调用display.draw（）才能产生帧
To handle this, OpenGL uses what is called double buffering. Instead of drawing directly to the window, we are drawing to an image stored in memory. Once we have finished drawing, this image is copied to the window. This is represented in glium by the Frame object. When you want to start drawing something on your window, you must first call display.draw() in order to produce a Frame:

```rust
let mut target = display.draw();
```

然后，我们可以将此目标用作图表。 OpenGL和Grium提供的操作之一是用给定的颜色填充表面。 这就是我们要做的。
We can then use this target as a drawing surface. One of the operations that OpenGL and glium provide is filling the surface with a given color. This is what we are going to do.

> target.clear_color(0.0, 0.0, 1.0, 1.0);

请注意，要使用此功能，我们将需要首先导入表面特征：
Note that to use this function, we will need to import the Surface trait first:

> use glium::Surface;

我们传递给clear_color的四个值表示我们颜色的四个组成部分：红色，绿色，蓝色和alpha。 仅在0.0到1.0之间的值有效。 在这里，我们正在画不透明的蓝色。
The four values that we pass to clear_color represent the four components of our color: red, green, blue and alpha. Only values between 0.0 and 1.0 are valid. Here we are drawing an opaque blue color.

就像我上面解释的那样，用户没有立即在屏幕上看到蓝色。 在这一点上，如果我们正在真正的应用中，我们很可能会绘制我们的角色，武器，地面，天空等。但是在本教程中，我们将在这里停留：
Like I explained above, the user doesn't immediately see the blue color on the screen. At this point if we were in a real application, we would most likely draw our characters, their weapons, the ground, the sky, etc. But in this tutorial we will just stop here:

> target.finish().unwrap();

这个呼叫完成（）意味着我们已经完成了绘图。 它会破坏框架对象，并将我们的背景图像复制到窗口。 我们的窗户现在充满了蓝色。
This call to finish() means that we have finished drawing. It destroys the Frame object and copies our background image to the window. Our window is now filled with blue.

这是我们在此步骤之后的完整主要功能：
Here is our full main function after this step:

extern crate glium;

```rust
fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
```