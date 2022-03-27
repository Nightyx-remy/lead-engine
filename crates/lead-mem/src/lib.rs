pub mod pointer;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ObjectState {
    Created,
    Initialized,
    Disposed
}

#[macro_export]
macro_rules! singleton {
    (func: $func: ident, $name: ident, $type: tt, $init: expr) => {
        static mut $name: Option<$type> = None;

        pub fn $func() -> Pointer<$type> {
            unsafe {
                if let Some(value) = &mut $name {
                    return Pointer::new(value);
                } else {
                    $name = Some($init);
                    return $func();
                }
            }
        }
    };
    ($name: ident, $type: tt, $init: expr) => {
        static mut $name: Option<$type> = None;

        pub fn get() -> Pointer<$type> {
            unsafe {
                if let Some(value) = &mut $name {
                    return Pointer::new(value);
                } else {
                    $name = Some($init);
                    return get();
                }
            }
        }
    };
}
