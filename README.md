### glutin::event_loop::EventLoop 
提供了一种从系统和已注册到事件循环的窗口中检索事件的方法。
Provides a way to retrieve events from the system and from the windows that were registered to the events loop.

事件卢比或多或少地视为“上下文”。 呼叫Eventloop :: new（）初始化创建Windows所需的所有内容。 例如，在Linux上创建事件循环会打开与X或Wayland服务器的连接。
An EventLoop can be seen more or less as a “context”. Calling EventLoop::new() initializes everything that will be required to create windows. For example on Linux creating an event loop opens a connection to the X or Wayland server.

要从另一个线程唤醒事件卢比，请参阅Eventloopproxy文档。
To wake up an EventLoop from a another thread, see the EventLoopProxy docs.

请注意，事件卢比不能跨线程共享（由于平台依赖性逻辑禁止它），因此既不发送也不同步。 如果您需要跨线程访问，则可以将此Eventloop创建的窗口发送到另一个线程，并且Eventloopproxy允许您从另一个线程唤醒Eventloop。
Note that the EventLoop cannot be shared across threads (due to platform-dependant logic forbidding it), as such it is neither Send nor Sync. If you need cross-thread access, the Window created from this EventLoop can be sent to an other thread, and the EventLoopProxy allows you to wake up an EventLoop from another thread.

run():
劫持了调用线程，并在提供的封闭情况下初始化Winit事件循环。 由于关闭是静态的，因此，如果需要从调用上下文访问任何数据，则必须是移动闭合。
Hijacks the calling thread and initializes the winit event loop with the provided closure. Since the closure is 'static, it must be a move closure if it needs to access any data from the calling context.

有关如何更改＆mut控制流影响事件循环的行为的信息，请参见ControlFlow文档
See the ControlFlow docs for information on how changes to &mut ControlFlow impact the event loop’s behavior.

任何未传递给此功能的值都不会删除。
Any values not passed to this function will not be dropped.




glutin::event_loop::EventLoopClosed
* 当事件曲线螺纹试图唤醒不再存在的事件行动时，返回的错误。 包含给send_event的原始事件
* The error that is returned when an EventLoopProxy attempts to wake up an EventLoop that no longer exists. Contains the original event given to send_event.

glutin::event_loop::EventLoopProxy
* 用于将自定义事件发送到Eventloop。
* Used to send custom events to EventLoop.

glutin::event_loop::EventLoopWindowTarget
* 针对将Windows与Eventloop关联的目标。
* Target that associates windows with an EventLoop.






glutin::window::WindowBuilder

glutin::ContextBuilder


glium::Display



Note:
* Default::default


1. 着色器
2. 缓冲---顶点、纹理、光照
3. 画

