#[cfg(feature = "single-threaded")]
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

/// # Hc<T>
/// A single-threaded custom smart pointer type for managing the lifecycle of consed values.
///
/// ## Type Parameters
/// * `T` - The type of values managed by this smart pointer. Must implement `Hash` and `Eq`.
///
/// ## Fields
/// * `inner`: `Rc<Inner<T>>` - Reference counted pointer to the inner value.
///
/// ## Example
/// ```
/// use hash_cons::single_threaded::Hc;
/// use hash_cons::single_threaded::HCTable;
/// let table = HCTable::new();
/// let hc_pointer = table.hashcons(42);
/// assert_eq!(*hc_pointer.get(), 42);
/// ```
pub struct Hc<T>
where
    T: Hash + Eq,
{
    inner: Rc<Inner<T>>,
}

impl<T> Hc<T>
where
    T: Hash + Eq,
{
    /// Retrieves a reference to the value stored in this `Hc<T>`.
    ///
    /// ## Returns
    /// A reference to the stored value.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let my_value = table.hashcons(10);
    /// assert_eq!(*my_value.get(), 10);
    /// ```
    pub fn get(&self) -> &T {
        &self.inner.elem
    }
}

impl<T: PartialEq> PartialEq for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to compare two `Hc<T>` instances for equality.
    ///
    /// ## Parameters
    /// * `other`: Another `Hc<T>` instance to compare with.
    ///
    /// ## Returns
    /// `true` if the two instances are equal, `false` otherwise.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value1 = table.hashcons(5);
    /// let value2 = table.hashcons(5);
    /// let value3 = table.hashcons(10);
    /// assert_eq!(value1, value2);
    /// assert_ne!(value1, value3);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.inner.elem == other.inner.elem
    }
}

impl<T> Eq for Hc<T> where T: Hash + Eq {}

impl<T> Hash for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to hash `Hc<T>` instances.
    /// This method is useful for storing `Hc<T>` instances in a `HashMap`.
    /// It is also used internally by the `HCTable` to manage the storage of
    /// `Hc<T>` instances.
    /// ## Parameters
    /// * `state`: The `Hasher` instance to use for hashing.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    /// let table = HCTable::new();
    /// let value = table.hashcons(5);
    /// let mut hasher = DefaultHasher::new();
    /// value.hash(&mut hasher);
    /// let hash = hasher.finish();
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.elem.hash(state);
    }
}

impl<T> Clone for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to clone `Hc<T>` instances.
    /// ## Returns
    /// A new `Hc<T>` instance with the same value as the original.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value = table.hashcons(5);
    /// let value_clone = value.clone();
    /// assert_eq!(value, value_clone);
    /// ```
    /// ## Note
    /// This method is implemented using `Rc::clone()`.
    /// This method does not actually clone the underlying value. Instead, it
    /// creates a new `Hc<T>` instance that points to the same value.
    /// This is the desired behavior for hash consing.
    /// If you need to clone the underlying value, you can use the `get()` method
    /// to retrieve a reference to the value and clone it.
    ///
    fn clone(&self) -> Self {
        Hc {
            inner: self.inner.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to print `Hc<T>` instances.
    /// This method is useful for debugging.
    /// ## Parameters
    /// * `f`: The `Formatter` instance to use for printing.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value = table.hashcons(5);
    /// println!("{:?}", value);
    /// ```
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.elem.fmt(f)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to print `Hc<T>` instances.
    /// This method is useful for debugging.
    /// ## Parameters
    /// * `f`: The `Formatter` instance to use for printing.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value = table.hashcons(5);
    /// println!("{}", value);
    /// ```
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.elem.fmt(f)
    }
}

impl<T> std::ops::Deref for Hc<T>
where
    T: Hash + Eq,
{
    type Target = T;

    /// Provides the functionality to dereference `Hc<T>` instances.
    /// This method is useful for accessing the underlying value.
    /// ## Returns
    /// A reference to the underlying value.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value = table.hashcons(5);
    /// assert_eq!(*value, 5);
    /// ```
    /// ## Note
    /// This method is implemented using `Rc::deref()`.
    /// This method does not actually dereference the underlying value. Instead, it
    /// returns a reference to the value.
    /// This is the desired behavior for hash consing.
    /// If you need to dereference the underlying value, you can use the `get()` method
    /// to retrieve a reference to the value.
    ///
    fn deref(&self) -> &Self::Target {
        &self.inner.elem
    }
}

impl<T> AsRef<T> for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to convert `Hc<T>` instances to references.
    /// This method is useful for accessing the underlying value.
    /// ## Returns
    /// A reference to the underlying value.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value = table.hashcons(5);
    /// assert_eq!(value.as_ref(), &5);
    /// ```
    /// ## Note
    /// This method is implemented using `Rc::as_ref()`.
    /// This method does not actually convert the underlying value to a reference. Instead, it
    /// returns a reference to the value.
    /// This is the desired behavior for hash consing.
    /// If you need to convert the underlying value to a reference, you can use the `get()` method
    /// to retrieve a reference to the value.
    ///
    fn as_ref(&self) -> &T {
        &self.inner.elem
    }
}

impl<T: PartialOrd> PartialOrd for Hc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to compare two `Hc<T>` instances.
    /// This method is useful for sorting `Hc<T>` instances.
    /// ## Parameters
    /// * `other`: Another `Hc<T>` instance to compare with.
    /// ## Returns
    /// `Some(std::cmp::Ordering)` if the two instances are comparable, `None` otherwise.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value1 = table.hashcons(5);
    /// let value2 = table.hashcons(10);
    /// assert!(value1 < value2);
    /// ```
    /// ## Note
    /// This method is implemented using `Rc::partial_cmp()`.
    /// This method does not actually compare the underlying values. Instead, it
    /// compares the `Hc<T>` instances.
    ///
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.elem.partial_cmp(&other.inner.elem)
    }
}

impl<T> Ord for Hc<T>
where
    T: Ord + Hash + Eq,
{
    /// Provides the functionality to compare two `Hc<T>` instances.
    /// This method is useful for sorting `Hc<T>` instances.
    /// ## Parameters
    /// * `other`: Another `Hc<T>` instance to compare with.
    /// ## Returns
    /// `std::cmp::Ordering` if the two instances are comparable.
    /// ## Example
    /// ```
    /// use hash_cons::single_threaded::HCTable;
    /// let table = HCTable::new();
    /// let value1 = table.hashcons(5);
    /// let value2 = table.hashcons(10);
    /// assert!(value1 < value2);
    /// ```
    /// ## Note
    /// This method is implemented using `Rc::cmp()`.
    /// This method does not actually compare the underlying values. Instead, it
    /// compares the `Hc<T>` instances.
    ///
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.elem.cmp(&other.inner.elem)
    }
}

///  # HCTable<T>
/// A table structure for efficiently managing `Hc<T>` instances.
/// This struct hides the underlying table and its reference count management.
///
/// This structure utilizes a HashMap to store `Hc<T>` instances, offering
/// quick retrieval and management capabilities.
///
/// ## Type Parameters
/// * `T` - The type of values managed by the `Hc<T>` instances within this table.
///
/// ## Fields
/// * `table`: HashMap - The underlying data structure storing `Hc<T>` instances.
///
pub struct HCTable<T>
where
    T: Hash + Eq,
{
    inner: Rc<InnerTable<T>>,
}

impl<T> HCTable<T>
where
    T: Hash + Eq,
{
    /// Creates a new `HCTable`.
    ///
    /// ## Returns
    /// A new instance of `HCTable<T>`.
    pub fn new() -> Self {
        HCTable {
            inner: Rc::new(InnerTable::new()),
        }
    }

    /// Simplifies object retrieval or creation with an intuitive interface.
    ///
    /// ## Parameters
    /// * `value`: The value to be managed.
    ///
    /// ## Returns
    /// A `Hc<T>` instance corresponding to the given value.
    pub fn hashcons(&self, value: T) -> Hc<T> {
        Hc {
            inner: self.intern(value),
        }
    }

    /// Internal method to manage the storage of values in `HCTable`.
    /// It ensures that each value is stored only once, providing a shared
    /// reference to the stored value.
    ///
    /// ## Parameters
    /// * `value`: The value to be stored or retrieved.
    ///
    /// ## Returns
    /// A `Rc<Inner<T>>` pointer to the stored value.
    ///
    fn intern(&self, value: T) -> Rc<Inner<T>> {
        let rc_table = self.inner.clone();
        let rc_table_dup = Rc::clone(&rc_table);

        let mut mut_table = rc_table_dup.table.borrow_mut();

        let rc_value = Rc::new(value);
        let rc_val_dup = rc_value.clone();

        match mut_table.entry(rc_val_dup) {
            Entry::Occupied(mut o) => {
                let weak_hc = o.get();

                if let Some(rc_hc) = weak_hc.upgrade() {
                    return rc_hc;
                }

                let elem = rc_value;
                let _table = Rc::downgrade(&rc_table);
                let new_elem = Rc::new(Inner { elem, _table });
                o.insert(Rc::downgrade(&new_elem));
                new_elem
            }

            Entry::Vacant(v) => {
                let _table = Rc::downgrade(&rc_table);
                let elem = rc_value;
                let new_elem = Rc::new(Inner { elem, _table });
                v.insert(Rc::downgrade(&new_elem));
                new_elem
            }
        }
    }

    /// Cleans up the `HCTable`, removing any values that are no longer in use.
    /// This method is useful for managing memory and ensuring that unused
    /// values are not unnecessarily kept in the table.
    ///
    pub fn cleanup(&self) {
        self.inner.cleanup();
    }

    /// Returns the number of elements currently stored in the `HCTable`.
    ///
    /// ## Returns
    /// The number of elements in the `HCTable`.
    ///
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

/// # Inner<T>
/// A struct to encapsulate the inner workings of `Hc<T>`.
/// It holds the actual value and a weak reference to its containing table.
///
/// ## Type Parameters
/// * `T` - The type of the encapsulated value.
///
/// ## Fields
/// * `elem`: The actual stored value.
/// * `_table`: A weak reference to the `HCTable` that contains this value.
///
struct Inner<T>
where
    T: Hash + Eq,
{
    /// The actual stored value.
    /// This is the value that is returned when the `Hc<T>` is dereference.
    elem: Rc<T>,

    /// A weak reference to the `HCTable` that contains this value.
    /// This is used to remove the value from the table when it is no longer in use.
    _table: Weak<InnerTable<T>>,
}

impl<T> Drop for Inner<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to drop `Inner<T>` instances.
    /// This method is useful for managing the lifecycle of `Hc<T>` instances.
    /// ## Note
    /// This method is implemented using `Weak::upgrade()`.
    fn drop(&mut self) {
        let weak_table = self._table.clone();
        match weak_table.upgrade() {
            Some(rc_table) => {
                let key = self.elem.clone();
                let mut mut_table = rc_table.table.borrow_mut();
                mut_table.remove_entry(&key);
            }
            None => {
                // The table has already been dropped;
                #[cfg(debug_assertions)]
                eprintln!("Warning: InnerTable<T> already dropped when trying to remove Inner<T>.");
            }
        }
    }
}

/// # InnerTable<T>
/// A helper struct to manage the internal storage of `HCTable`.
/// It provides mechanisms to manage and access stored `Hc<T>` instances.
///
/// ## Type Parameters
/// * `T` - The type of values stored in the `HCTable`.
///
/// ## Fields
/// * `table`: The actual HashMap that stores the `Hc<T>` instances.
///
pub struct InnerTable<T>
where
    T: Hash + Eq,
{
    /// The actual HashMap that stores the `Hc<T>` instances.
    /// This is the underlying data structure used by `HCTable`.
    /// It is hidden from the user.
    table: RefCell<HashMap<Rc<T>, Weak<Inner<T>>>>,
}

impl<T> InnerTable<T>
where
    T: Hash + Eq,
{
    /// Creates a new `InnerTable<T>`.
    /// ## Returns
    /// A new instance of `InnerTable<T>`.
    ///
    fn new() -> Self {
        InnerTable {
            table: RefCell::new(HashMap::new()),
        }
    }

    /// Returns the number of elements currently stored in the `InnerTable`.
    /// ## Returns
    /// The number of elements in the `InnerTable`.
    ///
    fn len(&self) -> usize {
        self.table.borrow().len()
    }

    /// Cleans up the `InnerTable`, removing any values that are no longer in use.
    /// This method is useful for managing memory and ensuring that unused
    /// values are not unnecessarily kept in the table.
    /// ## Note
    /// This method is implemented using `Weak::strong_count()`.
    /// It removes any values that have a `strong_count()` of 0.
    /// This is the desired behavior for hash consing.
    ///
    fn cleanup(&self) {
        let mut mut_table = self.table.borrow_mut();
        mut_table.retain(|_, weak_hc: &mut Weak<Inner<T>>| weak_hc.strong_count() > 0);
    }
}
