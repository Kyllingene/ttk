use std::{fmt, str::FromStr};

use linoleum::{EditResult, Editor};

pub enum Query<T, E> {
    Ok(T),
    Cancel,
    Quit,
    Err(E),
}

pub enum FailedQuery<E> {
    Cancel,
    Quit,
    Err(E),
}

pub fn line(level: usize, prompt: impl fmt::Display) -> Query<String, Never> {
    crate::print::prompt(level, prompt);
    match Editor::new(" > ").read().unwrap_or_else(|e| {
        crate::print::err(level, e);
        drop(cod::guard::Reset);
        std::process::exit(1);
    }) {
        EditResult::Ok(s) => Query::Ok(s),
        EditResult::Cancel => Query::Cancel,
        EditResult::Quit => Query::Quit,
    }
}

pub fn item<T: FromStr>(level: usize, prompt: impl fmt::Display) -> Query<T, T::Err> {
    line(level, prompt)
        .map_err(|e| match e {})
        .and_then(|s| T::from_str(&s).into())
}

impl<T, E: fmt::Display> Query<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Cancel => crate::exit::abort("Canceled"),
            Self::Quit => crate::exit::abort("Quit"),
            Self::Err(e) => crate::exit::abort(e),
        }
    }

    pub fn expect(self, msg: impl fmt::Display) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Cancel => crate::exit::abort("Canceled"),
            Self::Quit => crate::exit::abort("Quit"),
            Self::Err(e) => crate::exit::abort(format_args!("{msg}: {e}")),
        }
    }

    pub fn expect_all(
        self,
        cancel_msg: impl fmt::Display,
        quit_msg: impl fmt::Display,
        err_msg: impl fmt::Display,
    ) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Cancel => crate::exit::abort(cancel_msg),
            Self::Quit => crate::exit::abort(quit_msg),
            Self::Err(e) => crate::exit::abort(format_args!("{err_msg}: {e}")),
        }
    }
}

impl<T, E> Query<T, E> {
    pub fn unwrap_or(self, or: T) -> T {
        if let Self::Ok(t) = self { t } else { or }
    }

    pub fn unwrap_or_else(self, or: impl FnOnce(FailedQuery<E>) -> T) -> T {
        match self {
            Self::Ok(t) => t,
            Self::Cancel => or(FailedQuery::Cancel),
            Self::Quit => or(FailedQuery::Quit),
            Self::Err(e) => or(FailedQuery::Err(e)),
        }
    }

    pub fn as_ref(&self) -> Query<&T, &E> {
        match self {
            Self::Ok(t) => Query::Ok(t),
            Self::Cancel => Query::Cancel,
            Self::Quit => Query::Quit,
            Self::Err(e) => Query::Err(e),
        }
    }

    pub fn map<Q>(self, f: impl FnOnce(T) -> Q) -> Query<Q, E> {
        match self {
            Self::Ok(t) => Query::Ok(f(t)),
            Self::Cancel => Query::Cancel,
            Self::Quit => Query::Quit,
            Self::Err(e) => Query::Err(e),
        }
    }

    pub fn and_then<Q>(self, f: impl FnOnce(T) -> Query<Q, E>) -> Query<Q, E> {
        match self {
            Self::Ok(t) => f(t),
            Self::Cancel => Query::Cancel,
            Self::Quit => Query::Quit,
            Self::Err(e) => Query::Err(e),
        }
    }

    pub fn map_err<G>(self, f: impl FnOnce(E) -> G) -> Query<T, G> {
        match self {
            Self::Ok(t) => Query::Ok(t),
            Self::Cancel => Query::Cancel,
            Self::Quit => Query::Quit,
            Self::Err(e) => Query::Err(f(e)),
        }
    }
}

impl<T, E> From<Result<T, E>> for Query<T, E> {
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(t) => Query::Ok(t),
            Err(t) => Query::Err(t),
        }
    }
}

impl<T, E> From<Query<T, E>> for Option<T> {
    fn from(q: Query<T, E>) -> Self {
        match q {
            Query::Ok(t) => Some(t),
            _ => None,
        }
    }
}

mod never {
    pub trait Smugglers {
        type Handbag;
    }
    impl<Contraband> Smugglers for fn() -> Contraband {
        type Handbag = Contraband;
    }

    /// The never (`!`) type.
    ///
    /// See also [`std::convert::Infallible`].
    pub type Never = <fn() -> ! as Smugglers>::Handbag;
}

/// The never (`!`) type.
///
/// See also [`std::convert::Infallible`].
pub use never::Never;
