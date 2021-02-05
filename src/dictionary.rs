#[derive(Debug, Clone, PartialEq)]
pub enum DictionaryErr {
    Overflow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary<Key, Value>
where
    Key: PartialEq,
{
    capacity: usize,
    data: Vec<(Key, Value)>,
}
impl<Key, Value> Dictionary<Key, Value>
where
    Key: PartialEq,
{
    /// Creates a new dictionary with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Vec::with_capacity(capacity),
        }
    }

    /// Returns a slice of the values and keys
    pub fn dictionary(&self) -> &[(Key, Value)] {
        &self.data
    }

    /// Inserts a new value at the given key, overwriting any previous keys.
    pub fn insert(&mut self, key: Key, value: Value) -> Result<(), DictionaryErr> {
        // Remove first item with the same key
        let mut i = 0;
        while i < self.data.len() {
            if self.data[i].0 == key {
                self.data.remove(i);
                break;
            }

            i += 1;
        }

        if self.data.len() == self.capacity {
            return Err(DictionaryErr::Overflow);
        }

        self.data.push((key, value));

        Ok(())
    }

    pub fn get_addr(&self, key: Key) -> Option<usize> {
        // TODO: test
        for (i, (stored_key, _)) in self.data.iter().enumerate() {
            if key == *stored_key {
                return Some(i);
            }
        }

        None
    }

    pub fn get_from_addr(&self, addr: usize) -> Option<&Value> {
        // TODO: test
        if addr < self.data.len() {
            return Some(&self.data[addr].1);
        }

        None
    }

    /// Attempts to return a value at the given key.
    pub fn get(&self, key: Key) -> Option<&Value> {
        for (stored_key, value) in self.data.iter() {
            if key == *stored_key {
                return Some(value);
            }
        }

        None
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_no_match_returns_none() {
        let cap = 11;
        let mut d = Dictionary::<i32, i32>::new(cap);
        d.insert(4, 3).unwrap();
        assert_eq!(None, d.get(339));
    }

    #[test]
    fn insert_would_overflow_returns_err() {
        let cap = 1;
        let mut d = Dictionary::<i32, i32>::new(cap);
        d.insert(4, 3).unwrap();
        assert_eq!(DictionaryErr::Overflow, d.insert(34, 5).unwrap_err());
    }

    #[test]
    fn insert_removes_old_value() {
        let cap = 30201;
        let mut d = Dictionary::<i32, i32>::new(cap);
        d.insert(4, 3).unwrap();
        d.insert(4, 5).unwrap();

        assert_eq!((4, 5), d.dictionary()[0]);
    }

    #[test]
    fn insert_sets() {
        let cap = 30201;
        let mut d = Dictionary::<i32, i32>::new(cap);
        d.insert(2, 3).unwrap();
        d.insert(4, 5).unwrap();

        assert_eq!((2, 3), d.dictionary()[0]);
        assert_eq!((4, 5), d.dictionary()[1]);
    }

    #[test]
    fn new_sets_capacity() {
        let cap = 30201;
        let d = Dictionary::<i32, i32>::new(cap);
        assert_eq!(cap, d.capacity);
        assert_eq!(true, d.data.is_empty());
    }

    #[test]
    fn clear_wipes_data() {
        let cap = 30201;
        let mut d = Dictionary::<i32, i32>::new(cap);
        d.insert(2, 3);
        d.insert(4, 5);
        d.clear();
        assert_eq!(true, d.data.is_empty());
    }
}
