use orbtk::prelude::*;

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(
                    TextBox::new()
                        .text("OrbTk with a very very very very very very very long text")
                        .margin(4.0)
                        .build(ctx),
                )
                .build(ctx)
        })
        .run();
}
