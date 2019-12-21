pub trait OptionZip {
    fn zip<T, U>(v1: Option<T>, v2: Option<U>) -> Option<(T, U)> {
        match v1 {
            Some(t) => match v2 {
                Some(u) => Some((t, u)),
                None => None,
            },
            None => None,
        }
    }
}

#[doc(hidden)]
impl OptionZip for core::option::Option<()> {}
