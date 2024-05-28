use wunderbar::ui::{
    app::App,
    block::{Block, Props},
    color::Color,
    geometry::{Bounds, Padding},
    window::{WindowInitOptions, WindowTags},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();

    let mut window = app.create_window(WindowInitOptions {
        bounds: Bounds::new(0, 0, 1728, 40),
        tags: Some(WindowTags::ExposeFade | WindowTags::PreventsActivation),
        ..Default::default()
    })?;

    let inner_block = Block::new(
        (),
        Props {
            background_color: Some(Color::RED),
            ..Default::default()
        },
    );

    let block = Block::new(
        inner_block,
        Props {
            background_color: Some(Color::GREEN),
            padding: Some(Padding::uni(4)),
            ..Default::default()
        },
    );

    window.bring_to_front()?;
    window.draw(block)?;

    app.run()?;

    Ok(())
}
