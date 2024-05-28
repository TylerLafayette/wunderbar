use super::{
    color::Color,
    geometry::{Bounds, Padding, Size},
    Drawable, UiResult,
};

pub struct Block<Child = ()> {
    child: Child,
    props: Props,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Props {
    pub min_width: Option<usize>,
    pub max_width: Option<usize>,
    pub min_height: Option<usize>,
    pub max_height: Option<usize>,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub background_color: Option<Color>,
    pub corner_radius: Option<usize>,
    pub padding: Option<Padding>,
}

impl<Child: Drawable> Block<Child> {
    pub fn new(child: Child, props: Props) -> Self {
        Self { child, props }
    }

    fn get_child_bounds(&self, bounds: Bounds) -> Bounds {
        if let Some(padding) = &self.props.padding {
            bounds.padding_inset(&padding)
        } else {
            bounds
        }
    }

    fn draw_background(
        &self,
        ctx: &super::Context,
        bounds: super::geometry::Bounds,
    ) -> Result<(), super::Error> {
        if let Some(bg_color) = &self.props.background_color {
            ctx.set_fill_color(&(*bg_color).into());
            ctx.fill_rect(bounds.into());
        }

        Ok(())
    }
}

impl<Child: Drawable> Drawable for Block<Child> {
    fn content_size(&self, bounds: super::geometry::Bounds) -> super::geometry::Size {
        let Size {
            width: child_width,
            height: child_height,
        } = self.child.content_size(self.get_child_bounds(bounds));

        if let Some(padding) = &self.props.padding {
            let width = child_width + padding.left + padding.right;
            let height = child_height + padding.top + padding.bottom;

            Size::new(width, height)
        } else {
            Size::new(child_width, child_height)
        }
    }

    fn draw(&self, ctx: &super::Context, bounds: super::geometry::Bounds) -> UiResult<()> {
        self.draw_background(ctx, bounds)?;

        let child_bounds = self.get_child_bounds(bounds);
        self.child.draw(ctx, child_bounds)?;

        Ok(())
    }
}
