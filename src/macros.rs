// Copyright © 2018–2019 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

macro_rules! if_signed {
    (Signed; $($rem:tt)+) => {
        $($rem)+
    };
    (Unsigned; $($rem:tt)+) => {};
}

macro_rules! if_unsigned {
    (Signed; $($rem:tt)+) => {};
    (Unsigned; $($rem:tt)+) => {
        $($rem)+
    };
}

macro_rules! if_signed_unsigned {
    (Signed, $signed:expr, $unsigned:expr) => {
        $signed
    };
    (Unsigned, $signed:expr, $unsigned:expr) => {
        $unsigned
    };
    ($Signedness:tt, $signed:expr, $unsigned:expr,) => {
        if_signed_unsigned!($Signedness, $signed, $unsigned)
    };
}

macro_rules! if_signed_else_empty_str {
    (Signed, $($signed:tt)*) => {
        concat!($($signed)*)
    };
    (Unsigned, $($signed:tt)*) => {
        ""
    };
}

macro_rules! doc_comment {
    ($comment:expr; $($tt:tt)*) => {
        #[doc = $comment]
        $($tt)*
    };
}

macro_rules! comment {
    ($($comment:expr),*; $($tt:tt)*) => {
        doc_comment! {
            concat!($($comment),*);
            $($tt)*
        }
    };
}

macro_rules! delegate {
    ($($comment:expr),*; $Fixed:ident($Inner:ty) => fn $method:ident()) => {
        doc_comment! {
            concat!($($comment),*);
            #[inline]
            pub fn $method() -> $Fixed<Frac> {
                Self::from_bits(<$Inner>::$method())
            }
        }
    };
    ($($comment:expr),*; $Fixed:ident($Inner:ty) => fn $method:ident(self)) => {
        doc_comment! {
            concat!($($comment),*);
            #[inline]
            pub fn $method(self) -> $Fixed<Frac> {
                Self::from_bits(<$Inner>::$method(self.to_bits()))
            }
        }
    };
    ($($comment:expr),*; $Fixed:ident($Inner:ty) => fn $method:ident(self) -> $ret_ty:ty) => {
        doc_comment! {
            concat!($($comment),*);
            #[inline]
            pub fn $method(self) -> $ret_ty {
                <$Inner>::$method(self.to_bits())
            }
        }
    };
    (
        $($comment:expr),*;
        $Fixed:ident($Inner:ty) => fn $method:ident(self, $param:ident: $param_ty:ty)
    ) => {
        doc_comment! {
            concat!($($comment),*);
            #[inline]
            pub fn $method(self, $param: $param_ty) -> $Fixed<Frac> {
                Self::from_bits(<$Inner>::$method(self.to_bits(), $param))
            }
        }
    };
}