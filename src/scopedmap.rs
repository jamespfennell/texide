use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;

/// A scoped map is a regular hash map with an additional concept of scope.
///
/// # Examples
///
/// The map methods are the same as the standard hash map (although only a few methods are
/// implemented).
/// ```
/// # use texide::scopedmap::ScopedMap;
/// let mut cat_colors = ScopedMap::new();
/// cat_colors.insert("mint", "ginger");
/// assert_eq!(cat_colors.get(&"mint"), Some(&"ginger"));
/// ```
/// The scoped map additionally has `begin_scope` and `end_scope` methods along with the following
/// behavior: when a scope is ended, all mutations to the map since the beginning of the scope are
/// rolled back.
/// ```
/// # use texide::scopedmap::ScopedMap;
/// let mut cat_colors = ScopedMap::new();
///
/// // Insert a new value, update the value in a new scope, and then end the scope to roll back
/// // the update.
/// cat_colors.insert("paganini", "black");
/// cat_colors.begin_scope();
/// cat_colors.insert("paganini", "gray");
/// assert_eq!(cat_colors.get(&"paganini"), Some(&"gray"));
/// assert_eq!(cat_colors.end_scope(), true);
/// assert_eq!(cat_colors.get(&"paganini"), Some(&"black"));
///
/// // Begin a new scope, insert a value, and then end the scope to roll back the insert.
/// cat_colors.begin_scope();
/// cat_colors.insert("mint", "ginger");
/// assert_eq!(cat_colors.get(&"mint"), Some(&"ginger"));
/// assert_eq!(cat_colors.end_scope(), true);
/// assert_eq!(cat_colors.get(&"mint"), None);
/// ```
/// The `end_scope` method returns a boolean which is false if there is no scope to end, and true
/// otherwise. It is generally an error to end a scope that hasn't been started, so the method is
/// annoted with `#[must_use]`.
/// ```
/// # use texide::scopedmap::ScopedMap;
/// let mut cat_colors = ScopedMap::<String, String>::new();
/// assert_eq!(cat_colors.end_scope(), false);
/// ```
/// There is also a "global" variant of the `insert` method. It inserts the value at the global
/// scope, and erases all other values.
/// ```
/// # use texide::scopedmap::ScopedMap;
/// let mut cat_colors = ScopedMap::new();
/// cat_colors.insert("paganini", "black");
/// cat_colors.begin_scope();
/// cat_colors.insert_global("paganini", "gray");
/// assert_eq!(cat_colors.end_scope(), true);
/// assert_eq!(cat_colors.get(&"paganini"), Some(&"gray"));
/// ```
pub struct ScopedMap<K: Eq + Hash, V> {
    // The implementation is based on two internal data structures. The first is a map that for each
    // key contains the stack of values that have been inserted for that key. Each element of the
    // stack corresponds to a distinct scope: if a value in a scope is updated, we update the value
    // at the top of the stack, rather than pushing a new element on the stack. Not every scope
    // appears in this stack: if no mutation to a key occurs in a scope then the stack remains
    // the same.
    //
    // The second data structure is a stack that keeps track of which keys have been changed in
    // which scopes. This stack is used to rollback changes: at the end of each scope, we use it
    // to identify which keys need to have a value removed from their corresponding stack in the
    // map data structure. The stack has one element for each scope, with the exception of the
    // global scope (we never need to rollback changes to the global scope).
    //
    // The following invariants hold in the implementation:
    // (1) each value stack has at least 1 element. If all of a stack's values are popped off, the
    //     stack is removed from the map.
    // (2) The size of each value stack is equal to the number of times the corresponding key
    //     appears in the changed_keys_stack, plus 1 if the key has been set in the global scope.
    //
    // In the current implementation, all map operations are amortized O(1) with the exception
    // of insert_global which is O(number of current scopes). If we paid extra memory and in the
    // value stack also kept track of the scope index the value corresponds to, we could
    // make insert_global O(1) as well. The memory usage of the implementation is asymptotically
    // optimal: we store O(N) elements of data where N is the number of values of the map that can
    // be observed using get and end_scope.
    key_to_value_stack: HashMap<Rc<K>, Vec<V>>,
    changed_keys_stack: Vec<HashSet<Rc<K>>>,
}

impl<K: Eq + Hash, V> ScopedMap<K, V> {
    /// Inserts the key, value pair.
    pub fn insert(&mut self, key: K, val: V) {
        // If the key is already in the map we retrieve the reference counting smart pointer that
        // has already been created for that key. This ensures that each key is stored in memory
        // at most once.
        let key = match self.key_to_value_stack.get_key_value(&key) {
            None => Rc::new(key),
            Some(key_value) => key_value.0.clone(),
        };
        let new_in_scope = match self.changed_keys_stack.last_mut() {
            // If there are no elements of the changed_keys_stack it means we are in the global
            // scope. In this case, the key is new if it has no corresponding value stack.
            None => !self.key_to_value_stack.contains_key(&key),
            // In all other scopes, we inspect the changed_keys set.
            Some(changed_keys) => changed_keys.insert(key.clone()),
        };
        match new_in_scope {
            true => {
                self.key_to_value_stack
                    .entry(key)
                    .or_insert(Vec::new())
                    .push(val);
            }
            false => {
                // This key has already been set in this scope, which means the value_stack exists
                // and has length at least 1. This makes the two unwrap calls safe.
                *self
                    .key_to_value_stack
                    .get_mut(&key)
                    .unwrap()
                    .last_mut()
                    .unwrap() = val;
            }
        }
    }

    /// Inserts the key, value pair in the global scope.
    pub fn insert_global(&mut self, key: K, val: V) {
        for changed_keys in &mut self.changed_keys_stack {
            changed_keys.remove(&key);
        }
        let mut new_stack = Vec::new();
        new_stack.push(val);
        self.key_to_value_stack.insert(Rc::new(key), new_stack);
    }

    /// Retrieves the value at the provided key.
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.key_to_value_stack.get(key) {
            None => None,
            Some(value_stack) => value_stack.last(),
        }
    }

    /// Begins a new scope.
    pub fn begin_scope(&mut self) {
        // Note that `HashSet::new()` is basically a free operation: no allocations will occur
        // until elements are inserted into it. So even if no mutations are made in this scope, we
        // don't pay much for adding the set eagerly.
        self.changed_keys_stack.push(HashSet::new());
    }

    #[must_use]
    /// Attempts to end the current scope and returns true if there is a scope to end, and false
    /// otherwise.
    pub fn end_scope(&mut self) -> bool {
        match self.changed_keys_stack.pop() {
            None => false,
            Some(changed_keys) => {
                // Note that for the running time analysis we account each iteration of this loop
                // to the insert method that put the key in the changed_keys set. Put another way,
                // this can be considered a defer or cleanup step for all of the insert calls
                // in the scope that is being ended.
                for k in changed_keys {
                    match self.key_to_value_stack.get(&k).unwrap().len() <= 1 {
                        true => {
                            self.key_to_value_stack.remove(&k);
                        }
                        false => {
                            self.key_to_value_stack.get_mut(&k).unwrap().pop();
                        }
                    };
                }
                true
            }
        }
    }

    /// Returns a new empty `ScopedMap`.
    pub fn new() -> ScopedMap<K, V> {
        return ScopedMap {
            key_to_value_stack: HashMap::new(),
            changed_keys_stack: Vec::<HashSet<Rc<K>>>::new(),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::scopedmap::ScopedMap;

    #[test]
    fn insert_after_nested_insert() {
        let mut map = ScopedMap::new();
        map.begin_scope();
        map.insert(3, 5);
        assert_eq!(map.end_scope(), true);
        assert_eq!(map.get(&3), None);
        map.insert(3, 4);
        assert_eq!(map.get(&3), Some(&4));
    }

    #[test]
    fn insert_global_after_no_insert() {
        let mut map = ScopedMap::new();
        map.begin_scope();
        map.insert_global(3, 5);
        assert_eq!(map.end_scope(), true);
        assert_eq!(map.get(&3), Some(&5));
    }
}
