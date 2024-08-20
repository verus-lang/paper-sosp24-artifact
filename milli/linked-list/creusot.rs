// !!MAIN!!PRE_BEGIN!!
extern crate creusot_contracts;
use creusot_contracts::{logic::Int, *};

struct Node<V> {
    v: V,
    next: Option<Box<Node<V>>>,
}

// This doesn't work due to https://github.com/xldenis/creusot/issues/911
// impl<V> Node<V> {
//     #[ghost]
//     fn len_logic(self: Node<V>) -> Int {
//         {
//             1 + match self.next {
//                 Some(next) => next.len_logic(),
//                 None => 0,
//             }
//         }
//     }
// }

impl<V> Node<V> {
    #[ghost]
    fn len_logic(self: Node<V>) -> Int {
        {
            let Node { v: _, next } = self;
            1 + match next {
                Some(next) => next.len_logic(),
                None => 0,
            }
        }
    }
    
    #[ensures(result@.ext_eq(Seq::singleton(v).concat(match next {
        None => Seq::EMPTY,
        Some(n) => n@,
    })))]
    fn new(v: V, next: Option<Box<Node<V>>>) -> Self {
        Node {
            v,
            next,
        }
    }
}

impl<V> ShallowModel for Node<V> {
    type ShallowModelTy = Seq<V>;

    #[open(crate)] #[ghost]
    fn shallow_model(self) -> Seq<V> {
        pearlite! {
            let Node { v, next } = self;
            Seq::singleton(v).concat(match next {
                None => Seq::EMPTY,
                Some(node) => (*node)@,
            })
        }
    }
}

struct LinkedList<V> {
    head: Option<Box<Node<V>>>,
}

impl<V> ShallowModel for LinkedList<V> {
    type ShallowModelTy = Seq<V>;

    #[open(crate)] #[ghost]
    fn shallow_model(self) -> Self::ShallowModelTy {
        pearlite! {
            match self.head {
                None => Seq::EMPTY,
                Some(node) => node@,
            }
        }
    }    
}

// #[ghost]
// #[requires(s.len() > 0)]
// #[ensures(Seq::singleton(s[0]).concat(s.tail()).ext_eq(s))]
// fn seq_lemma<T>(s: Seq<T>) {
// }

impl<V> LinkedList<V> {
    #[ensures(result@ == Seq::<V>::EMPTY)]
    pub fn new() -> Self
    {
        LinkedList {
            head: None,
        }   
    }

    //#[requires(self@ != Seq::<V>::EMPTY)]
    #[requires(self@.len() > 0)]
    #[ensures(result == self@[0])]
    #[ensures((^self)@.ext_eq((*self)@.subsequence(1, (*self)@.len())))]
    pub fn pop(&mut self) -> V
    {
        // proof_assert!(match self.head {
        //     Some(n) => n@ == (*self)@,
        //     None => false,
        // });
        let pre = gh! { *self };
        let h = self.head.take().unwrap();
        // proof_assert!(h@ == pre@);
        proof_assert!({
            let Node { v, next } = *h;
            h@.ext_eq(Seq::singleton(v).concat({
                match next {
                    Some(n) => n@,
                    None => Seq::EMPTY,
                }
            }))
        });
        // proof_assert!({
        //     let Node { v, next: _ } = *h;
        //     h@[0] == v
        // });
        // proof_assert!(Seq::singleton(hv[0]).concat(hv.subsequence(1, hv.len())) == *hv);
        // gh!(seq_lemma(h@));
        // proof_assert!({
        //     let Node { v: _, next } = *h;
        //     match next {
        //         Some(n) => n@.ext_eq(h@.subsequence(1, h@.len())),
        //         None => true,
        //     }
        // });
        // gh!(seq_lemma(pre@));
        self.head = h.next;
        // proof_assert!({
        //     let LinkedList { head } = *self;
        //     match head {
        //         Some(n) => n@.ext_eq(pre@.subsequence(1, pre@.len())),
        //         None => true,
        //     }
        // });
        h.v
    }
    
    #[ensures((^self)@.ext_eq(Seq::singleton(v).concat((*self)@)))]
    pub fn push(&mut self, v: V)
    {
        let pre = gh! { *self };
        let next = self.head.take();
        proof_assert!({
            match next {
                Some(n) => n@.ext_eq(pre@),
                None => true,
            }
        });
        let n = Node::new(v, next);
        proof_assert!({
            let Node { v: vv, next } = n;
            n@.ext_eq(Seq::singleton(vv).concat({
                match next {
                    None => Seq::EMPTY,
                    Some(nn) => (*nn)@,
                }
            }))
        });
        self.head = Some(Box::new(n));
        proof_assert!({
            let LinkedList { head } = *self;
            match head {
                Some(n1) => {
                    let Node { v, next } = *n1;
                    n1@.ext_eq(Seq::singleton(v).concat(match next {
                        Some(n2) => n2@,
                        None => Seq::EMPTY,
                    }))
                },
                None => true,
            }
        });
    }
    
    #[requires(ix@ < self@.len())]
    #[requires(0 <= ix@ && ix@ < self@.len())]
    #[ensures(*result == self@[ix@])]
    pub fn index(&self, ix: usize) -> &V {
        let mut cur = &self.head;
        let mut i: usize = 0;
        #[invariant(0 <= i@ && i@ <= ix@)]
        #[invariant(match cur {
            Some(c) => c@.ext_eq(self@.subsequence(i@, self@.len())),
            None => false,
        })]
        while i < ix {
            cur = match cur {
                Some(c) => {
                    let Node { v: _, next: ref next } = &**c;
                    next
                },
                None => unreachable!(),
            };
            i += 1;
        }
        // proof_assert!(i == ix);
        // &cur.as_ref().unwrap().v
        proof_assert!(match cur {
            Some(c) => c@.ext_eq(self@.subsequence(i@, self@.len())),
            None => false,
        });
        match cur {
            Some(c) => {
                proof_assert!({
                    let Node { v: vv, next } = **c;
                    c@.ext_eq(Seq::singleton(vv).concat({
                        match next {
                            None => Seq::EMPTY,
                            Some(nn) => (*nn)@,
                        }
                    }))
                });

                let Node { v, next: _ } = &**c;
                proof_assert!(*v == self@.subsequence(i@, self@.len())[0]);
                v
            },
            None => unreachable!(),
        }
    }
}
// !!MAIN!!PRE_END!!

fn main() {
    let mut l1 = LinkedList::new();
    l1.push(1);
    l1.push(0);
    proof_assert!(l1@.len() == 2);
    let _ = l1.pop();
    proof_assert!(l1@.len() == 1);
}
