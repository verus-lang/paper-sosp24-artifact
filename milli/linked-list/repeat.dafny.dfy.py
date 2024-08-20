def main_code(n):
    p = lambda i: """
    l1.Push({i});
    l2.Push({i});
    l3.Push({i});
    l4.Push({i});
    assert(l1.repr !! l2.repr !! l3.repr !! l4.repr);
    """.format(i=i)
    return ("""
module Main {
import opened LinkedList
method Main() {
    var l1 := new LinkedList();
    var l2 := new LinkedList();
    var l3 := new LinkedList();
    var l4 := new LinkedList();
    """ +
    "".join(p(i) for i in range(n)) + """
}
}"""
    )