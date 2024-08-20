def main_code(n):
    p = lambda i: """
    l1.push_back({i});
    l2.push_back({i});
    l3.push_back({i});
    l4.push_back({i});
    """.format(i=i)
    return ("""
mod main {
    use creusot_contracts::*;
    use super::linked_list::LinkedList;

    fn main() {
        let mut l1 = LinkedList::new();
        let mut l2 = LinkedList::new();
        let mut l3 = LinkedList::new();
        let mut l4 = LinkedList::new();
    """ +
    "".join(p(i) for i in range(n)) + """
    }
}"""
    )