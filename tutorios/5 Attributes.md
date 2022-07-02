## Attributes 属性

在我们的编程管道中，三角形内部每个像素的颜色对应于我们片段着色器的输出。 由于我们的片段着色器返回（1.0、0.0、0.0、1.0），因此每个像素是不透明的红色（四个值对应于：红色，绿色，蓝色，alpha/obcity）。
In our programming pipeline, the color of each pixel inside the triangle corresponds to the output of our fragment shader. Since our fragment shader returns (1.0, 0.0, 0.0, 1.0), each pixel is an opaque red (the four values correspond to: red, green, blue, alpha/opacity).

为了输出正确的颜色，我们需要有一些有关我们要绘制的像素的信息。 幸运的是，可以在顶点和碎片着色器之间传递信息。
In order to output the correct color, we need to have some information about the pixel we are trying to draw. Fortunately, it is possible to pass information between the vertex and the fragment shader.

为此，我们只需在顶点着色器中添加一个变量...
To do so, we simply add an out variable in the vertex shader...

```js
#version 140

in vec2 position;
out vec2 my_attr;      // our new attribute

uniform mat4 matrix;

void main() {
    my_attr = position;     // we need to set the value of each `out` variable.
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}
```
...以及具有相同名称并输入片段着色器的变量。
...and an in variable with the same name and type in the fragment shader.


```js
#version 140

in vec2 my_attr;
out vec4 color;

void main() {
    color = vec4(my_attr, 0.0, 1.0);   // we build a vec4 from a vec2 and two floats
}
```

让我们看看发生了什么。 我们的顶点着色器被调用3次，一次每个顶点。 每个顶点返回my_attr的不同值。 然后，OpenGL确定在栅格化阶段哪些像素在三角形内部，并为每个像素中的每个像素调用一次片段着色器。 每个像素传递的MY_ATTR的值是根据像素的位置的位置的该值的插值。
Let's see what happens. Our vertex shader is invoked three times, once per vertex. Each vertex returns a different value for my_attr. OpenGL then determines which pixels are inside the triangle during the rasterization phase, and calls the fragment shader once for each of these pixels. The value of my_attr that is passed for each pixel is the interpolation of this value depending on the position of the pixel.

例如，在顶点旁边的像素将获得一个相等或非常接近my_attr值的my_attr值，即顶点着色器返回该顶点的MY_ATTR。 两个顶点之间边缘中间的像素将获得这两个顶点返回的My_attr的两个值的平均值。 三角形中间的像素将获得三个顶点的值的平均值。
For example, pixels that are right next to a vertex will get a value of my_attr that is equal or very near the value of my_attr that the vertex shader returned for this vertex. The pixel that is on the middle of the edge between two vertices will get the average of the two values of my_attr returned by the vertex shader for these two vertices. Pixels that are the middle of the triangle will get the average of the values of the three vertices.

注意：这是因为变量默认情况下具有光滑属性，这是您大多数时候想要的。 也可以指定平面属性。
Note: this is because variables have by default the smooth attribute, which is what you want most of the time. It is also possible to specify the flat attribute.

在上面的示例中，由顶点着色器返回的my_attr的值对应于顶点的位置。 因此，碎片着色器将获得的my_attr的值对应于正在处理的像素的位置。 为了进行演示，我们将这个位置变成颜色的红色和绿色组成部分。
In the example above, the value of my_attr returned by the vertex shader corresponds to the position of the vertex. Therefore the value of my_attr that the fragment shader will get corresponds to the position of the pixel being processed. For the demonstration, we turn this position into the red and green components of our color.

结果应该看起来像这样：
And the result should look like this:

The result

You can find the entire source code here.