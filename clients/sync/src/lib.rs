#[doc(hidden)]
pub mod private;

pub use cornucopia_client_core::ArrayIterator;
pub use cornucopia_client_core::ArraySql;
pub use cornucopia_client_core::BytesSql;
pub use cornucopia_client_core::IterSql;
pub use cornucopia_client_core::StringSql;

/// This trait allows you to bind parameters to a query using a single
/// struct, rather than passing each bind parameter as a function parameter.
pub trait Params<'a, P, O, C> {
    fn params(&'a mut self, client: &'a mut C, params: &'a P) -> O;
}