use crate::ui::geometry::{Bounds, Point, Size};

use super::Drawable;

pub struct Layout<Child> {
    children: Vec<Child>,
    props: Props,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Props {
    pub direction: Direction,
}

impl<Child> Layout<Child> {
    pub fn empty(props: Props) -> Self {
        Self {
            children: Vec::new(),
            props,
        }
    }

    pub fn with_children(children: Vec<Child>, props: Props) -> Self {
        Self { children, props }
    }
}

impl<Child: Drawable> Drawable for Layout<Child> {
    fn content_size(&self, bounds: super::geometry::Bounds) -> super::geometry::Size {
        let Bounds {
            position: Point { x, y },
            size: Size { width, height },
        } = bounds;

        match self.props.direction {
            Direction::Row => {
                let mut used_width = 0;

                for child in &self.children {
                    let child_bounds = Bounds::new(x + used_width, y, width - used_width, height);
                    let child_size = child.content_size(child_bounds);

                    used_width += child_size.width;
                }

                Size::new(used_width, height)
            }
            Direction::Column => {
                let mut used_height = 0;

                for child in &self.children {
                    let child_bounds = Bounds::new(x, y + used_height, width, height - used_height);
                    let child_size = child.content_size(child_bounds);

                    used_height += child_size.height;
                }

                Size::new(width, used_height)
            }
        }
    }

    fn draw(&self, ctx: &super::Context, bounds: super::geometry::Bounds) -> super::UiResult<()> {
        let Bounds {
            position: Point { x, y },
            size: Size { width, height },
        } = bounds;

        match self.props.direction {
            Direction::Row => {
                let mut used_width = 0;

                for child in &self.children {
                    let child_bounds = Bounds::new(x + used_width, y, width - used_width, height);
                    let child_size = child.content_size(child_bounds);
                    child.draw(ctx, child_bounds)?;

                    used_width += child_size.width;
                }
            }
            Direction::Column => {
                let mut used_height = 0;

                for child in &self.children {
                    let child_bounds = Bounds::new(x, y + used_height, width, height - used_height);
                    let child_size = child.content_size(child_bounds);
                    child.draw(ctx, child_bounds)?;

                    used_height += child_size.height;
                }
            }
        }

        Ok(())
    }
}
