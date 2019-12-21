pub trait ResultZip {
    fn zip<T, U, E>(v1: Result<T, E>, v2: Result<U, E>) -> Result<(T, U), E> {
        match v1 {
            Ok(t) => match v2 {
                Ok(u) => Ok((t, u)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

#[doc(hidden)]
impl ResultZip for core::result::Result<(), ()> {}
