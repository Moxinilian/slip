use proc_macro_hack::proc_macro_hack;

/// Converts a string literal into a slip token.
/// See the [the repository](https://github.com/Moxinilian/slip) for usage.
#[proc_macro_hack]
pub use slip_imp::slip;