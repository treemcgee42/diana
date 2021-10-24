/*
 * page.rs
 * 
 * Implementation of Page
 */

use crate::data_structs::*;

/* Unsafe implementation */
// TODO: check alignment safety assumptions for all unsafe calls
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{NonNull, copy_nonoverlapping};

pub struct Page {
    ptr: NonNull<u8>,
    size: usize,
    layout: Layout,
}

impl Page {
    pub fn new() -> Page
    {
        let layout = Layout::array::<u8>(PAGE_SIZE).expect("There was an arithmetic overflow.");
        // SAFETY: the layout is ensured to be nonzero size, memory may be unitialized
        // but the pointer will not be NULL
        let ptr = NonNull::new(unsafe { alloc(layout) }).expect("Could not allocate the pointer.");

        Page {
            ptr,
            size: PAGE_SIZE,
            layout,
        }
    }

    pub fn insert(&mut self, bytes: Vec<u8>, offset: usize, size: usize)
    {
        let vec_ptr = bytes.as_ptr();

        assert!(offset < (self.size - size));
        assert!(size <= bytes.len());

        unsafe {
            // SAFETY: we ensure the starting and resulting pointer are in bounds and
            // the offset does not overflow an isize
            let offset_page_ptr = (self.ptr.as_ptr()).offset(offset as isize);
            // SAFETY: we ensure the source vector is valid for reading by checking it is 
            // at least as large as the amount we want to write and the safety check for
            // the offset also ensures the destination is valid for a write of length size
            copy_nonoverlapping(vec_ptr, offset_page_ptr, size);
        }
    }
}

impl Drop for Page {
    fn drop(&mut self)
    {
        unsafe {
            dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn mypage_insert_test()
    {
        let mut page = Page::new();

        let bytes: Vec<u8> = vec![1, 2, 3];
        page.insert(bytes, 0, 3);
    }
}