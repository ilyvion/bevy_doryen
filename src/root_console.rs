use doryen_rs::Console;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct DoryenRootConsole(pub(crate) Option<Console>);
impl Deref for DoryenRootConsole {
    type Target = Console;

    fn deref(&self) -> &Self::Target {
        self.0
            .as_ref()
            .expect("Inner value should always be set during `update` and `render` phases")
    }
}
impl DerefMut for DoryenRootConsole {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
            .as_mut()
            .expect("Inner value should always be set during `update` and `render` phases")
    }
}
