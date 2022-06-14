pub trait Benchable {
    fn name(&self) -> &'static str;
    fn setup(&mut self) {}
    fn execute(&mut self);
    fn teardown(&mut self) {}
}

impl<T: Benchable + ?Sized> Benchable for Box<T> {
    fn name(&self) -> &'static str {
        (**self).name()
    }

    fn setup(&mut self) {
        (**self).setup()
    }

    fn execute(&mut self) {
        (**self).execute()
    }

    fn teardown(&mut self) {
        (**self).teardown()
    }
}
