//! This crate provides `inspect_or_log`, `unwrap_or_log` and `unwrap_or_default_log` methods
//! on all types implemented [`Try`].
//! They are useful if you want to log the error value.
//!
//! Different log levels are chosen by calling different methods.
//!
//! | method                   | level |
//! | ------------------------ | ----- |
//! | `inspect_or_log*`        | info  |
//! | `unwrap_or_default_log*` | warn  |
//! | `unwrap_or_log*`         | error |

#![feature(try_trait_v2)]
#![warn(missing_docs)]

use log::{error, info, warn};
use std::{
    fmt::{Debug, Display},
    ops::{ControlFlow, Try},
};

/// This trait provides the or-log methods.
pub trait TryLog<T, R: Debug> {
    /// Log with [`log::Level::Info`] if the value is [`Err`] or [`None`].
    fn inspect_or_log(self, msg: &str) -> Self;

    /// Log with [`log::Level::Info`] if the value is [`Err`] or [`None`].
    ///
    /// The log message is returned by `f`.
    fn inspect_or_log_with<M: Display>(self, f: impl FnOnce() -> M) -> Self;

    /// Log with [`log::Level::Warn`] if the value is [`Err`] or [`None`],
    /// and return a default value of `T`.
    fn unwrap_or_default_log(self, msg: &str) -> T
    where
        T: Default;

    /// Log with [`log::Level::Warn`] if the value is [`Err`] or [`None`],
    /// and return a default value of `T`.
    ///
    /// The log message is returned by `f`.
    fn unwrap_or_default_log_with<M: Display>(self, f: impl FnOnce() -> M) -> T
    where
        T: Default;

    /// Log with [`log::Level::Error`] if the value is [`Err`] or [`None`],
    /// and panic with the same error message.
    fn unwrap_or_log(self, msg: &str) -> T;

    /// Log with [`log::Level::Error`] if the value is [`Err`] or [`None`],
    /// and panic with the same error message.
    ///
    /// The log message is returned by `f`.
    fn unwrap_or_log_with<M: Display>(self, f: impl FnOnce() -> M) -> T;
}

#[inline(always)]
fn inspect_or_and<T: Try>(t: T, f: impl FnOnce(&T::Residual)) -> T {
    match t.branch() {
        ControlFlow::Continue(v) => T::from_output(v),
        ControlFlow::Break(e) => {
            f(&e);
            T::from_residual(e)
        }
    }
}

#[inline(always)]
fn unwrap_or_default_and<T: Try>(t: T, f: impl FnOnce(&T::Residual)) -> T::Output
where
    T::Output: Default,
{
    match t.branch() {
        ControlFlow::Continue(v) => v,
        ControlFlow::Break(e) => {
            f(&e);
            Default::default()
        }
    }
}

#[inline(always)]
fn unwrap_or_and<T: Try>(t: T, f: impl FnOnce(&T::Residual) -> String) -> T::Output {
    match t.branch() {
        ControlFlow::Continue(v) => v,
        ControlFlow::Break(e) => {
            let msg = f(&e);
            error!("{}", msg);
            panic!("{}", msg)
        }
    }
}

impl<T, R: Debug, Tr: Try<Output = T, Residual = R>> TryLog<T, R> for Tr {
    #[inline(always)]
    fn inspect_or_log(self, msg: &str) -> Self {
        inspect_or_and(self, |e| info!("{msg}: {e:?}"))
    }

    #[inline(always)]
    fn inspect_or_log_with<M: Display>(self, f: impl FnOnce() -> M) -> Self {
        inspect_or_and(self, |e| info!("{}: {e:?}", f()))
    }

    #[inline(always)]
    fn unwrap_or_default_log(self, msg: &str) -> T
    where
        T: Default,
    {
        unwrap_or_default_and(self, |e| warn!("{msg}: {e:?}"))
    }

    #[inline(always)]
    fn unwrap_or_default_log_with<M: Display>(self, f: impl FnOnce() -> M) -> T
    where
        T: Default,
    {
        unwrap_or_default_and(self, |e| warn!("{}: {e:?}", f()))
    }

    #[inline(always)]
    fn unwrap_or_log(self, msg: &str) -> T {
        unwrap_or_and(self, |e| format!("{msg}: {e:?}"))
    }

    #[inline(always)]
    fn unwrap_or_log_with<M: Display>(self, f: impl FnOnce() -> M) -> T {
        unwrap_or_and(self, |e| format!("{}: {e:?}", f()))
    }
}
