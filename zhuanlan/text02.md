# 用Rust写一个飞行员游戏-02

接着来写飞行员游戏.  

开始之前, 先把 *main.rs* 拆分成不同的文件, 单个文件的代码太长, 修改起来会很麻烦(当然也可以只写到一个文件里).  

### 01. 帧缓冲

帧缓冲是屏幕所显示画面的一个直接映像, 整个帧缓冲对应一帧图像.  

有关帧缓冲的内容, 可以参考 **[这里](https://learnopengl-cn.github.io/04%20Advanced%20OpenGL/05%20Framebuffers/)**. 接下来, 我们将按照上面链接文章里的思路, 把图像绘制到自定义的帧缓冲中, 再把帧缓冲渲染到纹理上, 然后把纹理贴到一个横跨整个屏幕的四边形上.  

之前一直都在用glium的默认帧缓冲:  

```
let mut target= display.draw();
```

帧缓冲由保存片段颜色信息的颜色缓冲, 保存片段深度信息的深度缓冲以及之前没用到的可以根据条件丢弃特定片段的模板缓冲组成.  

一个帧缓冲至少要有一个颜色缓冲, 同时因为我们在绘制图形时还进行了深度检测, 因此还需要附加一个深度缓冲.  

因此先创建两个纹理, 这两个纹理将被附加到我们创建的帧缓冲上, 在OpenGL绘制帧缓冲时, 颜色和深度信息会分别写入到这两个纹理中:    

```
// 创建保存颜色信息的纹理
let color_texture = glium::texture::srgb_texture2d::SrgbTexture2d::empty(&display, 800, 600).unwrap();
// 创建保存深度信息的纹理
let depth_texture = glium::texture::depth_texture2d::DepthTexture2d::empty(&display, 800, 600).unwrap();
```

之后创建帧缓冲:  

```
// 创建一个附加了颜色和深度附件的帧缓冲
let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&display, &color_texture, &depth_texture).unwrap();
// 清理帧缓冲的颜色和深度信息
frame_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
```

同时, 要想把上一节创建的物体绘制到该帧缓冲上, 首先要修改每个物体的 *draw* 方法:  

```
pub fn draw<S>(&self,
    target: &mut S,     // 将参数类型改成实现了trait glium::Surface 的类型
    program: &glium::Program,
    view: &[[f32; 4]; 4],
    perspective: &[[f32; 4]; 4],)
where
    S: glium::Surface,
```

之后将参数指定为我们新建的帧缓冲:    

```
airplane.draw(&mut frame_buffer, &program, &view_camera.view, &view_camera.perspective);
sea.wave(&display);
sea.draw(&mut frame_buffer, &program, &view_camera.view, &view_camera.perspective);
```

这样就完成了. 为了将保存颜色的纹理显示到屏幕上, 首先需要创建一个横跨整个屏幕的四边形.   

这个四边形上要贴纹理, 因此这里新建一个顶点结构体:  

```
#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 3], // 顶点坐标
    texcoord: [f32; 2], // 纹理坐标
}
implement_vertex!(Vertex, position, texcoord);
```

之后的步骤和立方体结构体一致, 不过该四边形绘制所需的着色器不同:  

顶点着色器代码:  

```
#version 330
layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texcoord;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = texcoord;
    gl_Position = vec4(position, 1.0);
}
```

片段着色器代码:

```
#version 330

in vec2 v_tex_coords;
out vec4 FragColor;

uniform sampler2D screen_texture;   // 将纹理作为uniform变量传入

void main() {
    FragColor = texture(screen_texture, v_tex_coords);
}
```

最后的结果如下图所示:  

![帧缓冲](11-framebuffer.png)

### 02. 抗锯齿

如果是用的默认的帧缓冲, 可以通过修改默认的 *context* 开启MSAA来消除锯齿:  

```
let context = glium::glutin::ContextBuilder::new()
                .with_depth_buffer(24)
                .with_srgb(true)
                .with_multisampling(4); // 只需添加这一句
```

效果如下:  

![抗锯齿](12-msaa.png)

在ubuntu下, 上面这句代码可能会报错, 这是glium的一个bug, 具体可以参考 **[这里](https://github.com/glium/glium/issues/1677)**.  

如果用的是新建的帧缓冲, 那么在为帧缓冲绑定附件的时候, 需要将深度缓冲和颜色缓冲的纹理类型从普通纹理替换成多重采样纹理:  

```
let color_texture = glium::texture::srgb_texture2d_multisample::SrgbTexture2dMultisample::empty(&display, 800, 600, 4).unwrap();
let depth_texture = glium::texture::depth_texture2d_multisample::DepthTexture2dMultisample::empty(&display, 800, 600, 4).unwrap();
```

之后就和前一节类似, 不过, 因为多重采样的图像比普通图像包含更多信息, 因此需要还原图像, 而不是将图像直接贴到屏幕上. 通过 *blit_color* 这类函数可以将一个帧缓冲中的某个区域复制到另一个帧缓冲中，并且将多重采样缓冲还原.   

```
// 将新建的帧缓冲中的内容复制到默认的帧缓冲中
target.blit_from_simple_framebuffer(
    &frame_buffer, 
    &glium::Rect{left:0, bottom: 0, width: 800, height: 600},   // 指定源缓冲区的图像大小
    &glium::BlitTarget{left:0, bottom: 0, width: 800, height: 600}, // 指定目标缓冲区的图像大小
    glium::uniforms::MagnifySamplerFilter::Nearest);
```

结果如下:  

![离屏MSAA](12-fmmsaa.png)

可以看到, 图像只占了窗口的一部分, 而窗口大小和绘制的图像大小都应该是800px*600px才对.  

这是因为窗口在创建时所使用的像素是 *逻辑像素* :  

```
// with_dimensions 函数的声明
pub fn with_dimensions(self, size: LogicalSize) -> WindowBuilder
```

而显卡在绘制图像时使用的是 *物理像素(或者设备像素)*.  

有关二者的区别, 可以参考 **[这里]()**. 简单来说, 物理像素就是真实的像素, 而不同显示器上的1个像素的大小是不一样的. 比如同样100px * 100px大小的图形, 在手机上会显得很大, 占了屏幕的很大一部分, 而在4K显示器上会显得很小. 逻辑像素则不一样, 100 * 100 逻辑像素的图形, 在不同的显示器上看上去是一样大的.  

逻辑像素和物理像素的比值跟显示器有关.  

因此, 需要在窗口创建时使用物理像素:  

```
// 首先创建另一个窗口, 用来获取显示器的逻辑像素和物理像素的比值
let monitor = glium::glutin::Window::new(&events_loop).unwrap();
let window = glium::glutin::WindowBuilder::new()
    .with_dimensions(
        glium::glutin::dpi::PhysicalSize::new(800.0, 600.0) // 创建物理像素
        .to_logical(monitor.get_hidpi_factor()))            // 然后转化为逻辑像素, 传递给绘制图形的窗口
    .with_title("aviator");
```

结果如下:  

![修改之后](12-fix.png)

### 03. 阴影

接下来, 在场景中添加阴影.  

