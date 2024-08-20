// !!PUSH!!PRE_BEGIN!!
// !!MAIN!!PRE_BEGIN!!
use vstd::prelude::*;

verus! {

pub trait VerusClone where Self: Sized {
    fn clone(&self) -> (r: Self)
        ensures r == self;
}

mod linked_list {
use super::VerusClone;
use vstd::prelude::*;

struct Node<V> {
    v: V,
    next: Option<Box<Node<V>>>,
}

impl<V> Node<V> {
    closed spec fn view(&self) -> Seq<V>
        decreases self,
    {
        seq![self.v] + match self.next {
            Some(n) => n.view(),
            None => seq![],
        }
    }
}

pub struct LinkedList<V> {
    head: Option<Box<Node<V>>>,
}

impl<V> View for LinkedList<V> {
    type V = Seq<V>;

    closed spec fn view(&self) -> Seq<V>
        decreases self,
    {
        match self.head {
            Some(h) => h.view(),
            None => seq![],
        }
    }
}

impl<V: VerusClone> LinkedList<V> {
    pub fn new() -> (res: Self)
        ensures res@ == Seq::<V>::empty(),
    {
        LinkedList {
            head: None,
        }   
    }
// !!PUSH!!PRE_END!!

    // &mut -> B.pointer
    pub fn pop(&mut self) -> (res: V)
        // /* !!SCRIPT!!ERRORS!!1!! */ requires old(self)@.len() > 0,
        ensures
            res == old(self)@[0],
            self@ == old(self)@.skip(1),
    {
        let h = self.head.take().unwrap();
        // assert(h@ == seq![h.v] + match h.next {
        //     Some(n) => n.view(),
        //     None => seq![],
        // });
        // assert(h@[0] == h.v);
        // assert(match h.next {
        //     Some(n) => n@ =~= h@.skip(1),
        //     None => h@.skip(1) =~= Seq::<V>::empty(),
        // });
        self.head = h.next;
        assert(self@ =~= old(self)@.skip(1));
        h.v
    }

    pub fn push(&mut self, v: V)
        ensures
            self@ == seq![v] + old(self)@,
    {
        let next = self.head.take();
        self.head = Some(Box::new(Node { v, next }));
    }

    // pub fn push_4(&mut self, v: V)
    //     ensures self@ == seq![v, v, v, v] + old(self)@,
    // {
    //     let next = self.head.take();
    //     self.head = Some(Box::new(Node { v: v.clone(), next }));
    //     assert(seq![v] + old(self)@ =~= self@);

    //     let next = self.head.take();
    //     self.head = Some(Box::new(Node { v: v.clone(), next }));
    //     assert(seq![v, v] + old(self)@ =~= self@);

    //     let next = self.head.take();
    //     self.head = Some(Box::new(Node { v: v.clone(), next }));
    //     assert(seq![v, v, v] + old(self)@ =~= self@);

    //     let next = self.head.take();
    //     self.head = Some(Box::new(Node { v, next }));
    //     assert(seq![v, v, v, v] + old(self)@ =~= self@);
    // }

    pub open spec fn len(&self) -> nat {
        self@.len()
    }

    fn index(&self, ix: usize) -> (v: &V)
        /* !!SCRIPT!!ERRORS!!2!! */ requires ix < self.len(),
        ensures v == self@[ix as int],
    {
        let mut cur = &self.head;
        let mut i = 0;
        loop
            invariant
                i <= ix < self.len(),
                cur.is_some() && cur.unwrap()@ =~= self@.skip(i as int),
            ensures
                i == ix,
                cur.is_some() && cur.unwrap()@ =~= self@.skip(i as int),
        {
            if i == ix {
                break;
            }
            // assert(cur.unwrap()@ =~= self@.skip(i as int));
            // assert(cur.unwrap().next.is_some());
            // assert(cur.unwrap()@ =~= seq![cur.unwrap().v] + cur.unwrap().next.unwrap()@);
            assert(cur.unwrap().next.unwrap()@ =~= cur.unwrap()@.skip(1));
            // assert(cur.unwrap().next.unwrap()@ =~= self@.skip(i as int + 1));
            cur = &cur.as_ref().unwrap().next;
            i += 1;
            // assert(cur.is_some());
            // assert(cur.unwrap()@ =~= self@.skip(i as int));
        }
        // assert(cur.unwrap()@[0] == self@.skip(i as int)[0]);
        assert(cur.unwrap()@ == seq![cur.unwrap().v] + match cur.unwrap().next {
            Some(n) => n.view(),
            None => seq![],
        });
        assert(cur.unwrap()@[0] == cur.unwrap().v);
        &cur.as_ref().unwrap().v
    }
}

impl<T: Copy> VerusClone for T {
    fn clone(&self) -> (r: Self) {
        *self
    }
}

}

}
// !!MAIN!!PRE_END!!

verus! {
use linked_list::LinkedList;
pub fn main() {
    let mut l = LinkedList::new();
    l.push(1);
    l.push(0);
    assert(l.len() == 2);
    let v = l.pop();
    assert(l.len() == 1);
    // assert(v == 0);
}
}