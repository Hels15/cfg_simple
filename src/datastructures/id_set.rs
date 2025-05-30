// bitset

use crate::datastructures::id::Id;

use std::marker::PhantomData;

pub struct IdSet<E> {
    words: Vec<usize>,
    phantom_data: PhantomData<E>,
}

impl<E: Id> IdSet<E> {
    pub fn zeros(capacity: usize) -> Self {
        Self {
            words: vec![
                0;
                if capacity ==0 {
                    0
                } else {
                    capacity / usize::BITS as usize + 1
                }
            ],
            phantom_data: Default::default(),
        }
    }

    fn ensure_size(&mut self, index: usize) {
        let size = index + 1;
        if size > self.words.len() {
            self.words.resize(size, 0);
        }
    }

    fn index(element: &E) -> (usize, u32) {
        let i = element.index();
        let word = i / usize::BITS as usize;
        let bit = (i % usize::BITS as usize) as u32;
        (word, bit)
    }

    fn add(&mut self, element: E) {
        let (word, bit) = IdSet::index(&element);
        self.ensure_size(word);
        self.words[word] |= 1 << bit;
        debug_assert!(self.get(element));
    }

    pub fn remove(&mut self, element: E) {
        let (word, bit) = IdSet::index(&element);
        if let Some(word) = self.words.get_mut(word) {
            *word &= !(1 << bit);
        }
        debug_assert!(!self.get(element));
    }

    pub fn get(&self, element: E) -> bool {
        let (word, bit) = IdSet::index(&element);
        self.words.get(word).is_some_and(|w| (w & (1 << bit)) != 0)
    }

    pub fn clear(&mut self) {
        self.words.iter_mut().for_each(|w| *w = 0);
    }

    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|w| *w == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::IdSet;

    #[test]
    fn basic() {
        let mut set = IdSet::zeros(0);
        set.add(42);
        assert!(set.get(42));
        assert!(!set.is_empty());
        set.add(43);
        set.remove(42);
        assert!(!set.get(42));
        assert!(set.get(43));
        set.remove(43);

        assert_eq!(set.words.iter().sum::<usize>(), 0);
        for i in 0..100 {
            assert!(!set.get(i));
        }

        assert!(set.is_empty());
    }
}