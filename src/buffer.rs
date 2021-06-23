use std::borrow::{Borrow, BorrowMut};

#[derive(Clone)]
pub struct BufferWrapper {
    pub data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}
impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }
}
impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut u8, self.data.len() * 4)
        }
    }
}
impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.data.as_slice()
    }
}
impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.data.as_mut_slice()
    }
}

impl From<BufferWrapper> for iced::image::Handle {
    fn from(b: BufferWrapper) -> Self {
        // plotters use BGRX pixel
        // iced::image::Handle require BGRA pixel
        // Convert between them.
        let mut bytes = {
            let bytes: &[u8] = b.borrow();
            bytes.to_vec()
        };
        bytes.chunks_exact_mut(4).for_each(|bgrx| {
            if let [b, g, r, x] = &mut bgrx[0..4] {
                *x = 255;
            } else {
                unreachable!()
            }
        });
        iced::image::Handle::from_pixels(b.width, b.height, bytes)
    }
}
