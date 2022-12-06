//! Provides another implementations.
//!
//! This mod uses macros instead of functions.
//! It can make use of macro expansions.

use std::{
    fmt::Debug,
    ops::{ControlFlow, Try},
};

#[doc(hidden)]
pub struct __TryWrapper<T: Try>(pub T);

impl<T: Try> __TryWrapper<T> {
    #[inline(always)]
    pub fn branch(self) -> __ControlFlowWrapper<T> {
        match self.0.branch() {
            ControlFlow::Continue(v) => __ControlFlowWrapper::Continue(__ControlFlowContinue(v)),
            ControlFlow::Break(e) => __ControlFlowWrapper::Break(__ControlFlowBreak(e)),
        }
    }
}

#[doc(hidden)]
pub enum __ControlFlowWrapper<T: Try> {
    Continue(__ControlFlowContinue<T>),
    Break(__ControlFlowBreak<T>),
}

#[doc(hidden)]
pub struct __ControlFlowContinue<T: Try>(pub T::Output);

impl<T: Try> __ControlFlowContinue<T> {
    #[inline(always)]
    pub fn restore(self) -> T {
        T::from_output(self.0)
    }
}

#[doc(hidden)]
pub struct __ControlFlowBreak<T: Try>(pub T::Residual);

impl<T: Try> __ControlFlowBreak<T> {
    #[inline(always)]
    pub fn restore(self) -> T {
        T::from_residual(self.0)
    }
}

impl<T: Try> Debug for __ControlFlowBreak<T>
where
    T::Residual: Debug,
{
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

/// Log with [`log::Level::Info`] if the value is [`Err`] or [`None`].
pub macro inspect_or_log($e:expr, $t:tt) {
    match $crate::macros::__TryWrapper($e).branch() {
        __ControlFlowWrapper::Continue(v) => v.restore(),
        __ControlFlowWrapper::Break(e) => {
            $crate::__log::info!("{}: {e:?}", $t);
            e.restore()
        }
    }
}

/// Log with [`log::Level::Warn`] if the value is [`Err`] or [`None`],
/// and return a default value of `T`.
pub macro unwrap_or_default_log($e:expr, $t:tt) {
    match $crate::macros::__TryWrapper($e).branch() {
        __ControlFlowWrapper::Continue(v) => v.0,
        __ControlFlowWrapper::Break(e) => {
            $crate::__log::warn!("{}: {e:?}", $t);
            Default::default()
        }
    }
}

/// Log with [`log::Level::Error`] if the value is [`Err`] or [`None`],
/// and panic with the same error message.
pub macro unwrap_or_log($e:expr, $t:tt) {
    match $crate::macros::__TryWrapper($e).branch() {
        __ControlFlowWrapper::Continue(v) => v.0,
        __ControlFlowWrapper::Break(e) => {
            let msg = format!("{}: {e:?}", $t);
            $crate::__log::error!("{}", msg);
            panic!("{}", msg)
        }
    }
}
