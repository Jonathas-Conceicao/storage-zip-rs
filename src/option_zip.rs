macro_rules! zip_fn {
    ($f:ident, [ $($arg:ident: $type:ident), * ]) => {
        fn $f<$($type,)*>($($arg: core::option::Option<$type>,)*)
                          -> core::option::Option<($($type,)*)> {
            match_args!($($arg),* => $($arg),*)
        }
    };
}

macro_rules! match_args {
    ($opt:ident => $($arg:ident),*) => {
        match $opt {
            core::option::Option::Some($opt) => core::option::Option::Some(($($arg),*)),
            core::option::Option::None => core::option::Option::None
        }
    };

    ($opt:ident, $($opts:ident), + => $($arg:ident),*) => {
        match $opt {
            core::option::Option::Some($opt) => match_args!($($opts),* => $($arg),*),
            core::option::Option::None => core::option::Option::None,
        }
    };
}

pub trait OptionZip {
    zip_fn!(zip, [v1: T1, v2: T2]);

    zip_fn!(zip3, [v1: T1, v2: T2, v3: T3]);

    zip_fn!(zip4, [v1: T1, v2: T2, v3: T3, v4: T4]);

    zip_fn!(zip5, [v1: T1, v2: T2, v3: T3, v4: T4, v5: T5]);
}

#[doc(hidden)]
impl OptionZip for core::option::Option<()> {}
