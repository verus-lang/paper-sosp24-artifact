def main_code(n):
    p = lambda i: """
    l1.InsertTail({i});
    l2.InsertTail({i});
    l3.InsertTail({i});
    l4.InsertTail({i});
    """.format(i=i)
    return ("""
module Main{
  import opened DoublyLinkedList

  method Main() {
    var l1 := new DoublyLinkedList();
    var l2 := new DoublyLinkedList();
    var l3 := new DoublyLinkedList();
    var l4 := new DoublyLinkedList();
    """ +
    "".join(p(i) for i in range(n)) + """
  }
}
"""
    )
