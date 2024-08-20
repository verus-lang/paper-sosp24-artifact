def main_code(n):
    p = lambda i: """
    l1.push({i});
    l2.push({i});
    l3.push({i});
    l4.push({i});
    """.format(i=i)
    return ("""
fn main() {
    let mut l1 = LinkedList::new();
    let mut l2 = LinkedList::new();
    let mut l3 = LinkedList::new();
    let mut l4 = LinkedList::new();
    """ +
    "".join(p(i) for i in range(n)) + """
}"""
    )