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
        let max_child_bounds = self.calculate_max_child_bounds(bounds);

        if let Some(padding) = &self.props.padding {
            max_child_bounds.padding_inset(&padding)
        } else {
            max_child_bounds
        }
    }

    fn calculate_max_child_bounds(&self, bounds: Bounds) -> Bounds {
        let mut width = if let Some(width) = self.props.width {
            width
        } else {
            bounds.size.width
        };
        if let Some(min_width) = self.props.min_width {
            width = width.max(min_width);
        }
        if let Some(max_width) = self.props.max_width {
            width = width.min(max_width);
        }

        let mut height = if let Some(height) = self.props.height {
            height
        } else {
            bounds.size.height
        };
        if let Some(min_height) = self.props.min_height {
            height = height.max(min_height);
        }
        if let Some(max_height) = self.props.max_height {
            height = height.min(max_height);
        }

        Bounds::new(bounds.position.x, bounds.position.y, width, height)
    }

    fn get_total_bounds(&self, bounds: Bounds) -> Bounds {
        let Size {
            width: child_width,
            height: child_height,
        } = self.child.content_size(self.get_child_bounds(bounds));

        let mut width = if let Some(width) = self.props.width {
            width
        } else {
            child_width
        };
        if let Some(min_width) = self.props.min_width {
            width = width.max(min_width);
        }
        if let Some(max_width) = self.props.max_width {
            width = width.min(max_width);
        }

        let mut height = if let Some(height) = self.props.height {
            height
        } else {
            child_height
        };
        if let Some(min_height) = self.props.min_height {
            height = height.max(min_height);
        }
        if let Some(max_height) = self.props.max_height {
            height = height.min(max_height);
        }

        width = width.min(bounds.size.width);
        height = height.min(bounds.size.height);

        Bounds::new(bounds.position.x, bounds.position.y, width, height)
    }

    fn draw_background(
        &self,
        ctx: &super::Context,
        bounds: super::geometry::Bounds,
    ) -> Result<(), super::Error> {
        let self_bounds = self.get_total_bounds(bounds);

        if let Some(bg_color) = &self.props.background_color {
            ctx.set_fill_color(&(*bg_color).into());
            ctx.fill_rect(self_bounds.into());
        }

        Ok(())
    }
}

impl<Child: Drawable> Drawable for Block<Child> {
    fn content_size(&self, bounds: super::geometry::Bounds) -> super::geometry::Size {
        self.get_total_bounds(bounds).size
    }

    fn draw(&self, ctx: &super::Context, bounds: super::geometry::Bounds) -> UiResult<()> {
        self.draw_background(ctx, bounds)?;

        let child_bounds = self.get_child_bounds(bounds);
        self.child.draw(ctx, child_bounds)?;

        Ok(())
    }
}
