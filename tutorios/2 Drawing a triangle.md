除某些例外（例如上面使用的清算操作），OpenGL不能提供任何功能来轻松绘制形状。 例如，没有draw_rectangle，draw_cube或draw_text函数。 取而代之的是，一切都以相同的方式处理：通过图形管道。 无论您绘制一个简单的三角形还是具有数千个多边形和高级阴影技术的3D模型都没关系，一切都使用相同的机制。
With some exceptions (like the clearing operation that was used above), OpenGL doesn't provide any function to easily draw shapes. There is no draw_rectangle, draw_cube or draw_text function for example. Instead everything is handled the same way: through the graphics pipeline. It doesn't matter whether you draw a simple triangle or a 3D model with thousands of polygons and advanced shadowing techniques, everything uses the same mechanics.

这就是学习曲线变得非常陡峭的一点，因为您需要学习图形管道的工作原理，即使您只想绘制一个三角形。 但是，一旦您通过了这一步骤，将变得更容易理解其余部分。
This is the point where the learning curve becomes very steep, as you need to learn how the graphics pipeline works even if you just want to draw a single triangle. However once you have passed that step, it will become easier to understand the rest.

在我们绘制三角形之前，我们需要在初始化期间准备两件事：
Before we can draw a triangle, we need to prepare two things during the initialization:

描述我们三角形的形状。
GPU将执行的程序。
A shape that describes our triangle.
A program that will be executed by the GPU.

形状
形状表示对象的几何形状。 当您认为“几何形状”时，您可能会想到正方形，圆形等，但是在图形编程中，我们要操纵的唯一形状是三角形（注意：镶嵌可以解锁使用其他多边形的可能性，但这是高级的 话题）。
Shape
A shape represents the geometry of an object. When you think "geometry", you may think of squares, circles, etc., but in graphics programming the only shapes that we are going to manipulate are triangles (note: tessellation unlocks the possibility to use other polygons, but this is an advanced topic).

这是对象形状的示例。 如您所见，它是由数百个三角形和只有三角形组成的。
Here is an example of an object's shape. As you can see, it is made of hundreds of triangles and only triangles.

著名的犹他州茶壶
The famous Utah Teapot

每个三角形都是由三个顶点组成的，这意味着形状只是链接在一起以形成三角形的顶点的集合。 用Glium描述这样的形状的第一步是创建一个名为“顶点”的结构（实际名称无关紧要），其目的是描述每个单独的顶点。 我们的顶点集合以后可以由Vec <vertex>表示。
Each triangle is made of three vertices, which means that a shape is just a collection of vertices linked together to form triangles. The first step to describe a shape like this with glium is to create a struct named Vertex (the actual name doesn't matter) whose purpose is to describe each individual vertex. Our collection of vertices can later be represented by a Vec<Vertex>.

```rust
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
```

我们的结构包含一个位置字段，我们将用于将每个顶点的位置存储在窗口上。 作为真正的矢量渲染器，OpenGL不使用像素中的坐标。 取而代之的是，它认为该窗口的宽度和高度为2个单元，并且原点位于窗口的中心。
Our struct contains a position field which we will use to store the position of each vertex on the window. Being a true vectorial renderer, OpenGL doesn't use coordinates in pixels. Instead it considers that the window has a width and a height of 2 units, and that the origin is at the center of the window.

Windows坐标系统
The windows coordinates system

当我们给OpenGL的位置时，我们需要使用此坐标系。 让我们为我们的三角形选择一个形状，例如这个：
When we give positions to OpenGL, we need to use this coordinate system. Let's pick a shape for our triangle, for example this one:

找到我们三角的坐标
Finding the coordinates of our triangle

转化为此代码：
Which translates into this code:

```rust
let vertex1 = Vertex { position: [-0.5, -0.5] };
let vertex2 = Vertex { position: [ 0.0,  0.5] };
let vertex3 = Vertex { position: [ 0.5, -0.25] };
let shape = vec![vertex1, vertex2, vertex3];
```

我们现在有自己的形状！ 最后一步是将这种形状上传到我们的视频卡中的内存中，以更快的访问速度。 即使这不是严格必要的，也很容易做到这一点，这将使我们的平局操作更快。
We now have our shape! There is a last step which consists in uploading this shape to the memory of our video card in what is called a vertex buffer, for faster access. Even though that is not strictly necessary, it is very easy to do so and it will make our draw operation considerably faster.

```rust
let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
```

更复杂的形状由数百千万个顶点组成。 我们不仅需要有一个顶点列表，还需要一种告诉OpenGL如何将这些顶点链接在一起以获得三角形的方法。 由于我们只有一个三角形，这与我们无关，因此我们只创建一个虚拟标记，以后我们将传递给Glium。
More complex shapes consist of hundred or thousands of vertices. We not only need to have a list of vertices, but also a way to tell OpenGL how to link these vertices together to obtain triangles. Since we only have one triangle, this isn't really relevant for us, so we just create a dummy marker that we will pass to glium later on.

``` rust
let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
```

程序
当第一次在1990年代创建OpenGL时，绘制一个对象简单地组成的是与各种参数一起发送形状，例如颜色，照明方向，雾距离等。但是，这些参数很快对于游戏创建者而言过于限制，而OpenGL 2是 添加了更灵活的系统，其中包括所谓的着色器。 几年后发布OpenGL 3时，所有这些参数被删除并完全由着色器取代。
Program
When OpenGL was first created in the 1990s, drawing an object simply consisted in sending a shape alongside with various parameters like the color, lighting direction, fog distance, etc. But these parameters quickly became too limiting for game creators, and when OpenGL 2 was released a more flexible system was added with what are called shaders. When OpenGL 3 was released a few years later, all these parameters were removed and totally replaced by shaders.

为了绘制三角形，您将需要对绘图过程（也称为管道）的工作方式进行一些基本了解。
In order to draw a triangle, you will need some basic understanding about how the drawing process (also called the pipeline) works.

图形管道
The graphics pipeline

模式左侧的坐标列表表示我们之前创建的形状的顶点。 当我们要求GPU绘制这种形状时，它将首先执行所谓的顶点着色器，一次为每个顶点（这意味着三次）。 顶点着色器是一个小程序，其目的是告诉GPU每个顶点的屏幕坐标是什么。 然后，GPU构建了我们的三角形，并确定屏幕的哪些像素在其中。 然后，它将为每个像素中的每个像素执行一次片段着色器。 片段着色器是一个小程序，其目的是告诉GPU每个像素的颜色需要什么。
The list of coordinates at the left of the schema represents the vertices of the shape that we have created earlier. When we will ask the GPU to draw this shape, it will first execute what is called a vertex shader, once for each vertex (that means three times here). A vertex shader is a small program whose purpose is to tell the GPU what the screen coordinates of each vertex is. Then the GPU builds our triangle and determines which pixels of the screen are inside of it. It will then execute a fragment shader once for each of these pixels. A fragment shader is a small program whose purpose is to tell the GPU what the color of each pixel needs to be.

棘手的部分是我们需要编写顶点和碎片着色器。 为此，我们必须使用名为GLSL的编程语言编写它，该语言与C编程语言非常相似。 教您GLSL现在有点太复杂了，所以我只会为您提供源代码。 这是我们将用于顶点着色器的源代码：
The tricky part is that we need to write the vertex and fragment shaders. To do so, we have to write it using a programming language named GLSL, which is very similar to the C programming language. Teaching you GLSL would be a bit too complicated for now, so I will just give you the source codes. Here is the source code that we will use for the vertex shader:

```rust
let vertex_shader_src = r#"

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;
```

首先，#version 140行在这里告诉OpenGL该源代码对应的GLSL的哪个版本。 一些硬件不支持GLSL的最新版本，因此，如果可能的话，我们正在尝试坚持较早的版本。
First of all, the #version 140 line is here to tell OpenGL what version of GLSL this source code corresponds to. Some hardware doesn't support the latest versions of GLSL, so we are trying to stick to earlier versions if possible.

当我们以形状定义顶点结构时，我们创建了一个名为位置的字段，该字段包含顶点的位置。 但是与我让您想到的相反，该结构不包含顶点的实际位置，而只是其值传递给顶点着色器的属性。 OpenGL不在乎属性的名称，它所做的就是将其值传递给顶点着色器。 在VEC2位置； 我们的着色器行在这里声明我们将通过一个属性位置，其类型为VEC2（对应于Rust中的[F32; 2]）。
When we defined the Vertex struct in our shape, we created a field named position which contains the position of our vertex. But contrary to what I let you think, this struct doesn't contain the actual position of the vertex but only an attribute whose value is passed to the vertex shader. OpenGL doesn't care about the name of the attribute, all it does is passing its value to the vertex shader. The in vec2 position; line of our shader is here to declare that we are expected to be passed an attribute named position whose type is vec2 (which corresponds to [f32; 2] in Rust).

我们的着色器的主要功能被称为每个顶点一次，这意味着我们的三角形三倍。 第一次，位置的值将是[-0.5，-0.5]，第二次将是[0，0.5]，第三次[0.5，-0.25]。 正是在此函数中，我们实际上告诉OpenGL我们的顶点的位置是什么，这要归功于GL_POINTION = VEC4（位置，0.0，1.0）; 线。 我们需要进行少量转换，因为OpenGL不希望二维坐标，而是四维坐标（其原因将在后来的教程中介绍）。
The main function of our shader is called once per vertex, which means three times for our triangle. The first time, the value of position will be [-0.5, -0.5], the second time it will be [0, 0.5], and the third time [0.5, -0.25]. It is in this function that we actually tell OpenGL what the position of our vertex is, thanks to the gl_Position = vec4(position, 0.0, 1.0); line. We need to do a small conversion because OpenGL doesn't expect two-dimensional coordinates, but four-dimensional coordinates (the reason for this will be covered in a later tutorial).

第二个着色器称为片段着色器（有时也称为像素着色器）。
The second shader is called the fragment shader (sometimes also named pixel shader).

``` rust
let fragment_shader_src = r#"

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;
```

此源代码与上面的顶点着色器非常相似。 这次，主函数每像素一次执行一次，并且必须返回此像素的颜色，我们将其与颜色= Vec4（1.0、0.0、0.0、0.0、1.0）一起进行。 线。 就像较早的clear_color一样，我们需要传递像素的红色，绿色，蓝色和α组件。 在这里，我们正在返回不透明的红色。 根据像素，可以返回不同的值，但这将在下一个教程中介绍。
This source code is very similar to our vertex shader above. This time the main function is executed once per pixel and has to return the color of this pixel, which we do with the color = vec4(1.0, 0.0, 0.0, 1.0); line. Just like with clear_color earlier, we need to pass the red, green, blue and alpha components of the pixel. Here we are returning an opaque red color. It is possible to return different values depending on the pixel, but this will be covered in the next tutorials.

现在，我们已经编写了着色器的源代码，让我们将它们发送到Glium库：
Now that we have written our shaders' source codes, let's send them to the glium library:

``` rust
let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
```

绘制
现在我们已经准备好了形状和程序，我们终于可以绘制这个三角形！
Drawing
Now that we have prepared our shape and program, we can finally draw this triangle!

还记得目标对象吗？ 我们将需要使用它来启动拉动操作。
Remember the target object? We will need to use it to start a draw operation.

```rust
let mut target = display.draw();
target.clear_color(0.0, 0.0, 1.0, 1.0);
// draw the triangle here
target.finish().unwrap();
```

启动拉动操作需要几件事：顶点源（在这里我们使用我们的Vertex_buffer），索引源（我们使用索引变量），程序，程序的制服和一些绘制参数。 我们将在下一个教程中解释哪些制服和绘制参数是什么，但是目前，我们将通过传递空格标记并构建默认的绘制参数来忽略它们。
Starting a draw operation needs several things: a source of vertices (here we use our vertex_buffer), a source of indices (we use our indices variable), a program, the program's uniforms, and some draw parameters. We will explain what uniforms and draw parameters are in the next tutorials, but for the moment we will just ignore them by passing an EmptyUniforms marker and by building the default draw parameters.

```rust
target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
```

“绘制命令”的名称可能会让您认为绘图是一项繁重的操作，需要大量时间。 实际上，绘制三角形的时间不到几微秒，如果一切顺利，您应该看到一个不错的小三角形：
The "draw command" designation could make you think that drawing is a heavy operation that takes a lot of time. In reality drawing a triangle takes less than a few microseconds, and if everything goes well you should see a nice little triangle:

我们的最终结果
Our final resul

您可以在此处找到整个源代码。
You can find the entire source code here.
