use std::{cmp::Ordering, collections::HashMap};

/// Trait to uniquely identify elements in bheap.
pub trait Uid {
    /// Unique identifier for the implementing struct. The same value must
    /// be returned in all invocations of this method on a given struct.
    fn uid(&self) -> u64;
}

/// A re-prioritizable binary max heap containing a buffer for storing elements
/// and a hashmap index for keeping track of element positions.
pub struct BinaryMaxHeap<T>
where
    T: Ord + Uid,
{
    /// in-memory storage for elements
    buffer: Vec<T>,

    /// mapping from element uids to positions in the heap buffer
    index: HashMap<u64, usize>,
}

impl<T> BinaryMaxHeap<T>
where
    T: Ord + Uid,
{
    /// Creates a new BinaryMaxHeap from a given vector, which may or may not be
    /// empty. If the vector already contains elements, the elements are
    /// re-arranged with a `build_heap()` operation.
    pub fn from_vec(buffer: Vec<T>) -> Self {
        let mut bheap = BinaryMaxHeap {
            buffer,
            index: HashMap::new(),
        };

        if !bheap.is_empty() {
            bheap.build_heap();
        }

        bheap
    }

    /// Creates an empty binary max heap with no elements.
    pub fn new() -> Self {
        BinaryMaxHeap::from_vec(vec![])
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Swaps elements at the given indices byt first swapping the elements
    /// in the buffer vector and next updating the `HashMap` index with
    /// new indices.
    #[inline]
    fn swap_elems_at_indices(&mut self, i: usize, j: usize) {
        let index = &mut self.index;

        index.insert(self.buffer[i].uid(), j);
        index.insert(self.buffer[j].uid(), i);

        self.buffer.swap(i, j);
    }

    /// Convenience method for comparing elements at the given indices.
    #[inline]
    fn cmp(&self, i: usize, j: usize) -> Ordering {
        self.buffer[i].cmp(&self.buffer[j])
    }

    /// Restores heap property by moving the element in the given index
    /// upwards along it's parents to the root, until it has no parents
    /// or it is <= to its parents.
    /// It operates in the following manner:
    /// ```text
    /// heapify_up(heap, i) {
    ///     while i > 0 {
    ///         let parent = (i - 1) / 2;
    ///         if heap[i] > heap[parent] {
    ///             swap(heap, i, parent)
    ///         } else { break; }
    ///     }
    /// }
    /// ```
    fn heapify_up(&mut self, idx: usize) -> Option<usize> {
        let mut i = idx;

        while i > 0 {
            let parent = (i - 1) / 2;

            if let Ordering::Greater = self.cmp(i, parent) {
                self.swap_elems_at_indices(i, parent);
                i = parent;
            } else {
                break;
            };
        }

        if i != idx {
            return Some(i);
        } else {
            return None;
        }
    }

    /// Restores heap property by moving the element at the given index,
    /// downwards along it's children, towards the leaves, until it
    /// has no children or it is >= to its children.
    /// It operates in the following manner:
    /// ```text
    /// heapify_dn(heap, i) {
    ///     while i < len(heap) / 2 {
    ///         let max = i;
    ///         let lc, rc = 2 * i + 1, 2 * i + 2;
    ///
    ///         if lc < len(heap) && heap[max] < lc { max = lc; }
    ///         if rc < len(heap) && heap[max] < rc { max = rc; }
    ///
    ///         if i != max { swap(heap, i, max); i = max; }
    ///         else { break; }
    ///     }
    /// }
    /// ```
    fn heapify_dn(&mut self, idx: usize) -> Option<usize> {
        let mut i = idx;

        while i < (self.len() / 2) {
            let mut max = i;
            let (lc, rc) = (2 * i + 1, 2 * i + 2);

            if lc < self.len() {
                if let Ordering::Less = self.cmp(max, lc) {
                    max = lc;
                }
            }

            if rc < self.len() {
                if let Ordering::Less = self.cmp(max, rc) {
                    max = rc;
                }
            }

            if i != max {
                self.swap_elems_at_indices(i, max);
                i = max;
            } else {
                break;
            }
        }

        if i != idx {
            return Some(i);
        } else {
            return None;
        }
    }

    /// Corrects the `HashMap` index for the given heap position,
    /// by updating the entry with the uid of the element at that
    /// position. The values stored is the index.
    /// Concisely:
    /// ```text
    /// index[buffer[i].uid()] = i
    /// ```
    #[inline]
    fn update_index(&mut self, i: usize) -> Option<usize> {
        if i >= self.len() {
            return None;
        }

        self.index.insert(self.buffer[i].uid(), i)
    }

    /// Returns a mutable reference to the element at the givven
    /// heap position, if present. This implementation assumes
    /// that no mutation used with respect to the returned
    /// mutable reference, modifies the uid() property for the
    /// element.
    pub fn get(&mut self, i: usize) -> Option<&mut T> {
        if i >= self.len() {
            return None;
        }

        Some(&mut self.buffer[i])
    }

    /// Pushes a new element in this priority queue.
    pub fn push(&mut self, elem: T) {
        let idx = self.buffer.len();

        self.buffer.push(elem);
        self.update_index(idx);

        self.heapify_up(idx);
    }

    /// Peeks at the element with highest priority, if present.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.buffer[0])
    }

    /// Pops the element with the highest property, if present.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let elem = self.buffer.swap_remove(0);
        self.index.remove(&elem.uid());

        self.update_index(0);
        self.heapify_dn(0);

        Some(elem)
    }

    /// Builds the `HashMap` index from uids to buffer positions.
    pub fn build_index(&mut self) {
        for i in 0..self.len() {
            self.update_index(i);
        }
    }

    /// Builds a heap from un-organized elements in the in-memory buffer.
    /// It operates as follows:
    /// ```text
    /// build_heap(heap) {
    ///     build_index(heap);
    ///
    ///     for i = len(heap) / 2; i >= 0; i-- {
    ///         heapify_dn(heap, i);
    ///     }
    /// }
    /// ````
    pub fn build_heap(&mut self) {
        self.build_index();

        for i in (0..(self.len() / 2)).rev() {
            self.heapify_dn(i);
        }
    }

    /// Restores heap property at the given position.
    pub fn restore_heap_property(&mut self, idx: usize) -> Option<usize> {
        if idx >= self.len() {
            return None;
        }

        self.heapify_up(idx).or(self.heapify_dn(idx))
    }

    /// Returns the position for element with given uid in the heap buffer.
    pub fn index_in_heap_from_uid(&self, uid: u64) -> Option<usize> {
        self.index.get(&uid).map(|&elem_idx| elem_idx)
    }

    /// Returns the position for element in the heap buffer.
    pub fn index_in_heap(&self, elem: &T) -> Option<usize> {
        self.index.get(&elem.uid()).map(|&elem_idx| elem_idx)
    }
}
