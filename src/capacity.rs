use core::convert::TryFrom;
use core::fmt;
use core::num::NonZeroUsize;

/// The capacity of a ring buffer.
pub trait Capacity: Into<NonZeroUsize> + TryFrom<usize> + Sized {
    /// Construct a new capacity from a compile-time value.
    ///
    /// If the given value is invalid, a compile-time error will occur.
    fn from_ct<const N: usize>() -> Self;
}

/// A non-zero capacity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroCapacity {
    inner: NonZeroUsize,
}

impl NonZeroCapacity {
    /// Construct a new [`NonZeroCapacity`], assuming it is valid.
    ///
    /// This is an `unsafe` function; if the given value is zero, undefined
    /// behaviour will occur.
    pub unsafe fn new_unchecked(value: usize) -> Self {
        // SAFETY: The caller checks for us that the value is non-zero.
        Self { inner: NonZeroUsize::new_unchecked(value) }
    }
}

impl Capacity for NonZeroCapacity {
    fn from_ct<const N: usize>() -> Self {
        struct Check<const N: usize> { _phd: [(); N] }
        impl<const N: usize> Check<N> {
            const IS_VALID: () = assert!(N != 0,
                "The given capacity value must be non-zero!");
        }

        let () = Check::<N>::IS_VALID;
        // SAFETY: We just asserted that 'N' is non-zero.
        unsafe { Self::new_unchecked(N) }
    }
}

impl From<NonZeroCapacity> for NonZeroUsize {
    fn from(value: NonZeroCapacity) -> Self {
        value.inner
    }
}

impl From<NonZeroUsize> for NonZeroCapacity {
    fn from(value: NonZeroUsize) -> Self {
        Self { inner: value }
    }
}

impl TryFrom<usize> for NonZeroCapacity {
    type Error = NonZeroCapacityError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value != 0 {
            // SAFETY: We just checked that the value is non-zero.
            Ok(unsafe { Self::new_unchecked(value) })
        } else {
            Err(NonZeroCapacityError)
        }
    }
}

impl fmt::Display for NonZeroCapacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// An error in constructing a [`NonZeroCapacity`].
#[derive(Clone, Debug)]
pub struct NonZeroCapacityError;

impl fmt::Display for NonZeroCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The given capacity must be non-zero!")
    }
}

/// A power-of-two capacity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PowerOfTwoCapacity {
    inner: NonZeroUsize,
}

impl PowerOfTwoCapacity {
    /// Construct a new [`PowerOfTwoCapacity`], assuming it is valid.
    ///
    /// This is an `unsafe` function; if the given value is not a power of two,
    /// undefined behaviour will occur.
    pub unsafe fn new_unchecked(value: usize) -> Self {
        // SAFETY: The caller checks for us that the value is a power of two.
        Self { inner: NonZeroUsize::new_unchecked(value) }
    }
}

impl Capacity for PowerOfTwoCapacity {
    fn from_ct<const N: usize>() -> Self {
        struct Check<const N: usize> { _phd: [(); N] }
        impl<const N: usize> Check<N> {
            const IS_VALID: () = assert!(N.is_power_of_two(),
                "The given capacity value must be a power of two!");
        }

        let () = Check::<N>::IS_VALID;
        // SAFETY: We just asserted that 'N' is a power of two.
        unsafe { Self::new_unchecked(N) }
    }
}

impl From<PowerOfTwoCapacity> for NonZeroUsize {
    fn from(value: PowerOfTwoCapacity) -> Self {
        value.inner
    }
}

impl TryFrom<usize> for PowerOfTwoCapacity {
    type Error = PowerOfTwoCapacityError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value.is_power_of_two() {
            // SAFETY: We just checked that the value is a power of two.
            Ok(unsafe { Self::new_unchecked(value) })
        } else {
            Err(PowerOfTwoCapacityError)
        }
    }
}

impl fmt::Display for PowerOfTwoCapacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/// An error in constructing a [`PowerOfTwoCapacity`].
#[derive(Clone, Debug)]
pub struct PowerOfTwoCapacityError;

impl fmt::Display for PowerOfTwoCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The given capacity must be a power of two!")
    }
}

/// A power-of-two capacity storing the mask.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaskingCapacity {
    inner: usize,
}

impl MaskingCapacity {
    /// Construct a new [`MaskingCapacity`], assuming it is valid.
    ///
    /// This is an `unsafe` function; if the given value is not a power of two,
    /// undefined behaviour will occur.
    pub unsafe fn new_unchecked(value: usize) -> Self {
        // SAFETY: The caller checks for us that the value is a power of two.
        Self { inner: value - 1 }
    }

    /// Get the capacity mask.
    ///
    /// This can be used to mask out-of-bound indices into range efficiently,
    /// using the binary AND operator.
    pub fn mask(&self) -> usize {
        self.inner
    }
}

impl Capacity for MaskingCapacity {
    fn from_ct<const N: usize>() -> Self {
        struct Check<const N: usize> { _phd: [(); N] }
        impl<const N: usize> Check<N> {
            const IS_VALID: () = assert!(N.is_power_of_two(),
                "The given capacity value must be a power of two!");
        }

        let () = Check::<N>::IS_VALID;
        // SAFETY: We just asserted that 'N' is a power of two.
        unsafe { Self::new_unchecked(N) }
    }
}

impl From<MaskingCapacity> for NonZeroUsize {
    fn from(value: MaskingCapacity) -> Self {
        // SAFETY: The 'MaskingCapacity' was created using a power of two that
        // was decremented to form the mask; so incrementing is well-defined.
        unsafe { NonZeroUsize::new_unchecked(value.inner + 1) }
    }
}

impl TryFrom<usize> for MaskingCapacity {
    type Error = PowerOfTwoCapacityError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value.is_power_of_two() {
            // SAFETY: We just checked that the value is a power of two.
            Ok(unsafe { Self::new_unchecked(value) })
        } else {
            Err(PowerOfTwoCapacityError)
        }
    }
}

impl fmt::Display for MaskingCapacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&NonZeroUsize::from(*self), f)
    }
}
