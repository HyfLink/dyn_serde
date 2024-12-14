//! TODO: module-level documentation
//!
//!
//!
//!

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(feature = "std")]
use std::boxed::Box;

/// An arbitrary value.
///
/// In the dynamic deserialization, the concrete result ([`Visitor::Value`])
/// will be boxed and converted into `Box<dyn Arbitrary>`.
///
/// Different from [`Any`], `Arbitrary` does not provide with information like
/// [type id], thus there are no safe ways to downcast the boxed trait objects
/// into concrete values.
///
/// [`Visitor::Value`]: serde::de::Visitor::Value
/// [`Any`]: https://doc.rust-lang.org/std/any/trait.Any.html
/// [type id]: https://doc.rust-lang.org/std/any/struct.TypeId.html
pub trait Arbitrary {}

// /////////////////////////////////////////////////////////////////////////////
// TRAIT IMPLEMENTATION
// /////////////////////////////////////////////////////////////////////////////

impl<T> Arbitrary for T {}

// /////////////////////////////////////////////////////////////////////////////
// TRAIT OBJECT IMPLEMENTATION
// /////////////////////////////////////////////////////////////////////////////

impl<'a> dyn Arbitrary + 'a {
    /// Boxes the value and returns an trait object.
    ///
    /// # Example
    ///
    /// ```
    /// use dyn_serde::de::Arbitrary;
    ///
    /// let value: Box<dyn Arbitrary> = <dyn Arbitrary>::new(123u32);
    /// let _ = value;
    /// ```
    #[inline]
    #[must_use]
    pub fn new<T: 'a>(value: T) -> Box<Self> {
        Box::new(value)
    }

    /// Downcasts the boxed trait object to a concrete type.
    ///
    /// # Safety
    ///
    /// The contained value must be of type `T`. Calling this method with the
    /// incorrect type is *undefined behavior*.
    ///
    /// # Example
    ///
    /// ```
    /// use dyn_serde::de::Arbitrary;
    ///
    /// let value = <dyn Arbitrary>::new(123u32);
    /// // SAFETY: `value` is just construct with an `u32`.
    /// let value: Box<u32> = unsafe { value.downcast_unchecked() };
    /// assert_eq!(*value, 123);
    /// ```
    #[must_use]
    pub unsafe fn downcast_unchecked<T: 'a>(self: Box<Self>) -> Box<T> {
        let ptr = Box::into_raw(self);
        // SAFETY: Assumes that the unsafe pre-conditions are satisfied.
        unsafe { Box::from_raw(ptr as *mut T) }
    }
}
