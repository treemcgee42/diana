/*
 * page.rs
 * 
 * Implementation of Page
 */

use crate::data_structs::*;

pub struct Page {
    pub bytes: Vec<Option<u8>>,
}

impl Page {
    pub fn new() -> Page
    {
        Page {
            bytes: (0..PAGE_SIZE).map(|_| None).collect(),
        }
    }

    pub fn insert(&mut self, bytes: Vec<Option<u8>>, offset: usize, size: usize)
    {
        self.bytes.splice(offset..(offset+size), bytes);
    }

    pub fn get_slice(&self, offset: usize, size: usize) -> &[Option<u8>]
    {
        &self.bytes[offset..(offset+size)]
    }
}