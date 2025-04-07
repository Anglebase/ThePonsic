use ponsic::{
    App, SystemError, WindowManager,
    graphics::context_2d::Rect,
    widgets::{MainWindow, PushButton},
};

fn main() -> Result<(), SystemError> {
    let main_window = MainWindow::new(Rect::from_ps(100, 100, 800, 600), "New Ponsic Window")?;
    let mut buttons = vec![];

    for i in 0..8 {
        for j in 0..12 {
            let mut button = PushButton::new(
                Rect::from_ps(i * 90 + 10, j * 45 + 10, 80, 40),
                &format!("Button {}x{}", i, j),
                &main_window,
            )?;
            button.add_callback(move |b| {
                println!("Button {}x{} pressed!: {}", i, j, b);
            });
            buttons.push(button);
        }
    }

    main_window.show();
    buttons.iter_mut().for_each(|b| b.show());

    while App::handle_event(true).unwrap_or(true) {}
    Ok(())
}
