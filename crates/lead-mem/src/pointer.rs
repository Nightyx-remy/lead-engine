////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Mut Pointer                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct MutPointer<T> {
    ptr: *mut T
}

impl<T> MutPointer<T> {

    pub fn new(object: &mut T) -> MutPointer<T> {
        return MutPointer {
            ptr: object as *mut T
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe {
            return &*self.ptr;
        }
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe {
            return &mut *self.ptr;
        }
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Pointer                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Pointer<T> {
    ptr: *const T
}

impl<T> Pointer<T> {

    pub fn new(object: &T) -> Pointer<T> {
        return Pointer {
            ptr: object as *const T
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe {
            return &*self.ptr;
        }
    }

}

