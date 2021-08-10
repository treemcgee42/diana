# Design

## Byte representation

### In C
In C, it is very easy to work with individual bytes via pointers and pointer arithmetic.
In Rust, this is `unsafe`. One reason it is unsafe is because the byte a pointer 
points to may not have been initialized. In the best case, the value of the pointer 
is `NULL`, but it might not be. 

It is also easy in C to allocate a contiguous block of bytes, e.g. 
`void *block = malloc(size)`. Of course, explicit calls to `malloc` are `unsafe` in Rust.
Pointer arithmetic is also out of the question, as we don't work with pointers. 

### In Rust
Bytes in rust are represented by the type `u8`. We can add in the `NULL` functionality 
by using `Option<u8>` instead (where `NULL` becomes `None`).

We have elected to allocate a contiguous block of bytes with a `Vec`, e.g. 
```
block = Vec::<Option<u8>>::with_capacity(size);
```
This gives us heap allocation along with the robust functionality of the `Vec` type. 

We recommend initializing the vector to `None` immediately. This will allow one to index 
into the vector at any point within the size. 

One danger is that `Vec` is a dynamically allocated structure, so if we were to exceed the 
capacity of it, it would double in size. But this is often not what we want. In a database 
we want efficient, low level handling of memory. For example, we want to store data in 
pages, which are typically 4kb. This is because it is efficient in terms of the hardware and 
operating system to deal with blocks of memory that size. It would defeat the purpose of a 
page if it were to represent more that 4kb of memory.

It is the responsibility of the programmer to not exceed the capacity of a vector of bytes. 
We recommend implemented checks upon insertion. For example, if we were designed a `Page`, 
which was a `struct` whose only field was a `Vec<Option<u8>>`, then we might implement an 
`insert()` method, which contains a check that we are not exceeding the size (e.g. 4kb) of
the vector before writing to it.
