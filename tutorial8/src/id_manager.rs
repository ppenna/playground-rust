/*
    Struct which assigns IDs to objects and remembers them!
*/

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct ID(usize);
impl ID {
    // for convenience, function to step to the next ID:
    pub fn step(&mut self) {
        self.0 += 1;
    }
}

/*
    Desired Functionality:
    1. Get the ID for an item &T.
    2. Get the item &T from an ID.
    3. Insert an item of type T, assigning it a new ID.
    4. Delete an item of type T.
*/

/*
    ========== ATTEMPT #1 ==========
*/

pub struct IDManager1<T>
where
    T: Eq + Hash,
{
    next_id: ID,
    id_to_item: HashMap<ID, T>,
    item_to_id: HashMap<T, ID>,
}

impl<T> Default for IDManager1<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        // Empty maps
        Self {
            next_id: Default::default(),
            id_to_item: Default::default(),
            item_to_id: Default::default(),
        }
    }
}

impl<T> IDManager1<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        // Doesn't store any IDs
        Default::default()
    }

    // The bidirectional map
    pub fn get_id(&self, item: &T) -> Option<ID> {
        self.item_to_id.get(item).copied()
    }
    pub fn get_item(&self, id: ID) -> Option<&T> {
        self.id_to_item.get(&id)
    }

    // Insertion and deletion
    pub fn insert(&mut self, item: T) -> ID {
        let id = self.next_id;
        self.id_to_item.insert(id, item.clone());
        self.item_to_id.insert(item, id);
        self.next_id.step();
        id
    }
    pub fn delete(&mut self, item: &T) -> bool {
        // true if item existed, false if not
        if let Some(id) = self.get_id(item) {
            self.id_to_item.remove(&id);
            self.item_to_id.remove(item);
            true
        } else {
            eprintln!("Warning: tried to delete nonexistent item");
            false
        }
    }
}

/*
    Sadly this would not be very appreciated by all users,
    and this code is not really stdlib-quality

    Reason: we required Clone on T, which is both inefficient and
    less general.
        Reason 1: Sometimes cloning a data type is very inefficient;
        Reason 2: Not all data types can be cloned at all.

    The above design:
        GOOD for your own use case, where T: Clone is perfectly easy to satisfy
        BAD for a general data structure library that you want others to use
        and that you want to be useful in general code.

    Rule of thumb: avoid unnecessary Trait bounds!
*/

/*
    ========== ATTEMPT #2 ==========

    Try to remove the Clone bound
*/

pub struct IDManager2<'a, T>
where
    T: Eq + Hash,
{
    next_id: ID,
    id_to_item: HashMap<ID, T>,
    item_to_id: HashMap<&'a T, ID>,
}

impl<'a, T> Default for IDManager2<'a, T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        // Empty maps
        Self {
            next_id: Default::default(),
            id_to_item: Default::default(),
            item_to_id: Default::default(),
        }
    }
}

impl<'a, T> IDManager2<'a, T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        // Doesn't store any IDs
        Default::default()
    }

    // The bidirectional map
    pub fn get_id(&self, item: &T) -> Option<ID> {
        self.item_to_id.get(item).copied()
    }
    pub fn get_item(&self, id: ID) -> Option<&T> {
        self.id_to_item.get(&id)
    }

    // Insertion and deletion
    pub fn insert(&mut self, item: T) -> ID {
        // **Hard Part!**
        let id = self.next_id;

        self.id_to_item.insert(id, item);

        // This line in particular is just unsafe
        // Use case for unsafe code:
        // - I know something the compiler doesn't
        // - Localized: unsafe reference is only in this block
        // How to do this with unsafe code?

        // First make a raw pointer
        // (unwrap OK -- just inserted, so I know it exists)
        let item_ref_raw: *const T = self.id_to_item.get(&id).unwrap();

        // Convert the pointer to a reference and put it in
        // the other HashMap
        unsafe {
            // as long as we're already unsafe, unwrap is fine here
            // (I know this isn't a null pointer)
            let item_ref: &T = item_ref_raw.as_ref().unwrap();
            self.item_to_id.insert(item_ref, id);
        }

        self.next_id.step();
        id
    }
    pub fn delete(&mut self, item: &T) -> bool {
        // true if item existed, false if not
        if let Some(id) = self.get_id(item) {
            self.id_to_item.remove(&id);
            self.item_to_id.remove(item);
            true
        } else {
            eprintln!("Warning: tried to delete nonexistent item");
            false
        }
    }
}

/*
    This works!

    But it's precarious: I am relying on some assumptions about memory.

    I'm assuming that the memory location of the T in the first hash map
    never changes.

    To really be safe, we have to read the HashMap API and make sure that
    the HashMap isn't moved...
    - maybe we call a method at the beginning when we create the HashMap
      to ensure its size statically...
*/

/*
    Really what we want: a safe solution?

    Smart pointer types!

    - Box<T>, Cell<T>, RefCell<T>, Rc<T>, Arc<T>, ...

      (we've seen Box from Box<dyn Trait>)
      Box: allocated on the heap
      In particular, means that you don't need to know its size at compile time

    - Intuitively: references to a T with additional smart features.

    What will help us here? Reference counting.
    - Allow multiple references to an object, and neither one is the designated owner
    - When all references go out of scope, the object is deallocated.

    Reference counting is done with the smart pointer type
        Rc<T>
*/

/*
    ========== ATTEMPT #3 ==========

    Principled approach: use a smart pointer Rc<T>

    - No Clone bound
    - No unsafe code
    - High-quality stdlib-worthy implementation
*/

use std::ops::Deref;
use std::rc::Rc;

pub struct IDManager3<T>
where
    T: Eq + Hash,
{
    next_id: ID,
    id_to_item: HashMap<ID, Rc<T>>,
    item_to_id: HashMap<Rc<T>, ID>,
}

impl<T> Default for IDManager3<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        // Empty maps
        Self {
            next_id: Default::default(),
            id_to_item: Default::default(),
            item_to_id: Default::default(),
        }
    }
}

impl<T> IDManager3<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        // Doesn't store any IDs
        Default::default()
    }

    // The bidirectional map
    pub fn get_id(&self, item: &T) -> Option<ID> {
        self.item_to_id.get(item).copied()
    }
    pub fn get_item(&self, id: ID) -> Option<&T> {
        // to convert the Rc<T> to &T can use deref
        self.id_to_item.get(&id).map(|x| x.deref())
    }

    // Insertion and deletion
    pub fn insert(&mut self, item: T) -> ID {
        // **Hard Part!**
        let id = self.next_id;

        let item_ref = Rc::new(item);

        // Notice that T doesn't implement clone
        // But Rc<T> does!
        self.id_to_item.insert(id, item_ref.clone());
        self.item_to_id.insert(item_ref, id);

        self.next_id.step();
        id
    }
    pub fn delete(&mut self, item: &T) -> bool {
        // true if item existed, false if not
        if let Some(id) = self.get_id(item) {
            self.id_to_item.remove(&id);
            // more type magic, &T auto converted to Rc<T>
            self.item_to_id.remove(item);
            true
        } else {
            eprintln!("Warning: tried to delete nonexistent item");
            false
        }
    }
}

/*
    SUMMARY

    This was an introduction to smart pointers and how they can be used
    to implement data structures efficiently and safely.

    Rc<T> behaves like &T, reference to T, but it keeps a reference count
    and allows passing around multiple (immutable) references to the T.

    Avoids borrowing/lifetime issues:
    - Notice how we don't have 'a in our struct anymore
    - We were able to pass in a reference to T without transferring ownership
      explicitly (bypass ownership rules, in a safe way)

    (If we need mutability, we will need other types, we will see these
    in the near future.)
*/
