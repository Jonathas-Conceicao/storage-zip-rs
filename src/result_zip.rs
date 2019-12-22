macro_rules! zip_fn {
    ($f:ident, [ $($arg:ident: $type:ident), * ]) => {
        fn $f<$($type,)* E>($($arg: core::result::Result<$type, E>,)*)
                             -> core::result::Result<($($type,)*), E> {
            match_args!($($arg),* => $($arg),*)
        }
    };
}

macro_rules! match_args {
    ($opt:ident => $($arg:ident),*) => {
        match $opt {
            core::result::Result::Ok($opt) => core::result::Result::Ok(($($arg),*)),
            core::result::Result::Err(e) => core::result::Result::Err(e)
        }
    };

    ($opt:ident, $($opts:ident), + => $($arg:ident),*) => {
        match $opt {
            core::result::Result::Ok($opt) => match_args!($($opts),* => $($arg),*),
            core::result::Result::Err(e) => core::result::Result::Err(e),
        }
    };
}

/// Trait for using zip function on [Result](core::result::Result) type, import
/// the trait and call it using `Result::zip` or any variant with more arguments
pub trait ResultZip {
    zip_fn!(zip, [v1: T1, v2: T2]);

    zip_fn!(zip3, [v1: T1, v2: T2, v3: T3]);

    zip_fn!(zip4, [v1: T1, v2: T2, v3: T3, v4: T4]);

    zip_fn!(zip5, [v1: T1, v2: T2, v3: T3, v4: T4, v5: T5]);
}

#[doc(hidden)]
impl ResultZip for core::result::Result<(), ()> {}
