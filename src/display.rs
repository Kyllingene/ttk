use std::fmt;

pub fn from_fn<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result>(f: F) -> FromFn<F> {
    FromFn(f)
}

pub fn from_iter<I: Clone + IntoIterator<Item: fmt::Display>, S: fmt::Display>(
    i: I,
    sep: S,
) -> FromIter<I, S> {
    FromIter(i, sep)
}

pub struct FromFn<F>(pub F);
impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Debug for FromFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Display for FromFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.0)(f)
    }
}

pub struct FromIter<I, S>(I, S);

impl<I: Clone + IntoIterator<Item: fmt::Debug>, S: fmt::Display> fmt::Debug for FromIter<I, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.0.clone().into_iter();
        let mut first = true;
        for item in iter {
            if !first {
                write!(f, "{}", self.1)?;
            }
            first = false;

            write!(f, "{item:?}")?;
        }

        Ok(())
    }
}

impl<I: Clone + IntoIterator<Item: fmt::Display>, S: fmt::Display> fmt::Display for FromIter<I, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.0.clone().into_iter();
        let mut first = true;
        for item in iter {
            if !first {
                write!(f, "{}", self.1)?;
            }
            first = false;

            write!(f, "{item}")?;
        }

        Ok(())
    }
}
