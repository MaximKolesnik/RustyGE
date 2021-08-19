use crate::standard;

const VERSION_MASK: u64 = 0xFFFFF_FFFFF;
const INDEX_MASK: u64 = 0xFFFF_FFFF_FFFF_FFFF & (!VERSION_MASK);

const VERSION_BITS: u32 = VERSION_MASK.count_ones();
#[allow(dead_code)]
const INDEX_BITS: u32 = INDEX_MASK.count_ones();

pub const NUM_SLOTS_PER_PAGE: usize = 64;

const_assert!((VERSION_MASK | INDEX_MASK) == u64::MAX);
const_assert!(!INDEX_MASK == VERSION_MASK);
const_assert!(VERSION_MASK.count_ones() == 40);
const_assert!(INDEX_MASK.count_ones() == 24);
const_assert!(INDEX_MASK.trailing_zeros() == 40);
const_assert!(VERSION_BITS + INDEX_BITS == 64);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Slot {
    data: u64
}

pub struct Generator {
    head: usize,
    slots: standard::Vec<Slot>
}

impl Slot {
    #[inline]
    pub const fn get_index(&self) -> u64 {
        return (self.data & INDEX_MASK) >> VERSION_BITS;
    }

    #[inline]
    pub const fn get_version(&self) -> u64 {
        return self.data & VERSION_MASK;
    }

    fn from_index_version(index: u64, version: u64) -> Slot {
        Slot {
            data: (index << VERSION_BITS) | (version & VERSION_MASK)
        }
    }

    #[inline]
    pub(crate) fn set_index(&mut self, index: u64) {
        self.data = index << VERSION_BITS | (self.data & VERSION_MASK);
    }

    #[inline]
    pub(crate) fn increment_version(&mut self) {
        self.data = (self.data & INDEX_MASK) | ((self.get_version() + 1) & VERSION_MASK);
    }

    #[inline]
    pub(crate) fn set_version(&mut self, version: u64) {
        self.data = (self.data & INDEX_MASK) | (version & VERSION_MASK);
    }

    pub fn new() -> Self {
        Self {
            data: u64::MAX,
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.data & INDEX_MASK != INDEX_MASK
    }
}

impl Generator {
    pub fn acquire(&mut self) -> Slot {
        if self.head == self.slots.len() {
            self.slots.push(Slot::from_index_version(self.slots.len() as u64 + 1, 0));
        }

        let assigned_at = self.head;
        let slot = &mut self.slots[self.head];
        self.head = slot.get_index() as usize;
        
        slot.set_index(assigned_at as u64);

        *slot
    }

    pub fn release(&mut self, slot: &Slot) {
        if !self.is_valid(slot) {
            return;
        }

        let slot = &mut self.slots[slot.get_index() as usize];
        slot.increment_version();
        let new_head = slot.get_index();
        slot.set_index(self.head as u64);
        self.head = new_head as usize;
    }

    #[inline]
    pub fn is_valid(&self, slot: &Slot) -> bool {
        return slot.get_index() < (self.slots.len()) as u64
            && self.get_version_at(&slot) == slot.get_version()
    }

    pub fn new() -> Self {
        Generator {
            head: 0,
            slots: Default::default()
        }
    }

    #[inline]
    fn get_version_at(&self, slot: &Slot) -> u64 {
        return self.slots[slot.get_index() as usize].get_version();
    }
}

type RedirectMemPage = standard::Vec<Slot>;
type DirectStorage<T> = standard::Vec<T>;

pub struct SparseStorage<T> {
    redirect_slots: RedirectMemPage,
    direct_storage: DirectStorage<T>
}

impl<T> SparseStorage<T> {
    pub fn new() -> Self {
        Self {
            redirect_slots: RedirectMemPage::new(),
            direct_storage: DirectStorage::<T>::new(),
        }
    }

    pub fn create(&mut self, slot: &Slot, value: T) {
        self.prepare_redirection_memory(&slot);

        let internal_slot = unsafe {
            self.redirect_slots.get_unchecked_mut(slot.get_index() as usize)
        };
        internal_slot.set_index(self.direct_storage.len() as u64);
        internal_slot.set_version(slot.get_version());

        self.direct_storage.push(value);
    }

    #[inline]
    pub fn has(&self, slot: &Slot) -> bool {
        slot.get_index() < self.redirect_slots.len() as u64
            && self.redirect_slots[slot.get_index() as usize].is_valid()
            && self.redirect_slots[slot.get_index() as usize].get_version() == slot.get_version()
    }

    pub fn erase(&mut self, slot: &Slot) {
        if slot.get_index() < self.redirect_slots.len() as u64 {
            self.erase_unchecked(slot);
        }
    }

    pub fn erase_unchecked(&mut self, slot: &Slot) {
        let internal_slot = unsafe { 
            self.redirect_slots.get_unchecked_mut(slot.get_index() as usize)
        };

        let location = internal_slot.get_index() as usize;
        internal_slot.increment_version();
        internal_slot.set_index(INDEX_MASK);
        self.direct_storage.swap_remove(location);
    }

    #[inline]
    pub fn get_size(&self) -> usize {
        self.direct_storage.len()
    }

    pub fn get(&self, slot: &Slot) -> Option<&T> {
        if !self.has(slot) {
            return None;
        }

        Some(self.get_unchecked(slot))
    }

    pub fn get_mut(&mut self, slot: &Slot) -> Option<&mut T> {
        if !self.has(slot) {
            return None;
        }

        Some(self.get_unchecked_mut(slot))
    }

    pub fn get_unchecked(&self, slot: &Slot) -> &T {
        unsafe {
            self.direct_storage.get_unchecked(self.redirect_slots
                .get_unchecked(slot.get_index() as usize).get_index() as usize)
        }
    }

    pub fn get_unchecked_mut(&mut self, slot: &Slot) -> &mut T {
        unsafe {
            self.direct_storage.get_unchecked_mut(self.redirect_slots
                .get_unchecked_mut(slot.get_index() as usize).get_index() as usize)
        }
    }

    #[inline]
    fn prepare_redirection_memory(&mut self, slot: &Slot) {
        if slot.get_index() as usize >= self.redirect_slots.len() {
            self.redirect_slots.resize(slot.get_index() as usize + 1, Slot::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use slots::*;

    #[test]
    fn work () {
        let mut generator = Generator::new();
        {
            let new_slot = generator.acquire();
            assert_eq!(new_slot.get_index(), 0,
                "Expected index 0, Result {}", new_slot.get_index());
            assert_eq!(new_slot.get_version(), 0,
                "Expected version 0, Result {}", new_slot.get_version());
            assert!(generator.is_valid(&new_slot),
                "Slot is expected to be valid");
        }
        {
            let new_slot = generator.acquire();
            assert_eq!(new_slot.get_index(), 1,
                "Expected index 1, Result {}", new_slot.get_index());
            assert_eq!(new_slot.get_version(), 0,
                "Expected version 0, Result {}", new_slot.get_version());
            assert!(generator.is_valid(&new_slot),
                "Slot is expected to be valid");
            generator.release(&new_slot);
            assert!(!generator.is_valid(&new_slot),
                "Slot is expected to be invalid");

            let new_slot_v2 = generator.acquire();
            assert_eq!(new_slot_v2.get_index(), 1,
                "Expected index 1, Result {}", new_slot_v2.get_index());
            assert_eq!(new_slot_v2.get_version(), 1,
                "Expected version 1, Result {}", new_slot_v2.get_version());
            assert!(generator.is_valid(&new_slot_v2),
                "Slot is expected to be valid");
        }
    }

    use crate::slots::Slot;

    struct Data(u64);

    #[test]
    fn test0() {
        let mut storage = SparseStorage::<Data>::new();
        let mut generator = Generator::new();
        let d = Data(1);
        let s = generator.acquire();

        storage.create(&s, d);

        assert!(storage.has(&s));

        {
            let res = storage.get(&s);

            if let Err(_) = res {
                assert!(false, "This should not be an error")
            }

            let res = res.unwrap();

            assert_eq!(1, res.0,
                "Expected result is {}, result {} ", 1, res.0);
        }

        {
            let res = storage.get_mut(&s);

            if let Err(_) = res {
                assert!(false, "This should not be an error")
            }

            let res = res.unwrap();

            assert_eq!(1, res.0,
                "Expected result is {}, result {} ", 1, res.0);
        }

        {
            let res = storage.get_unchecked_mut(&s);
            assert_eq!(1, res.0,
                "Expected result is {}, result {} ", 1, res.0);
        }

        {
            let res = storage.get_unchecked(&s);
            assert_eq!(1, res.0,
                "Expected result is {}, result {} ", 1, res.0);
        }
    }

    #[test]
    fn test1() {
        let mut storage = SparseStorage::<Data>::new();
        let mut generator = Generator::new();
        let mut slots = crate::standard::Vec::<Slot>::new();
        let mut mapping = crate::standard::HashMap::<Slot, u64>::default();
        for n in 0..9 {
            let slot = generator.acquire();
            storage.create(&slot, Data(n));
            slots.push(slot);
            mapping.insert(slot, n);
        }

        assert_eq!(9, storage.get_size(),
                "Expected size is {}, result {} ", 9, storage.get_size());

        for item in slots.iter() {
            assert!(storage.has(&item));
        }

        for n in 0..9 as u64 {
            let val = storage.get_unchecked(slots.get(n as usize).unwrap());
            assert_eq!(n, val.0,
                "Expected result is {}, result {} ", n, val.0);
        }

        storage.erase(slots.get(8).unwrap());
        assert_eq!(8, storage.get_size(),
                "Expected size is {}, result {} ", 8, storage.get_size());

        assert!(!storage.has(slots.get(8).unwrap()));
        slots.swap_remove(8);

        for item in slots.iter() {
            let val = storage.get_unchecked(item);
            let mapped = mapping.get(item).unwrap();
            assert_eq!(*mapped, val.0,
                "Expected result is {}, result {} ", *mapped, val.0);
        }

        while !slots.is_empty() {
            let val = storage.get_unchecked(&slots[0]);
            let mapped = mapping.get(&slots[0]).unwrap();
            assert_eq!(*mapped, val.0,
                "Expected result is {}, result {} ", *mapped, val.0);
            slots.swap_remove(0);
        }
    }
}
