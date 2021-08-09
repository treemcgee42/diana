/*
 * table.rs
 * 
 * Implementation of Table
 */

use crate::data_structs::{
    *,
    page::*,
};

pub struct Table {
    pub num_rows: usize,
    pub pages: [Option<Page>; TABLE_MAX_PAGES],
}

const INIT: Option<Page> = None;

impl Table {
    pub fn new() -> Table
    {
        Table {
            num_rows: 0,
            pages: [INIT; TABLE_MAX_PAGES],
        }
    }

    pub fn row_slot(&mut self, row_num: usize) -> (&mut Page, usize)
    {
        let page_num: usize = row_num / ROWS_PER_PAGE;
        let page = &mut self.pages[page_num];
        match page {
            None => { *page = Some(Page::new()); }
            _ => ()
        }

        let row_offset: usize = row_num % ROWS_PER_PAGE;
        let byte_offset: usize = row_offset * ROW_SIZE;

        return (page.as_mut().unwrap(), byte_offset);
    }
}