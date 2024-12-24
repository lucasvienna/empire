macro_rules! fail {
    ($expr:expr) => {
        return Err(::std::convert::From::from($expr))
    };
}

macro_rules! try_read {
    ($expr:expr, $val:expr) => {{
        if $expr? != $val {
            fail!((ErrorKind::UnreadBytesError, "not all bytes were read"));
        }
    }};
}
