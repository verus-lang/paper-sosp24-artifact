def main_code(n):
    p = lambda i: """
    l1.push_front({i});
    l2.push_front({i});
    l3.push_front({i});
    l4.push_front({i});
    """.format(i=i)
    return ("""
verus! {
mod main {
use vstd::prelude::*;
use super::doubly_linked_list::{DoublyLinkedList, Iterator};
pub fn run() {
    let mut l1 = DoublyLinkedList::<u32>::new();
    let mut l2 = DoublyLinkedList::<u32>::new();
    let mut l3 = DoublyLinkedList::<u32>::new();
    let mut l4 = DoublyLinkedList::<u32>::new();
    """ +
    "".join(p(i) for i in range(n)) + """
}
}
}"""
    )
