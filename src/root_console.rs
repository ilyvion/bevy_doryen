use crate::doryen::Console;
use std::ops::{Deref, DerefMut};

/// Provides access to the root console of the Doryen engine.
#[derive(Default)]
pub struct RootConsole(pub(crate) Option<Console>);

impl std::fmt::Debug for RootConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RootConsole")
            .field(
                "0",
                if self.0.is_some() {
                    &"<present>"
                } else {
                    &"<absent>"
                },
            )
            .finish()
    }
}

impl Deref for RootConsole {
    type Target = Console;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
            .as_ref()
            .expect("Inner value should always be set during `update` and `render` phases")
    }
}

impl DerefMut for RootConsole {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
            .as_mut()
            .expect("Inner value should always be set during `update` and `render` phases")
    }
}
