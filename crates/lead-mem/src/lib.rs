pub mod pointer;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Object State                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ObjectState {
    Created,
    Initialized,
    Disposed
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Macros                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! singleton_mut {
    (func: $func: ident, $name: ident, $type: tt, $init: expr) => {
        static mut $name: Option<$type> = None;

        pub fn $func() -> $crate::pointer::MutPointer<$type> {
            unsafe {
                if let Some(value) = &mut $name {
                    return $crate::pointer::MutPointer::new(value);
                } else {
                    $name = Some($init);
                    return $func();
                }
            }
        }
    };
    ($name: ident, $type: tt, $init: expr) => {
        static mut $name: Option<$type> = None;

        pub fn get() -> $crate::pointer::MutPointer<$type> {
            unsafe {
                if let Some(value) = &mut $name {
                    return $crate::pointer::MutPointer::new(value);
                } else {
                    $name = Some($init);
                    return get();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! singleton {
    (func: $func: ident, $name: ident, $type: tt, $init: expr) => {
        static mut $name: Option<$type> = None;

        pub fn $func() -> $crate::pointer::Pointer<$type> {
            unsafe {
                if let Some(value) = &mut $name {
                    return $crate::pointer::Pointer::new(value);
                } else {
                    $name = Some($init);
                    return $func();
                }
            }
        }
    };
    ($name: ident, $type: tt, $init: expr) => {
        static mut $name: Option<$type> = None;

        pub fn get() -> $crate::pointer::Pointer<$type> {
            unsafe {
                if let Some(value) = &mut $name {
                    return $crate::pointer::Pointer::new(value);
                } else {
                    $name = Some($init);
                    return get();
                }
            }
        }
    };
}
