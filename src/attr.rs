//! Shared attribute code for all types of netlink attributes.
//!  
//! This module is relatively small right now and will eventually
//! contain more code once type parameters in associated
//! types defined in traits are stabilized. Due to `neli` being
//! supported on stable and nightly, I cannot currently use
//! this feature and have opted to define implementations of the
//! trait separately for [`Rtattr`][crate::rtnl::Rtattr] and
//! [`Nlattr`][crate::genl::Nlattr] types in the
//! `rtnl.rs` and `genl.rs` modules respectively.

use crate::{err::NlError, Buffer, Nl};

/// Trait that defines shared operations for netlink attributes.
/// Currently, this applies to generic netlink and routing netlink
/// attributes.
pub trait Attribute<T> {
    /// Get the payload of the given attribute.
    ///
    /// Due to Rust's requirement that all elements of a [`Vec`] are of
    /// the same type, payloads are represented as a byte buffer so
    /// that nested attributes that contain multiple types for the
    /// payload can be type checked before serialization yet still
    /// contained all in the same top level attribute.
    fn payload(&self) -> &Buffer;

    /// Set the payload to a data type that implements `Nl` -
    /// this function will overwrite the current payload.
    ///
    /// This method serializes the `payload` parameter and stores
    /// the resulting byte buffer as the payload.
    fn set_payload<P>(&mut self, payload: &P) -> Result<(), NlError>
    where
        P: Nl;

    /// Get an [`Nlattr`][crate::genl::Nlattr] payload as the
    /// provided type parameter, `R`.
    fn get_payload_as<R>(&self) -> Result<R, NlError>
    where
        R: Nl,
    {
        R::deserialize(self.payload().as_ref()).map_err(NlError::new)
    }
}