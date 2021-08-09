/*
 * Module data-structs
 * 
 * Implementations of in-memory data structures such as tables
 */

pub mod row;
pub mod table;
pub mod page;
pub mod constants;

use std::convert::{TryInto};
use std::fmt;

use constants::*;

/****************/
/* Global types */
/****************/

pub trait Serializable {
    fn serialize(&self) -> Vec<Option<u8>>;
    fn deserialize(arr: &[Option<u8>]) -> Self;
}

/********************/
/* Global functions */
/********************/

/// Returns the valid `u8` elements from an array of `Option<u8>`.
/// 
/// Elements are read and pushed into a `Vec<u8>` until a `None` is read or the end 
/// of the provided range is reached. 
/// 
/// # Parameters
/// 
/// * `arr` - the array you want to read from
/// * `offset` - where in the array you want to start reading from
/// * `size` - the maximum number of elements you want to read
/// 
/// # Returns
/// 
/// A `Vec<u8>` containing the elements.
pub fn u8_from_option_u8(arr: &[Option<u8>], offset: usize, size: usize)
-> Vec<u8>
{
    let mut bytes = Vec::with_capacity(size);

    for i in offset..(offset+size) {
        match arr[i] {
            Some(b) => { bytes.push(b); }
            None => { break; }
        }
    }

    return bytes;
}

fn copy_vec_bytes_to_vec(target: &mut Vec<Option<u8>>, bytes: Vec<Option<u8>>, 
                            offset: usize, size: usize)
-> ()
{
    let mut vec_bytes_unwrapped: Vec<u8> = Vec::new();
    for byte in bytes {
        match byte {
            Some(b) => { vec_bytes_unwrapped.push(b); }
            None => { break; }
        }
    }

    copy_bytes_to_vec(target, vec_bytes_unwrapped.as_slice(), offset, size);
}

/*******************/
/* Local functions */
/*******************/

fn copy_bytes_to_vec(target: &mut Vec<Option<u8>>, bytes: &[u8], offset: usize, size: usize)
{
    for (ct, b) in bytes.iter().enumerate() {
        if ct >= size { break; }
        target[offset + ct] = Some(*b);
    }
}