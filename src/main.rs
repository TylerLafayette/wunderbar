use wunderbar::ui::{
    app::App,
    block::{Block, Props},
    color::Color,
    geometry::{Bounds, Padding},
    window::{WindowInitOptions, WindowTags},
    Drawable,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();

    let mut window = app.create_window(WindowInitOptions {
        bounds: Bounds::new(0, 0, 1728, 40),
        tags: Some(
            WindowTags::Sticky
                | WindowTags::ExposeFade
                | WindowTags::PreventsActivation
                | WindowTags::DisableShadow,
        ),
        ..Default::default()
    })?;

    window.disable_shadow()?;

    let inner_block = Block::new(
        (),
        Props {
            background_color: Some(Color::BLACK),
            min_width: Some(86),
            min_height: Some(26),
            ..Default::default()
        },
    );

    let block = Block::new(
        inner_block,
        Props {
            background_color: Some(Color::BLUE),
            padding: Some(Padding::uni(2)),
            min_width: Some(90),
            min_height: Some(30),
            ..Default::default()
        },
    );

    println!("{:?}", block.content_size(Bounds::new(0, 0, 1728, 40)));

    window.bring_to_front()?;
    window.draw(block)?;

    app.run()?;

    Ok(())
}
