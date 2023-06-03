use std::{ops::Range, ptr};

pub struct GapBuffer<T> {
    storage: Vec<T>,
    gap: Range<usize>,
}

impl<T> GapBuffer<T> {
    pub fn new() -> GapBuffer<T> {
        GapBuffer {
            storage: Vec::new(),
            gap: 0..0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    pub fn len(&self) -> usize {
        self.capacity() - self.gap.len()
    }

    pub fn position(&self) -> usize {
        self.gap.start
    }

    /// Return a pointer to the `index`th element of the underlying storage, regardless of the
    /// gap
    ///
    /// Safety: `index` must be a valid index into `self.storage`.
    unsafe fn space(&self, index: usize) -> *const T {
        self.storage
            .as_ptr()
            .offset(index as isize)
    }

    /// Return a pointer to the `index`th element of the underlying storage, regardless of the
    /// gap
    ///
    /// Safety: `index` must be a valid index into `self.storage`.
    unsafe fn space_mut(&mut self, index: usize) -> *mut T {
        self.storage
            .as_mut_ptr()
            .offset(index as isize)
    }

    /// Return the offset in the buffer of the `index`th element, taking the gap into account.
    /// this does not check wether index is in range, but it never returns an index in the gap.
    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.len()
        }
    }

    /// Return a reference to the `index`th element, or `None` if `index` is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = self.index_to_raw(index);

        if raw < self.capacity() {
            unsafe { Some(&*self.space(raw)) }
        } else {
            None
        }
    }

    /// Set the current insertion position to `pos`
    /// If `pos` is out of bounds, panic.
    pub fn set_potision(&mut self, pos: usize) {
        if pos > self.len() {
            panic!("index {} out of range for GapBuffer", pos);
        }

        unsafe {
            let gap = self.gap.clone();
            if pos > gap.start {
                let distance = pos - gap.start;
                ptr::copy(
                    self.space(gap.end),
                    self.space_mut(gap.start),
                    distance,
                );
            } else if pos < gap.start {
                let distance = gap.start - pos;
                ptr::copy(
                    self.space(pos),
                    self.space_mut(gap.end - distance),
                    distance,
                );
            }
            self.gap = pos..pos + gap.len()
        }
    }

    /// Insert `elt` at the current insertion position
    /// and leave the insertion position after it
    pub fn insert(&mut self, elt: T) {
        if self.gap.len() == 0 {
            self.enlarge_gap();
        }

        unsafe {
            let index = self.gap.start;
            ptr::write(self.space_mut(index), elt);
        }

        self.gap.start += 1;
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = T>) {
        for elt in iter {
            self.insert(elt)
        }
    }

    /// Remove `elt` atfter the current insertion position
    /// and return it, or return `None` if the insertion position is at the end of the GapBuffer
    pub fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }

        let elem = unsafe { ptr::read(self.space(self.gap.end)) };

        self.gap.end += 1;
        Some(elem)
    }

    fn enlarge_gap(&mut self) {
        let mut new_capcity = self.capacity() * 2;
        if new_capcity == 0 {
            new_capcity = 4;
        }

        let mut new = Vec::with_capacity(new_capcity);
        let after_gap = self.capacity() - self.gap.end;
        let new_gap = self.gap.start..new.capacity() - after_gap;

        unsafe {
            ptr::copy_nonoverlapping(
                self.space(0),
                new.as_mut_ptr(),
                self.gap.start,
            );

            let new_gap_end = new
                .as_mut_ptr()
                .offset(new_gap.end as isize);

            ptr::copy_nonoverlapping(
                self.space(self.gap.end),
                new_gap_end,
                after_gap,
            );
        }

        self.storage = new;
        self.gap = new_gap;
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.gap.start {
                ptr::drop_in_place(self.space_mut(i));
            }
            for i in self.gap.end..self.capacity() {
                ptr::drop_in_place(self.space_mut(i));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::GapBuffer;

    #[test]
    fn basic() {
        let mut buf = GapBuffer::<char>::new();
        buf.insert_iter("boo ".chars());
        buf.insert_iter("bar".chars());

        assert_eq!(buf.get(0), Some(&'b'));
        assert_eq!(buf.get(4), Some(&'b'));
        assert_eq!(buf.get(5), Some(&'a'));
        assert_eq!(buf.len(), 7);
        buf.insert('!');
        assert_eq!(buf.len(), 8);

        assert_eq!(unsafe { *buf.space(0) }, 'b');
        assert_eq!(unsafe { *buf.space(4) }, 'b');
        assert_eq!(unsafe { *buf.space(5) }, 'a');
        assert_eq!(unsafe { *buf.space(7) }, '!');

        assert_eq!(unsafe { *buf.space_mut(0) }, 'b');
        assert_eq!(unsafe { *buf.space_mut(4) }, 'b');
        assert_eq!(unsafe { *buf.space_mut(5) }, 'a');
        assert_eq!(unsafe { *buf.space_mut(7) }, '!');

        assert_eq!(buf.gap.start, 8);
        buf.set_potision(4);

        assert_eq!(buf.gap.start, 4);
        buf.remove();
        assert_eq!(buf.get(0), Some(&'b'));
        assert_eq!(buf.get(4), Some(&'a'));
        assert_eq!(buf.len(), 7);
    }
}
