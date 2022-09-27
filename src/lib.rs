#![feature(try_trait_v2)]

use log::{error, info, warn};
use std::{
    fmt::Debug,
    ops::{ControlFlow, Try},
};

pub trait TryLog<T, R: Debug> {
    fn inspect_or_log(self, msg: &str) -> Self;

    fn inspect_or_log_with(self, f: impl FnOnce() -> String) -> Self;

    fn unwrap_or_default_log(self, msg: &str) -> T
    where
        T: Default;

    fn unwrap_or_default_log_with(self, f: impl FnOnce() -> String) -> T
    where
        T: Default;

    fn unwrap_or_log(self, msg: &str) -> T;

    fn unwrap_or_log_with(self, f: impl FnOnce() -> String) -> T;
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
    fn inspect_or_log_with(self, f: impl FnOnce() -> String) -> Self {
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
    fn unwrap_or_default_log_with(self, f: impl FnOnce() -> String) -> T
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
    fn unwrap_or_log_with(self, f: impl FnOnce() -> String) -> T {
        unwrap_or_and(self, |e| format!("{}: {e:?}", f()))
    }
}
