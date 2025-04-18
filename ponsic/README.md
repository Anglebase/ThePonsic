# Ponsic

Ponsic 是一个专门为 Windows 平台开发的基于 Rust 的 GUI 程序开发框架

## 基本介绍
Ponsic 是基于 WIN32API 开发，并提供了安全且高自由度的接口，得益于它完全使用 WIN32 系统调用，使用你不需要在电脑上预先安装任何额外的库或运行时环境，并且也使它所开发的程序具有高性能的潜力，Ponsic 会尽可能避免引入运行时，做到几乎零成本的 WIN32API 安全抽象

## 特性
- **高性能**: 尽可能避免额外的运行时开销，提供高性能的 GUI 程序开发
- **安全性**: 提供安全的抽象层，防止直接使用系统调用带来的潜在风险
- **灵活性**: 提供高自由度的接口，开发者可以根据需求自由定制应用程序

## 快速入门

与其他框架不同，Ponsic 不会依赖 crate 之外的任何内容，你只需要在你的项目中添加以下依赖项便可以使用它：

```toml
[dependencies]
ponsic = "*"
```

Ponsic 具有较为接近原生 WIN32API 的使用方式，在显式窗口之前，你需要像在 WIN32 编程中一样，先注册一个窗口类，然后创建一个窗口实例，便可以显式窗口。

```rust
use ponsic::*;

fn main() -> Result<(), SystemError> {
    let class = Registrar::new("HelloWorld")
        .set_cursor(Cursor::Arrow)
        .set_process(wndproc!(();|Events { event,.. }|{
            if let Event::Mouse { button: Button::Left, status: ButtonStatus::Down, .. } = event {
                println!("Hello Ponsic!");
                return Return::Finish
            }
            if let Event::Window(WindowEvent::Destroy) = event {
                App::should_exit(0);
                return Return::Finish
            }
            Return::Default
        }))
        .build()?;

    let window = class
        .make_window(Rect::from((Point::new(100, 100), Size::new(800, 600))))
        .set_style(&[WindowStyle::OverlappedWindow])
        .set_title("Hello Ponsic!")
        .build()?;

    window.show();

    while App::handle_event(true).unwrap() {}

    Ok(())
}
```

和 WIN32 编程一样，你依然需要定义一个用于窗口过程的回调函数来处理窗口所产生的一系列事件，但 Ponsic 提供了一种更友好的方式：`wndproc!(...)`，这个宏可以方便地生成窗口过程的定义，并且你也可以通过这个宏指示一个绑定于窗口的类型，窗口便可以通过这个类型中你所定义的内容与其它窗口交互或存储窗口的状态信息

Ponsic 也提供了如下一种基于传统多态扩展的窗口定义方式，但并不推荐使用：

```rust
use ponsic::{
    *,
    widgets::{Proc, Window},
    graphics::context_2d::{Context2D, DrawText},
};

struct MyWindow {}

impl Proc for MyWindow {
    fn handle(&mut self, Events { event, .. }: Events) -> Return {
        if let Event::Window(WindowEvent::Destroy) = event {
            App::should_exit(0);
            return Return::Finish;
        }
        Return::Default
    }

    fn draw(&mut self, context: ponsic::graphics::Context) {
        let context: Context2D = context.into();
        context.draw_text("Hello World", &mut Recti::new(10, 10, 100, 100), &[]);
    }
}

fn main() {
    let window = Window::create(
        Rect::from((Point::new(100, 100), Size::new(800, 600))),
        "Hello World",
        None,
        MyWindow {},
    )
    .unwrap();

    window.show();

    while App::handle_event(true).unwrap() {}
}
```

## 线程相关

WIN32API 中并不能跨线程执行窗口操作，因而 Ponsic 中与窗口相关的大多数类型都不是 Send 或 Sync 的，如果想要在两个不同线程中运行的窗口进行通信，一种可行的方式是通过 Rust 的通道并利用窗口绑定类型来进行通信。

你可以在多个线程中创建窗口，但要保证每个拥有窗口的线程都具有自己的事件循环，窗口实例无法在线程间移动，`App::handle_event(...)` 也并不会处理其它线程的窗口事件。

## 许可证

Ponsic 及其所有子 crate 遵循 Apache-2.0 许可证