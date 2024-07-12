use std::{borrow::Cow, io};

use crate::compress::lz77::Lz77;

use super::raw;

pub struct Overlay<'a> {
    id: u32,
    base_address: u32,
    code_size: u32,
    bss_size: u32,
    ctor_start: u32,
    ctor_end: u32,
    file_id: u32,
    compressed: bool,
    data: Cow<'a, [u8]>,
}

const LZ77: Lz77 = Lz77 {};

impl<'a> Overlay<'a> {
    pub fn parse(overlay: &raw::Overlay, fat: &'a [&[u8]]) -> Self {
        Self {
            id: overlay.id,
            base_address: overlay.base_addr,
            code_size: overlay.code_size,
            bss_size: overlay.bss_size,
            ctor_start: overlay.ctor_start,
            ctor_end: overlay.ctor_end,
            file_id: overlay.file_id,
            compressed: overlay.is_compressed(),
            data: Cow::Borrowed(fat[overlay.file_id as usize]),
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn base_address(&self) -> u32 {
        self.base_address
    }

    pub fn code_size(&self) -> u32 {
        self.code_size
    }

    pub fn bss_size(&self) -> u32 {
        self.bss_size
    }

    pub fn ctor_start(&self) -> u32 {
        self.ctor_start
    }

    pub fn ctor_end(&self) -> u32 {
        self.ctor_end
    }

    pub fn file_id(&self) -> u32 {
        self.file_id
    }

    pub fn is_compressed(&self) -> bool {
        self.compressed
    }

    pub fn decompress(&mut self) {
        if !self.compressed {
            return;
        }
        self.data = LZ77.decompress(&self.data).into_vec().into();
        self.compressed = false;
    }

    pub fn compress(&mut self) -> Result<(), io::Error> {
        if self.compressed {
            return Ok(());
        }
        self.data = LZ77.compress(&self.data, 0)?.into_vec().into();
        self.compressed = true;
        Ok(())
    }

    pub fn full_data(&self) -> &[u8] {
        &self.data
    }
}