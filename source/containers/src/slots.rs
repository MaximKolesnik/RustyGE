const VERSION_MASK: u64 = 0xFFFFF_FFFFF;
const INDEX_MASK: u64 = 0xFFFF_FFFF_FFFF_FFFF & (!VERSION_MASK);

const VERSION_BITS: u32 = VERSION_MASK.count_ones();
#[allow(dead_code)]
const INDEX_BITS: u32 = INDEX_MASK.count_ones();

const SLOTS_PER_PAGE: u64 = 64;
const PAGE_SIZE: usize = std::mem::size_of::<Slot>() * SLOTS_PER_PAGE as usize;

const_assert!((VERSION_MASK | INDEX_MASK) == u64::MAX);
const_assert!(!INDEX_MASK == VERSION_MASK);
const_assert!(VERSION_MASK.count_ones() == 40);
const_assert!(INDEX_MASK.count_ones() == 24);
const_assert!(INDEX_MASK.trailing_zeros() == 40);
const_assert!(VERSION_BITS + INDEX_BITS == 64);

#[derive(Copy, Clone)]
pub struct Slot {
    data: u64
}

pub struct Generator {
    head: u64,
    pages: Vec<MemPage>
}

struct MemPage {
    mem: Box<[Slot]>
}

impl Slot {
    pub const fn get_index(&self) -> u64 {
        return (self.data & INDEX_MASK) >> VERSION_BITS;
    }

    pub const fn get_version(&self) -> u64 {
        return self.data & VERSION_MASK;
    }

    fn from_index_version(index: u64, version: u64) -> Slot {
        Slot {
            data: (index << VERSION_BITS) | (version & VERSION_MASK)
        }
    }

    fn set_index(&mut self, index: u64) {
        self.data = index << VERSION_BITS | (self.data & VERSION_MASK);
    }

    fn increment_version(&mut self) {
        self.data = (self.data & INDEX_MASK) | ((self.get_version() + 1) & VERSION_MASK);
    }
}

impl Generator {
    pub fn acquire(&mut self) -> Result<Slot, AllocError> {
        if self.head == self.pages.len() as u64 * SLOTS_PER_PAGE {
            match self.allocate_new_page() {
                None => (),
                Some(err) => return Err(err)
            }
        }

        let page_num = (self.head / SLOTS_PER_PAGE) as usize;
        let page_offset = (self.head % SLOTS_PER_PAGE) as usize;

        let slot = &mut self.pages[page_num].mem[page_offset];
        self.head = slot.get_index();
        
        slot.set_index((page_num  * SLOTS_PER_PAGE as usize + page_offset) as u64);

        return Ok(*slot);
    }

    pub fn release(&mut self, slot: &Slot) {
        if !self.is_valid(slot) {
            return;
        }

        let page_num = (slot.get_index() / SLOTS_PER_PAGE) as usize;
        let page_offset = (slot.get_index() % SLOTS_PER_PAGE) as usize;

        let slot = &mut self.pages[page_num].mem[page_offset];
        slot.increment_version();
        let new_head = slot.get_index();
        slot.set_index(self.head);
        self.head = new_head;
    }

    pub fn is_valid(&self, slot: &Slot) -> bool {
        return slot.get_index() < (self.pages.len() as u64) * SLOTS_PER_PAGE
            && self.get_version_at(&slot) == slot.get_version()
    }

    pub fn new() -> Self {
        Generator {
            head: 0,
            pages: Default::default()
        }
    }

    fn allocate_new_page(&mut self) -> Option<AllocError> {
        let num_pages: usize = self.pages.len();

        unsafe {
            let new_page = match MemPage::new() {
                Ok(page) => page,
                Err(err) => return Some(err)
            };
            self.pages.push(new_page);
        }

        for n in 0..=SLOTS_PER_PAGE as usize {
            let index = (num_pages * (SLOTS_PER_PAGE as usize) + n + 1) as u64;
            self.pages[num_pages].mem[n] = Slot::from_index_version(index, 0);
        }

        return None;
    }

    #[inline]
    fn get_version_at(&self, slot: &Slot) -> u64 {
        return self.pages[(slot.get_index() / SLOTS_PER_PAGE) as usize]
            .mem[(slot.get_index() % SLOTS_PER_PAGE) as usize].get_version();
    }
}

impl MemPage {
    unsafe fn new() -> Result<MemPage, AllocError> {
        type PageType = [Slot; PAGE_SIZE];

        let layout = std::alloc::Layout::new::<PageType>();
        let mem = std::alloc::alloc(layout) as *mut PageType;
        if mem == std::ptr::null_mut() {
            return Err(AllocError)
        }

        Ok(MemPage {
            mem: Box::from_raw(mem)
        })
    }
}

#[derive(Debug, Clone)]
pub struct AllocError;

impl std::fmt::Display for AllocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "allocation failed")
    }
}

#[cfg(test)]
mod tests {
    use slots::*;

    #[test]
    fn work () {
        let mut generator = Generator::new();
        {
            let new_slot = generator.acquire().unwrap();
            assert_eq!(new_slot.get_index(), 0,
                "Expected index 0, Result {}", new_slot.get_index());
            assert_eq!(new_slot.get_version(), 0,
                "Expected version 0, Result {}", new_slot.get_version());
            assert!(generator.is_valid(&new_slot),
                "Slot is expected to be valid");
        }
        {
            let new_slot = generator.acquire().unwrap();
            assert_eq!(new_slot.get_index(), 1,
                "Expected index 1, Result {}", new_slot.get_index());
            assert_eq!(new_slot.get_version(), 0,
                "Expected version 0, Result {}", new_slot.get_version());
            assert!(generator.is_valid(&new_slot),
                "Slot is expected to be valid");
            generator.release(&new_slot);
            assert!(!generator.is_valid(&new_slot),
                "Slot is expected to be invalid");

            let new_slot_v2 = generator.acquire().unwrap();
            assert_eq!(new_slot_v2.get_index(), 1,
                "Expected index 1, Result {}", new_slot_v2.get_index());
            assert_eq!(new_slot_v2.get_version(), 1,
                "Expected version 1, Result {}", new_slot_v2.get_version());
            assert!(generator.is_valid(&new_slot_v2),
                "Slot is expected to be valid");
        }
    }
}
