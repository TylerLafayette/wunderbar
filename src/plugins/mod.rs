mod clock;

pub trait Plugin {
    fn initialize(&mut self);
    fn deinitialize(&mut self) {}
}

type P = Box<dyn Plugin>;

fn plugin() -> P {
    todo!()
}
