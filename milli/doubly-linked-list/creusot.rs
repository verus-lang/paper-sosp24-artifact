
// !!MAIN!!PRE_BEGIN!!

mod lemmas {
    use creusot_contracts::logic::{FMap, Int, Mapping, Seq};
    use creusot_contracts::*;
    
    // #[law]
    // #[open(self)]
    // #[ensures(x.set(k, v1).set(k, v2) == x.set(k, v2))]
    // pub fn map_set_overwrite<K, V>(x: Mapping<K, V>, k: K, v1: V, v2: V) {}
    
    #[law]
    #[open(self)]
    #[requires(k1 != k2)]
    #[ensures(x.set(k1, v1).set(k2, v2) == x.set(k2, v2).set(k1, v1))]
    pub fn map_set_commute<K, V>(x: Mapping<K, V>, k1: K, k2: K, v1: V, v2: V) {}
    
}

mod linked_list {
    use super::lemmas::*;
    
    use ::std::ptr;
    use creusot_contracts::__stubs::fin;
    use creusot_contracts::ghost_ptr::{GhostPtrExt, GhostPtrToken};
    use creusot_contracts::logic::FMap;
    use creusot_contracts::*;
    
    struct Node<T> {
        data: T,
        next: *const Node<T>,
        prev: *const Node<T>,
    }
    
    pub struct LinkedList<T> {
        head: *const Node<T>,
        tail: *const Node<T>,
        ptrs: Ghost<Seq<*const Node<T>>>,
        token: GhostPtrToken<Node<T>>,
    }
    
    impl<T> LinkedList<T> {
        #[logic]
        #[open(self)]
        #[requires(i >= 0)]
        fn prev_of(self, i: Int) -> *const Node<T> {
            if i == 0 {
                <*const Node<T>>::null_logic()
            } else {
                self.ptrs[i - 1]
            }
        }

        #[logic]
        #[open(self)]
        #[requires(i >= 0)]
        fn next_of(self, i: Int) -> *const Node<T> {
            if i + 1 == self.ptrs.len() {
                <*const Node<T>>::null_logic()
            } else {
                self.ptrs[i + 1]
            }
        }

        #[predicate]
        #[open(self)]
        #[requires(i >= 0 && i < self.ptrs.len())]
        fn wf_token(self, i: Int) -> bool {
            self.token.shallow_model().contains(self.ptrs[i])
            && self.token.shallow_model().lookup(self.ptrs[i]).prev == self.prev_of(i)
            && self.token.shallow_model().lookup(self.ptrs[i]).next == self.next_of(i)
        }

        #[predicate]
        #[open(self)]
        pub fn invariant(self) -> bool {
            pearlite! {
                (forall<i: Int> (0 <= i && i < self.ptrs.len()) ==> self.wf_token(i))
                && if self.ptrs.len() == 0 {
                    self.head == <*const Node<T>>::null_logic() &&
                    self.tail == <*const Node<T>>::null_logic()
                } else {
                    self.head == self.ptrs[0]
                    && self.tail == self.ptrs[self.ptrs.len() - 1]
                }
            }
        }
    
        #[ghost]
        #[open(self)]
        pub fn model(self) -> Seq<T> {
            pearlite! {
                Seq::new((*self.ptrs).len(), 
                    |i| self.token.shallow_model().lookup(self.ptrs[i]).data)
            }
            // if self.head == <*const Node<T>>::null_logic() {
            //     Seq::EMPTY
            // } else {
            //     lseg_forward_seq(
            //         self.head,
            //         self.tail,
            //         self.token.shallow_model().remove(self.tail),
            //     )
            //     .concat(Seq::singleton(
            //         self.token.shallow_model().lookup(self.tail).data,
            //     ))
            // }
        }
    
        #[ensures(result.invariant())]
        #[ensures(result.model() == Seq::EMPTY)]
        pub fn new() -> Self {
            let this = LinkedList {
                head: ptr::null(),
                tail: ptr::null(),
                ptrs: gh!(Seq::EMPTY),
                token: GhostPtrToken::new(),
            };
            proof_assert!(this.model().ext_eq(Seq::EMPTY));
            this
        }
    
        #[ensures(result.invariant())]
        #[ensures(result.model().ext_eq(Seq::singleton(v)))]
        pub fn singleton(v: T) -> Self {
            // map_set_commute::<*const Node<T>, Option<Node<T>>>;
            let mut token = GhostPtrToken::new();
            let node = Node {
                data: v,
                next: ptr::null(),
                prev: ptr::null(),
            };
            let ptr = token.ptr_from_box(Box::new(node));
            LinkedList {
                head: ptr,
                tail: ptr,
                ptrs: gh!(Seq::singleton(ptr)),
                token,
            }
        }

    
        #[requires((*self).invariant())]
        #[requires((*self).model().len() > 0)]
        #[ensures((^self).invariant())]
        #[ensures(Seq::singleton(result).concat((^self).model()).ext_eq((*self).model()))]
        pub fn pop_front(&mut self) -> T {
            let self_ghost_old = gh!(*self);
            map_set_commute::<*const Node<T>, Option<Node<T>>>;
            if self.head.is_null() {
                unreachable!();
            } else {
                proof_assert!(self.head == self.ptrs[0]);
                proof_assert!(self.ptrs.len() > 0);
                proof_assert!(self.token.shallow_model().contains(self.ptrs[0]));
                let node = self.token.ptr_to_box(self.head);
                proof_assert!(forall<i: _> (self_ghost_old.token.shallow_model().contains(i) && i != self.ptrs[0]) ==> self.token.shallow_model().contains(i));
                proof_assert!(forall<i: Int> (1 <= i && i < self.ptrs.len()) ==> self.token.shallow_model().contains(self.ptrs[i]));
                self.head = node.next;
                if self.head.is_null() {
                    self.tail = ptr::null();
                    self.ptrs = gh!(self.ptrs.subsequence(1, self.ptrs.len()));
                } else {
                    proof_assert!(self.ptrs.len() > 1);
                    proof_assert!(self.token.shallow_model().subset(self_ghost_old.token.shallow_model()));
                    proof_assert!(self.wf_token(1));
                    proof_assert!(self.token.shallow_model().contains(self.ptrs[1]));
                    proof_assert!(self.head == self.ptrs[1]);
                    let new_head = self.token.ptr_as_mut(self.head);
                    new_head.prev = ptr::null();
                    self.ptrs = gh!(self.ptrs.subsequence(1, self.ptrs.len()));
                    proof_assert!(forall<i: Int> (0 <= i && i < self.ptrs.len() && self_ghost_old.wf_token(i + 1)) ==> self.wf_token(i));
                    proof_assert!(forall<i: Int> (1 <= i && i < self.ptrs.len()) ==> self.wf_token(i));
                }
                // proof_assert!(self.model().ext_eq(self_ghost_old.model().subsequence(1, self_ghost_old.model().len())));
                proof_assert!(node.data == self_ghost_old.model()[0]);
                node.data
            }
        }

        #[requires((*self).invariant())]
        #[requires((*self).model().len() > 0)]
        #[ensures((^self).invariant())]
        #[ensures((^self).model().concat(Seq::singleton(result)).ext_eq((*self).model()))]
        pub fn pop_back(&mut self) -> T {
            let self_ghost_old = gh!(*self);
            map_set_commute::<*const Node<T>, Option<Node<T>>>;
            if self.tail.is_null() {
                unreachable!();
            } else {
                let node = self.token.ptr_to_box(self.tail);
                proof_assert!(forall<i: _> (self_ghost_old.token.shallow_model().contains(i) && i != self.ptrs[self.ptrs.len() - 1]) ==> self.token.shallow_model().contains(i));
                proof_assert!(forall<i: Int> (0 <= i && i < self.ptrs.len() - 1) ==> self.token.shallow_model().contains(self.ptrs[i]));
                self.tail = node.prev;
                if self.tail.is_null() {
                    self.head = ptr::null();
                } else {
                    proof_assert!(self.ptrs.len() > 1);
                    proof_assert!(self.token.shallow_model().subset(self_ghost_old.token.shallow_model()));
                    proof_assert!(self.wf_token(self.ptrs.len() - 2));
                    proof_assert!(self.token.shallow_model().contains(self.ptrs[self.ptrs.len() - 2]));
                    proof_assert!(self.tail == self.ptrs[self.ptrs.len() - 2]);
                    let new_tail = self.token.ptr_as_mut(self.tail);
                    new_tail.next = ptr::null();
                }
                self.ptrs = gh!(self.ptrs.subsequence(0, self.ptrs.len() - 1));
                proof_assert!(forall<i: Int> (0 <= i && i < self.ptrs.len()) ==> self_ghost_old.wf_token(i) ==> self.wf_token(i));
                node.data
            }
        }
    
        #[requires((*self).invariant())]
        #[requires(other.invariant())]
        #[ensures((^self).invariant())]
        #[ensures((^self).model().ext_eq((*self).model().concat(other.model())))]
        pub fn append(&mut self, mut other: Self) {
            let old_self = gh!(self);
            let old_other = gh!(other);
            if self.head.is_null() {
                *self = other
            } else if !other.head.is_null() {
                let tail = self.token.ptr_as_mut(self.tail);
                tail.next = other.head;
                let head = other.token.ptr_as_mut(other.head);
                head.prev = self.tail;

                self.token.merge(other.token);
                self.tail = other.tail;
                self.ptrs = gh!(self.ptrs.concat(*other.ptrs));
                proof_assert!(forall<a: FMap<*const Node<T>, Node<T>>, b: _> a.disjoint(b) ==> a.subset(a.union(b)));

                proof_assert!(forall<i: Int> (0 <= i && i < old_self.ptrs.len() - 1) ==>
                    old_self.token.shallow_model().lookup(old_self.ptrs[i]) ==
                    self.token.shallow_model().lookup(old_self.ptrs[i]));
                proof_assert!(forall<i: Int> (1 <= i && i < old_other.ptrs.len()) ==>
                    old_other.token.shallow_model().lookup(old_other.ptrs[i]) ==
                    self.token.shallow_model().lookup(old_other.ptrs[i]));
                proof_assert!(self.token.shallow_model().lookup(old_self.ptrs[old_self.ptrs.len() - 1]).prev ==
                    old_self.token.shallow_model().lookup(old_self.ptrs[old_self.ptrs.len() - 1]).prev);
                proof_assert!(self.token.shallow_model().lookup(old_other.ptrs[0]).next ==
                    old_other.token.shallow_model().lookup(old_other.ptrs[0]).next);
                proof_assert!(self.wf_token(old_self.ptrs.len() - 1));
                proof_assert!(self.wf_token(old_self.ptrs.len()));

                proof_assert!(forall<i: Int> (0 <= i && i < old_self.ptrs.len() - 1) ==> old_self.wf_token(i) ==> self.wf_token(i));
                proof_assert!(forall<i: Int> (1 <= i && i < old_other.ptrs.len()) ==> old_other.wf_token(i) ==> self.wf_token(i + old_self.ptrs.len()));
            }
        }
    
        #[requires((*self).invariant())]
        #[ensures((^self).invariant())]
        #[ensures((^self).model().ext_eq((*self).model().concat(Seq::singleton(val))))]
        pub fn push_back(&mut self, val: T) {
            self.append(Self::singleton(val))
        }
        
        #[requires((*self).invariant())]
        #[ensures((^self).invariant())]
        #[ensures((^self).model().ext_eq(Seq::singleton(val).concat((*self).model())))]
        pub fn push_front(&mut self, val: T) {
            let mut this = Self::singleton(val);
            std::mem::swap(self, &mut this);
            self.append(this);
        }
    
        #[requires((*self).invariant())]
        #[requires(self.ptrs.len() > 0)]
        #[ensures(result.invariant())]
        // #[ensures(*result.index == 0)]
        // #[ensures(result.model() == self.model())]
        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                l: &self,
                curr: self.head,
                index: gh!(0),
            }
        }
    }
    
    pub struct Iter<'a, T> {
        pub l: &'a LinkedList<T>,
        curr: *const Node<T>,
        pub index: Ghost<Int>,
    }
    
    impl<'a, T> Iter<'a, T> {
        #[predicate]
        #[open(self)]
        pub fn invariant(self) -> bool {
            pearlite! {
                if self.curr != <*const Node<T>>::null_logic() {
                    self.l.invariant()
                    && *self.index < self.l.model().len()
                    && self.curr == self.l.ptrs[*self.index]
                    && self.l.ptrs.len() > 0
                    && *self.index >= 0
                } else {
                    true
                }
            }
        }
    
        // #[ghost]
        // #[open(self)]
        // pub fn model(self) -> Seq<T> {
        //     LinkedList {
        //         head: self.curr,
        //         tail: *self.tail,
        //         token: *self.token,
        //         ptrs: self.ptrs,
        //     }
        //     .model()
        // }
    
        #[requires((*self).invariant())]
        #[ensures((^self).invariant())]
        #[ensures(match result {
            Some(val) => *val == self.l.model()[*self.index],
            None => true,
        })]
        #[ensures(match result {
            Some(val) => *(^self).index == *(*self).index + 1,
            None => true,
        })]
        pub fn next(&mut self) -> Option<&'a T> {
            map_set_commute::<*const Node<T>, Option<Node<T>>>;
            if self.curr.is_null() {
                return None;
            }

            proof_assert!(self.l.wf_token(*self.index));
            let node = self.l.token.ptr_as_ref(self.curr);
            self.curr = node.next;
            self.index = gh!(*self.index + 1);
            Some(&node.data)
        }
    }
}
// !!MAIN!!PRE_END!!

mod main {
    use creusot_contracts::*;
    use super::linked_list::LinkedList;

    fn main() {
        let mut l1 = LinkedList::new();
        l1.push_back(1);
        l1.push_back(0);
        // proof_assert!(l1@.len() == 2);
        let _ = l1.pop_back();
        // proof_assert!(l1@.len() == 1);
    }

}
