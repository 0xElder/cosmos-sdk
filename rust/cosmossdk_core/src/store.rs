use crate::context::{Context};

pub struct KVStoreService {
    store_id: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl KVStoreService {
    pub fn open(&self, ctx: &Context) -> KVStore {
        // let store_id = unsafe { cosmossdk_store_open(self.store_id, ctx.id) };
        // KVStore { store_id }
        todo!()
    }

    pub fn open_mut(&self, ctx: &mut Context) -> KVStore {
        // let store_id = unsafe { cosmossdk_store_open(self.store_id, ctx.id) };
        // KVStore { store_id }
        todo!()
    }
}

pub struct KVStore {
    store_id: usize,
}

impl KVStore {
    pub fn has(&self, key: &[u8]) -> bool {
        // unsafe { cosmossdk_store_has(self.store_id, key.as_ptr() as *mut u8, key.len()) == 0 }
        todo!()
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        // let value = vec![0u8; 0x10000];
        // let mut value_len = value.len();
        // let res = unsafe {
        //     cosmossdk_store_get(
        //         self.store_id,
        //         key.as_ptr(),
        //         key.len(),
        //         value.as_ptr() as *mut u8,
        //         &mut value_len,
        //     )
        // };
        // if res == 0 {
        //     Some(value[..value_len].to_vec())
        // } else {
        //     None
        // }
        todo!()
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        // unsafe {
        //     cosmossdk_store_set(
        //         self.store_id,
        //         key.as_ptr(),
        //         key.len(),
        //         value.as_ptr(),
        //         value.len(),
        //     )
        // };
        todo!()
    }

    pub fn delete(&mut self, key: &[u8]) {
        // unsafe { cosmossdk_store_delete(self.store_id, key.as_ptr() as *mut u8, key.len()) };
        todo!()
    }
}

impl Drop for KVStore {
    fn drop(&mut self) {
        // unsafe { cosmossdk_store_close(self.store_id) };
        todo!()
    }
}

// pub(crate) static mut cosmossdk_store_open: extern "C" fn(usize, usize) -> usize = |_, _| 0;
// pub(crate) static mut cosmossdk_store_has: extern "C" fn(usize, *mut u8, usize) -> u32 = |_, _, _| 0;
// pub(crate) static mut cosmossdk_store_get: extern "C" fn(usize, *const u8, usize, *mut u8, *mut usize) -> i32 = |_, _, _, _, _| 0;
// pub(crate) static mut cosmossdk_store_set: extern "C" fn(usize, *const u8, usize, *const u8, usize) -> i32 = |_, _, _, _, _| 0;
// pub(crate) static mut cosmossdk_store_delete: extern "C" fn(usize, *mut u8, usize) -> i32 = |_, _, _| 0;
// pub(crate) static mut cosmossdk_store_close: extern "C" fn(usize) -> i32 = |_| 0;

// extern "C" {
//     fn cosmossdk_store_open(module_id: usize, ctx_id: usize) -> usize;
//
//     fn cosmossdk_store_has(store_id: usize, key_ptr: *mut u8, key_len: usize) -> u32;
//
//     fn cosmossdk_store_get(
//         store_id: usize,
//         key_ptr: *const u8,
//         key_len: usize,
//         value_ptr: *mut u8,
//         value_len: *mut usize
//     ) -> i32;
//
//     fn cosmossdk_store_set(
//         store_id: usize,
//         key_ptr: *const u8,
//         key_len: usize,
//         value_ptr: *const u8,
//         value_len: usize
//     ) -> i32;
//
//     fn cosmossdk_store_delete(store_id: usize, key_ptr: *mut u8, key_len: usize) -> i32;
//
//     fn cosmossdk_store_close(store_id: usize) -> i32;
// }

// #[no_mangle]
// extern fn __entry() {
//     unsafe { cosmossdk_store_close(0); }
// }
