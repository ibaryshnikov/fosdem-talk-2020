
use std::mem::ManuallyDrop;

#[no_mangle]
extern "C" fn alloc_array(len: i32) -> i32 {
    let array = ManuallyDrop::new(vec![0; len as usize]);
    array.as_ptr() as i32
}

#[no_mangle]
extern "C" fn sum_array(ptr: *const i32, len: i32) -> i32 {
    let array = unsafe {
        std::slice::from_raw_parts(ptr, len as usize)
    };
    array.iter().sum::<i32>()
}

