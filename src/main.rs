use wunderbar::ui::{
    app::App,
    block::{Block, BoxProps},
    color::Color,
    geometry::{Bounds, Padding},
    layout::{Direction, Layout, LayoutProps},
    text::{Font, Text, TextProps},
    window::{WindowInitOptions, WindowTags},
    Drawable, IntoBoxDrawable,
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
        resolution: Some(2.0),
        ..Default::default()
    })?;

    window.disable_shadow()?;

    let inner_block = Block::new(
        Text::new(
            "Hello world aaa  a a a a",
            TextProps {
                font: Font::new("Soleil", 14.0).unwrap_or_default(),
                color: Color::GREEN,
            },
        ),
        BoxProps {
            background_color: Some(Color::BLACK),
            padding: Some(Padding::uni(4)),
            ..Default::default()
        },
    );

    let block = Block::new(
        inner_block,
        BoxProps {
            background_color: Some(Color::BLUE),
            padding: Some(Padding::uni(10)),
            ..Default::default()
        },
    );

    let block2 = Block::new(
        (),
        BoxProps {
            background_color: Some(Color::GREEN),
            width: Some(4),
            height: Some(4),
            ..Default::default()
        },
    );

    let layout = Layout::with_children(
        vec![block.erase(), block2.erase()],
        LayoutProps {
            direction: Direction::Row,
        },
    );

    window.bring_to_front()?;
    window.draw(layout)?;

    app.run()?;

    Ok(())
}
