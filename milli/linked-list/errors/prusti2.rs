// From https://viperproject.github.io/prusti-dev/user-guide/tour/generics.html

// !!SCRIPT!!PRE_BEGIN!!
// !!MAIN!!PRE_BEGIN!!
use prusti_contracts::*;

pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[extern_spec(std::mem)]
#[ensures(snap(dest) === src)]
#[ensures(result === old(snap(dest)))]
fn replace<T>(dest: &mut T, src: T) -> T;

#[extern_spec]
impl<T> std::option::Option<T> {
    #[requires(self.is_some())]
    #[ensures(old(self) === Some(result))]
    pub fn unwrap(self) -> T;
    
    #[pure]
    #[ensures(result == matches!(self, None))]
    pub const fn is_none(&self) -> bool;

    #[pure]
    #[ensures(result == matches!(self, Some(_)))]
    pub const fn is_some(&self) -> bool;

    #[ensures(result === old(snap(self)))]
    #[ensures(self.is_none())]
    pub fn take(&mut self) -> Option<T>;
}

pub trait PrustiClone where Self: Sized {
    #[ensures(result === old(snap(self)))]
    fn clone(&self) -> Self;
}

impl<T: PrustiClone> LinkedList<T> {
    #[pure]
    #[trusted]
    pub fn len(&self) -> usize {
        link_len(&self.head)
    }

    #[pure]
    #[trusted]
    fn is_empty(&self) -> bool {
        matches!(self.head, None)
    }

    #[ensures(result.len() == 0)]
    #[trusted]
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    #[pure]
    #[requires(index < self.len())]
    #[trusted]
    // Return type is changed from `T` to `&T`
    pub fn lookup(&self, index: usize) -> &T {
        link_lookup(&self.head, index)
    }

// !!SCRIPT!!PRE_END!!
    #[trusted]
    #[ensures(self.len() == old(self.len()) + 1)]
    #[ensures(snap(self.lookup(0)) === elem)] // Here we add a `snap`
    #[ensures(forall(|i: usize| (i < old(self.len())) ==>
        old(self.lookup(i)) === self.lookup(i + 1)))]
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    // #[ensures(self.len() == old(self.len()) + 4)]
    // #[ensures(snap(self.lookup(0)) === elem)]
    // #[ensures(snap(self.lookup(1)) === elem)]
    // #[ensures(snap(self.lookup(2)) === elem)]
    // #[ensures(snap(self.lookup(3)) === elem)]
    // #[ensures(forall(|i: usize| (i < old(self.len())) ==>
    //     old(self.lookup(i)) === self.lookup(i + 4)))]
    // pub fn push4(&mut self, elem: T) {
    //     let new_node = Box::new(Node {
    //         elem: elem.clone(),
    //         next: self.head.take(),
    //     });
    //     self.head = Some(new_node);
    //     prusti_assert!(self.len() == old(self.len()) + 1);
    //     prusti_assert!(snap(self.lookup(0)) === elem);
    //     prusti_assert!(forall(|i: usize| (i < old(self.len())) ==>
    //         old(self.lookup(i)) === self.lookup(i + 1)));

    //     let new_node = Box::new(Node {
    //         elem: elem.clone(),
    //         next: self.head.take(),
    //     });
    //     self.head = Some(new_node);
    //     prusti_assert!(self.len() == old(self.len()) + 2);
    //     prusti_assert!(snap(self.lookup(0)) === elem);
    //     prusti_assert!(snap(self.lookup(1)) === elem);
    //     prusti_assert!(forall(|i: usize| (i < old(self.len())) ==>
    //         old(self.lookup(i)) === self.lookup(i + 2)));

    //     let new_node = Box::new(Node {
    //         elem: elem.clone(),
    //         next: self.head.take(),
    //     });
    //     self.head = Some(new_node);
    //     prusti_assert!(self.len() == old(self.len()) + 3);
    //     prusti_assert!(snap(self.lookup(0)) === elem);
    //     prusti_assert!(snap(self.lookup(1)) === elem);
    //     prusti_assert!(snap(self.lookup(2)) === elem);
    //     prusti_assert!(forall(|i: usize| (i < old(self.len())) ==>
    //         old(self.lookup(i)) === self.lookup(i + 3)));

    //     let new_node = Box::new(Node {
    //         elem: elem.clone(),
    //         next: self.head.take(),
    //     });
    //     self.head = Some(new_node);
    // }

    predicate! {
        // two-state predicate to check if the head of a list was correctly removed
        fn head_removed(&self, prev: &Self) -> bool {
            self.len() == prev.len() - 1 // The length will decrease by 1
            && forall(|i: usize| // Every element will be shifted forwards by one
                (1 <= i && i < prev.len())
                    ==> prev.lookup(i) === self.lookup(i - 1))
        }
    }

    #[ensures(old(self.is_empty()) ==>
        result.is_none() &&
        self.is_empty()
    )]
    #[trusted]
    #[ensures(!old(self.is_empty()) ==>
        self.head_removed(&old(snap(self)))
        &&
        result === Some(snap(old(snap(self)).lookup(0)))
    )]
    // Return type changed from `Option<i32>`
    #[trusted]
    pub fn try_pop(&mut self) -> Option<T> {
        // ...
        match self.head.take() { // Replace mem::swap with the buildin Option::take
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    /* !!SCRIPT!!ERRORS!!1!! */ #[requires(!self.is_empty())]
    #[ensures(self.head_removed(&old(snap(self))))]
    #[ensures(result === old(snap(self)).lookup(0))]
    #[trusted]
    // Return type changed from `i32`
    pub fn pop(&mut self) -> T {
        self.try_pop().unwrap()
    }
    
    // #[requires(!self.is_empty())]
    // #[ensures(result === snap(self).lookup(ix))]
    // pub fn index(&self, ix: usize) -> &T {
    //     let mut cur = &self.head;
    //     let mut i = 0;
    //     while i < ix {
    //         cur = match cur {
    //             None => unreachable!(),
    //             Some(ref node) => &node.next,
    //         };
    //         i += 1;
    //     }
    //     match cur {
    //         None => unreachable!(),
    //         Some(node) => &node.elem,
    //     }
    // }
    // 

    // #[requires(ix < self.len())]
    #[ensures(result === snap(self).lookup(ix))]
    pub fn index(&self, ix: usize) -> &T {
        index_link_lookup(&self.head, ix)
    }
    
// !!SCRIPT!!POST_BEGIN!!!
}

// /* !!SCRIPT!!ERRORS!!2!! */ #[requires(index < link_len(link))]
#[ensures(result === link_lookup(link, index))]
fn index_link_lookup<T>(link: &Link<T>, index: usize) -> &T {
    match link {
        Some(node) => {
            if index == 0 {
                &node.elem
            } else {
                link_lookup(&node.next, index - 1)
            }
        }
        None => unreachable!(),
    }
}

#[pure]
#[requires(index < link_len(link))]
// Return type is changed from `T` to `&T`
#[trusted]
fn link_lookup<T>(link: &Link<T>, index: usize) -> &T {
    match link {
        Some(node) => {
            if index == 0 {
                // Here we return a reference to `elem` instead of the `elem` itself
                &node.elem
            } else {
                link_lookup(&node.next, index - 1)
            }
        }
        None => unreachable!(),
    }
}

#[pure]
#[trusted]
fn link_len<T>(link: &Link<T>) -> usize {
    match link {
        None => 0,
        Some(node) => 1 + link_len(&node.next),
    }
}

impl PrustiClone for u64 {
    // #[ensures(result === old(snap(self)))]
    #[trusted]
    fn clone(&self) -> Self {
        *self
    }
}
// !!SCRIPT!!POST_END!!!
// !!MAIN!!PRE_END!!

#[trusted]
fn main() {
    let mut l = LinkedList::new();
    l.push(1);
    l.push(0);
    prusti_assert!(l.len() == 2);
    let v = l.pop();
    prusti_assert!(l.len() == 1);
    // prusti_assert!(v == 0);
}
