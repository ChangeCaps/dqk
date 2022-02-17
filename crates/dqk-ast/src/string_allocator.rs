use std::{collections::HashSet, path::Path};

#[derive(Default)]
pub struct StringAllocator {
    allocations: HashSet<&'static str>,
}

impl StringAllocator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&mut self, string: impl AsRef<str>) -> &'static str {
        let string = string.as_ref();

        if !self.allocations.contains(string) {
            let string = String::from(string);

            let allocation = Box::leak(string.into_boxed_str());

            self.allocations.insert(allocation);
        }

        self.allocations.get(string).unwrap()
    }

    pub fn get_path(&mut self, string: impl AsRef<str>) -> &'static Path {
        Path::new(self.get(string))
    }

    /// Consumes self deallocates all allocated strings.
    ///
    /// # Safety
    /// * no references to strings allocated with self can be alive.
    pub unsafe fn dealloc(self) {
        for allocation in self.allocations {
            unsafe { Box::from_raw(allocation as *const _ as *mut str) };
        }
    }
}

#[cfg(test)]
mod tests {}
