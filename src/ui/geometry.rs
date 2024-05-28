use cocoa::foundation::{NSPoint, NSRect, NSSize};
use core_graphics::geometry::{CGPoint, CGRect, CGSize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Into<NSPoint> for Point {
    fn into(self) -> NSPoint {
        NSPoint::new(self.x as f64, self.y as f64)
    }
}

impl Into<CGPoint> for Point {
    fn into(self) -> CGPoint {
        CGPoint::new(self.x as f64, self.y as f64)
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn translate(self, dx: usize, dy: usize) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn translate_x(self, dx: usize) -> Self {
        self.translate(dx, 0)
    }

    pub fn translate_y(self, dy: usize) -> Self {
        self.translate(0, dy)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl std::ops::Sub for Size {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl std::ops::Add for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl Into<CGSize> for Size {
    fn into(self) -> CGSize {
        CGSize::new(self.width as f64, self.height as f64)
    }
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub fn expand(self, expand_by_width: usize, expand_by_height: usize) -> Self {
        Self {
            width: self.width + expand_by_width,
            height: self.height + expand_by_height,
        }
    }

    pub fn expand_width(self, expand_by_width: usize) -> Self {
        self.expand(expand_by_width, 0)
    }

    pub fn expand_height(self, expand_by_height: usize) -> Self {
        self.expand(0, expand_by_height)
    }

    pub fn contract(self, contract_by_width: usize, contract_by_height: usize) -> Self {
        Self {
            width: self.width - contract_by_width,
            height: self.height - contract_by_height,
        }
    }

    pub fn contract_width(self, contract_by_width: usize) -> Self {
        self.contract(contract_by_width, 0)
    }

    pub fn contract_height(self, contract_by_height: usize) -> Self {
        self.contract(0, contract_by_height)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Bounds {
    pub position: Point,
    pub size: Size,
}

impl Bounds {
    pub fn new(pos_x: usize, pos_y: usize, width: usize, height: usize) -> Self {
        Self {
            position: Point::new(pos_x, pos_y),
            size: Size::new(width, height),
        }
    }

    pub fn new_at_origin(width: usize, height: usize) -> Self {
        Self {
            position: Point::origin(),
            size: Size::new(width, height),
        }
    }

    pub fn new_with_zero_size(pos_x: usize, pos_y: usize) -> Self {
        Self {
            position: Point::new(pos_x, pos_y),
            size: Size::zero(),
        }
    }

    pub fn padding_inset(self, padding: &Padding) -> Self {
        let Point { x, y } = self.position;
        let Size { width, height } = self.size;

        Self {
            position: Point {
                x: x + padding.left,
                y: y + padding.top,
            },
            size: Size {
                width: width.checked_sub(padding.left + padding.right).unwrap_or(0),
                height: height
                    .checked_sub(padding.top + padding.bottom)
                    .unwrap_or(0),
            },
        }
    }
}

impl Into<NSRect> for Bounds {
    fn into(self) -> NSRect {
        let Point { x, y } = self.position;
        let Size { width, height } = self.size;

        NSRect::new(
            NSPoint::new(x as f64, y as f64),
            NSSize::new(width as f64, height as f64),
        )
    }
}

impl Into<CGRect> for Bounds {
    fn into(self) -> CGRect {
        let Point { x, y } = self.position;
        let Size { width, height } = self.size;

        CGRect::new(
            &CGPoint::new(x as f64, y as f64),
            &CGSize::new(width as f64, height as f64),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padding {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
}

impl Padding {
    pub fn new(left: usize, right: usize, top: usize, bottom: usize) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn uni(padding: usize) -> Self {
        Self {
            left: padding,
            right: padding,
            top: padding,
            bottom: padding,
        }
    }

    pub fn yx(y_padding: usize, x_padding: usize) -> Self {
        Self {
            left: x_padding,
            right: x_padding,
            top: y_padding,
            bottom: y_padding,
        }
    }
}
