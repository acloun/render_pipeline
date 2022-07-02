## Matrices 矩阵
我们正在以简单的添加方式将三角形从屏幕的左侧移到屏幕右侧。 但是，其他转换（例如旋转，偏斜或重新归日）又如何呢？
We are moving our triangle from the left to the right of the screen with a simple addition. But what about other transformations like rotations, skews or rescalings?

我们需要的所有几何操作都可以通过一些数学来完成：
All the geometrical operations that we need can be done with some maths:

用位置 *=因子重新缩放三角形； 
 旋转我们的三角形使用new_position = vec2（pos.x * cos（Angle） -  pos.y * sin（Angle），pos.x * sin（angle） + pos.y * cos（angle））;
Rescaling our triangle is done with position *= factor;
Rotating our triangle is done with new_position = vec2(pos.x * cos(angle) - pos.y * sin(angle), pos.x * sin(angle) + pos.y * cos(angle));
Skewing our triangle is done with position.x += position.y * factor;
但是，如果我们想进行轮换，然后翻译，然后再进行弯腰怎么办？ 还是偏斜和旋转？ 即使可以使用数学来做到这一点，但事情也会变得非常复杂。
But what if we want to do a rotation, then a translation, then a rescale? Or a skew and a rotation? Even though it's possible to do this with maths, things become very complex to handle.

相反，程序员使用矩阵。 矩阵是数字的二维表，可以代表几何转换。 在计算机图形学中，我们使用4x4矩阵。
Instead, programmers use matrices. A matrix is a two-dimensional table of numbers which can represent a geometrical transformation. In computer graphics, we use 4x4 matrices.

让我们回到我们移动的三角形。 我们将更改顶点着色器以使用矩阵。 我们将通过将矩阵应用于它们，而不是将t的值添加到坐标中。 这将我们的矩阵描述的转换应用于顶点的坐标。
Let's get back to our moving triangle. We are going to change the vertex shader to use a matrix. Instead of adding the value of t to the coordinates, we are going to apply the matrix to them by multiplying it. This applies the transformation described by our matrix to the vertex's coordinates.

```rust
let vertex_shader_src = r#"

    in vec2 position;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;
```

请注意，编写矩阵 *顶点而不是顶点 *矩阵很重要。 基质操作根据顺序产生不同的结果。
Note that it is important to write matrix * vertex and not vertex * matrix. Matrix operations produce different results depending on the order.

调用Draw函数时，我们还需要传递矩阵：
We also need to pass the matrix when calling the draw function:

``` rust
let uniforms = uniform! {
    matrix: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [ t , 0.0, 0.0, 1.0f32],
    ]
};

target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
```

请注意，在OpenGL，因此是Glium中，矩阵是柱状的。 如果我们要在标准数学符号中编写上述矩阵，这是行 - 马约尔，那将是这样的：
Note that in OpenGL, and therefore glium, the matrices are column-major. If we were to write the above matrix in standard mathematical notation, which is row-major, it would look like this:

1.0   0.0   0.0    t
0.0   1.0   0.0   0.0
0.0   0.0   1.0   0.0
0.0   0.0   0.0   1.0

您应该看到与以前完全相同的东西，但是我们现在拥有的更加灵活。 例如，如果我们想旋转三角形，我们可以尝试此矩阵：
You should see exactly the same thing as previously, but what we now have is much more flexible. For example, if instead we want to rotate the triangle we can try this matrix instead:

```rust
let uniforms = uniform! {
    matrix: [
        [ t.cos(), t.sin(), 0.0, 0.0],
        [-t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
};
```
您可以在此处找到整个源代码。
You can find the entire source code here.