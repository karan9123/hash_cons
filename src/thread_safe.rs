#[cfg(feature = "thread-safe")]
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, Weak};

/// # Ahc<T>
/// A thread-safe custom smart pointer type for managing the lifecycle of consed values.
///
/// ## Type Parameters
/// * `T` - The type of values managed by this smart pointer. Must implement `Hash` and `Eq`.
///
/// ## Fields
/// * `inner`: `Arc<Inner<T>>` - Atomically reference counted pointer to the inner value.
///
/// ## Example
/// ```
/// use hash_cons::Ahc;
/// use hash_cons::AhcTable;
/// let table = AhcTable::new();
/// let ahc_pointer = table.hashcons(42);
/// assert_eq!(*ahc_pointer.get(), 42);
/// ```
pub struct Ahc<T>
where
    T: Hash + Eq,
{
    // This is the reference to the underlying value.
    inner: Arc<Inner<T>>,
}

// Implementing the traits for the custom smart pointer type.
impl<T> Ahc<T>
where
    T: Hash + Eq,
{
    /// Retrieves a reference to the value stored in this `Ahc<T>`.
    ///
    /// ## Returns
    /// A reference to the stored value.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let my_value = table.hashcons(10);
    /// assert_eq!(*my_value.get(), 10);
    /// ```
    // Retrieve the inner value
    pub fn get(&self) -> &T {
        &self.inner.elem
    }
}

impl<T: PartialEq> PartialEq for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to compare two `Ahc<T>` instances for equality.
    ///
    /// ## Parameters
    /// * `other`: Another `Ahc<T>` instance to compare with.
    ///
    /// ## Returns
    /// `true` if the two instances are equal, `false` otherwise.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
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

impl<T> Eq for Ahc<T> where T: Hash + Eq {}

impl<T> Hash for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to hash `Ahc<T>` instances.
    /// This method is useful for storing `Ahc<T>` instances in a `HashMap`.
    /// It is also used internally by the `AhcTable` to manage the storage of
    /// `Ahc<T>` instances.
    /// ## Parameters
    /// * `state`: The `Hasher` instance to use for hashing.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    ///
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// let mut hasher = DefaultHasher::new();
    /// value.hash(&mut hasher);
    /// let hash = hasher.finish();
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.elem.hash(state);
    }
}

impl<T> Clone for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to clone `Ahc<T>` instances.
    /// ## Returns
    /// A new `Ahc<T>` instance with the same value as the original.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// let value_clone = value.clone();
    /// assert_eq!(value, value_clone);
    /// ```
    ///
    /// ## Note
    /// This method is implemented using `Arc::clone()`.
    /// This method does not actually clone the underlying value. Instead, it
    /// creates a new `Ahc<T>` instance that points to the same value.
    /// This is the desired behavior for hash consing.
    /// If you need to clone the underlying value, you can use the `get()` method
    /// to retrieve a reference to the value and clone it.
    ///
    fn clone(&self) -> Self {
        Ahc {
            inner: self.inner.clone(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to print `Ahc<T>` instances.
    /// This method is useful for debugging.
    /// ## Parameters
    /// * `f`: The `Formatter` instance to use for printing.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// println!("{:?}", value);
    /// ```
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.elem.fmt(f)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to print `Ahc<T>` instances.
    /// This method is useful for debugging.
    /// ## Parameters
    /// * `f`: The `Formatter` instance to use for printing.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// println!("{}", value);
    /// ```
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.elem.fmt(f)
    }
}

impl<T> std::ops::Deref for Ahc<T>
where
    T: Hash + Eq,
{
    type Target = T;

    /// Provides the functionality to dereference `Ahc<T>` instances.
    /// This method is useful for accessing the underlying value.
    /// ## Returns
    /// A reference to the underlying value.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// assert_eq!(*value, 5);
    /// ```
    ///
    /// ## Note
    /// This method is implemented using `Arc::deref()`.
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

impl<T> AsRef<T> for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to convert `Ahc<T>` instances to references.
    /// This method is useful for accessing the underlying value.
    /// ## Returns
    /// A reference to the underlying value.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// assert_eq!(value.as_ref(), &5);
    /// ```
    ///
    /// ## Note
    /// This method is implemented using `Arc::as_ref()`.
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

impl<T: PartialOrd> PartialOrd for Ahc<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to compare two `Ahc<T>` instances.
    /// This method is useful for sorting `Ahc<T>` instances.
    /// ## Parameters
    /// * `other`: Another `Ahc<T>` instance to compare with.
    /// ## Returns
    /// `Some(std::cmp::Ordering)` if the two instances are comparable, `None` otherwise.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value1 = table.hashcons(5);
    /// let value2 = table.hashcons(10);
    /// assert!(value1 < value2);
    /// ```
    ///
    /// ## Note
    /// This method is implemented using `Arc::partial_cmp()`.
    /// This method does not actually compare the underlying values. Instead, it
    /// compares the `Ahc<T>` instances.
    ///
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.elem.partial_cmp(&other.inner.elem)
    }
}

impl<T> Ord for Ahc<T>
where
    T: Ord + Hash + Eq,
{
    /// Provides the functionality to compare two `Ahc<T>` instances.
    /// This method is useful for sorting `Ahc<T>` instances.
    /// ## Parameters
    /// * `other`: Another `Ahc<T>` instance to compare with.
    /// ## Returns
    /// `std::cmp::Ordering` if the two instances are comparable.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    /// let table = AhcTable::new();
    /// let value1 = table.hashcons(5);
    /// let value2 = table.hashcons(10);
    /// assert!(value1 < value2);
    /// ```
    ///
    /// ## Note
    /// This method is implemented using `Arc::cmp()`.
    /// This method does not actually compare the underlying values. Instead, it
    /// compares the `Ahc<T>` instances.
    ///
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.elem.cmp(&other.inner.elem)
    }
}

///  # AhcTable<T>
/// A table structure for efficiently managing `Ahc<T>` instances.
/// This struct hides the underlying table and its reference count management.
///
/// This structure utilizes a HashMap to store `Ahc<T>` instances, offering
/// quick retrieval and management capabilities.
///
/// ## Type Parameters
/// * `T` - The type of values managed by the `Ahc<T>` instances within this table.
///
/// ## Fields
/// * `inner`: HashMap - The underlying data structure storing `Ahc<T>` instances.
///
pub struct AhcTable<T>
where
    T: Hash + Eq,
{
    inner: Arc<InnerTable<T>>,
}

// Implementing the traits for the custom smart pointer type.
impl<T> AhcTable<T>
where
    T: Hash + Eq,
{
    /// Creates a new `AhcTable`.
    ///
    /// ## Returns
    /// A new instance of `AhcTable<T>`.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    ///
    /// let table: AhcTable<i32> = AhcTable::new();
    /// ```
    pub fn new() -> Self {
        AhcTable {
            inner: Arc::new(InnerTable::new()),
        }
    }

    /// Simplifies object retrieval or creation with an intuitive interface.
    ///
    /// ## Parameters
    /// * `value`: The value to be managed.
    ///
    /// ## Returns
    /// A `Ahc<T>` instance corresponding to the given value.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    ///
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// ```
    ///
    pub fn hashcons(&self, value: T) -> Ahc<T> {
        Ahc {
            inner: self.intern(value),
        }
    }

    /// Internal method to manage the storage of values in `AhcTable`.
    /// It ensures that each value is stored only once, providing a shared
    /// reference to the stored value.
    ///
    /// ## Parameters
    /// * `value`: The value to be stored or retrieved.
    ///
    /// ## Returns
    /// A `Arc<Inner<T>>` pointer to the stored value.
    ///
    ///
    fn intern(&self, value: T) -> Arc<Inner<T>> {
        let arc_table = self.inner.clone();
        let arc_table_dup = arc_table.clone();

        let mut_table_result = arc_table_dup.table.write();

        let mut mut_table = match mut_table_result {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex is poisoned. Continuing with the poisoned lock.");
                poisoned.into_inner() // continues, because we will add a new value
            }
        };

        let rc_value = Arc::new(value);
        let rc_val_dup = rc_value.clone();

        match mut_table.entry(rc_val_dup) {
            Entry::Occupied(mut o) => {
                let weak_ahc = o.get();

                if let Some(rc_ahc) = weak_ahc.upgrade() {
                    return rc_ahc;
                }

                let elem = rc_value;

                let _table = Arc::downgrade(&arc_table);
                let new_elem = Arc::new(Inner { elem, _table });
                o.insert(Arc::downgrade(&new_elem));
                new_elem
            }

            Entry::Vacant(v) => {
                let _table = Arc::downgrade(&arc_table);
                let elem = rc_value;
                let new_elem = Arc::new(Inner { elem, _table });
                v.insert(Arc::downgrade(&new_elem));
                new_elem
            }
        }
    }

    #[cfg(not(feature = "auto-cleanup"))]
    /// Cleans up the `AhcTable`, removing any values that are no longer in use.
    /// This method is useful for managing memory and ensuring that unused
    /// values are not unnecessarily kept in the table.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    ///
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// drop(value);
    /// table.cleanup();
    /// ```
    ///
    pub fn cleanup(&self) {
        self.inner.cleanup();
    }

    /// Returns the number of elements currently stored in the `AhcTable`.
    ///
    /// ## Returns
    /// The number of elements in the `AhcTable`.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    ///
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// assert_eq!(table.len(), 1);
    /// ```
    ///
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> Clone for AhcTable<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to clone `AhcTable<T>` instances.
    /// ## Returns
    /// A new `AhcTable<T>` instance with the same values as the original.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    ///
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// let table_clone = table.clone();
    /// assert_eq!(table.len(), table_clone.len());
    /// ```
    ///
    fn clone(&self) -> Self {
        AhcTable {
            inner: self.inner.clone(),
        }
    }
}

/// # Inner<T>
/// A struct to encapsulate the inner workings of `Ahc<T>`.
/// It holds the actual value and a weak reference to its containing table.
///
/// ## Type Parameters
/// * `T` - The type of the encapsulated value.
///
/// ## Fields
/// * `elem`: The actual stored value.
/// * `_table`: A weak reference to the `AhcTable` that contains this value.
///
struct Inner<T>
where
    T: Hash + Eq,
{
    elem: Arc<T>,

    _table: Weak<InnerTable<T>>,
}

#[cfg(feature = "auto-cleanup")]
impl<T> Drop for Inner<T>
where
    T: Hash + Eq,
{
    /// Provides the functionality to drop `Inner<T>` instances.
    /// This method is useful for managing the lifecycle of `Ahc<T>` instances.
    /// ## Note
    /// This method is implemented using `Weak::upgrade()`.
    /// It removes the entry from the table if the table still exists.
    ///
    /// ## Example
    /// ```
    /// use hash_cons::AhcTable;
    ///
    /// let table = AhcTable::new();
    /// let value = table.hashcons(5);
    /// drop(value);
    /// table.cleanup();
    /// ```
    ///
    fn drop(&mut self) {
        let weak_table = self._table.clone();
        match weak_table.upgrade() {
            Some(arc_table) => {
                let key = self.elem.clone();
                let mut_table_result = arc_table.table.write();
                let mut mut_table = match mut_table_result {
                    Ok(guard) => guard,
                    Err(poisoned) => {
                        eprintln!("Mutex is poisoned. Continuing with the poisoned lock.");
                        poisoned.into_inner() // continues, because we are not using
                                              // any inconsistent value(if any)
                    }
                };
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
/// A helper struct to manage the internal storage of `AhcTable`.
/// It provides mechanisms to manage and access stored `Ahc<T>` instances.
///
/// ## Type Parameters
/// * `T` - The type of values stored in the `AhcTable`.
///
/// ## Fields
/// * `table`: The actual HashMap that stores the `Ahc<T>` instances.
///
struct InnerTable<T>
where
    T: Hash + Eq,
{
    table: RwLock<HashMap<Arc<T>, Weak<Inner<T>>>>,
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
            table: RwLock::new(HashMap::new()),
        }
    }

    /// Returns the number of elements currently stored in the `InnerTable`.
    /// ## Returns
    /// The number of elements in the `InnerTable`.
    ///
    fn len(&self) -> usize {
        let table_result = self.table.read();
        let table = match table_result {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex is poisoned. Continuing with the poisoned lock.");
                poisoned.into_inner() // continues, because we don't need the value(even if inconsistent)
            }
        };
        table.len()
    }

    #[cfg(not(feature = "auto-cleanup"))]
    /// Cleans up the `InnerTable`, removing any values that are no longer in use.
    /// This method is useful for managing memory and ensuring that unused
    /// values are not unnecessarily kept in the table.
    /// ## Note
    /// This method is implemented using `Weak::strong_count()`.
    /// It removes any values that have a `strong_count()` of 0.
    /// This is the desired behavior for hash consing.
    ///
    ///
    fn cleanup(&self) {
        loop {
            let mut_table_result = self.table.write();

            let mut mut_table = match mut_table_result {
                Ok(guard) => guard,
                Err(poisoned) => {
                    eprintln!("Mutex is poisoned. Continuing with the poisoned lock.");
                    poisoned.into_inner() // continues, because we are removing the value
                }
            };

            // Flag to check if any weak references are dropped in this iteration
            let mut dropped = false;

            mut_table.retain(|_, weak_ahc: &mut Weak<Inner<T>>| {
                if weak_ahc.strong_count() == 0 {
                    dropped = true; // A weak reference was dropped
                    false // Remove this entry
                } else {
                    true // Keep this entry
                }
            });

            // Break the loop if no weak references were dropped in this iteration
            if !dropped {
                break;
            }
        }
    }

    /*fn cleanup(&self) {
        let mut_table_result = self.table.write();

        let mut mut_table = match mut_table_result {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Mutex is poisoned. Continuing with the poisoned lock.");
                poisoned.into_inner() // continues, because we are removing the value
            }
        };

        mut_table.retain(|_, weak_ahc: &mut Weak<Inner<T>>| weak_ahc.strong_count() > 0);
    }*/
}
