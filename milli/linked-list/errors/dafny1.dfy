// !!PUSH!!PRE_BEGIN!!
// !!MAIN!!PRE_BEGIN!!
module NativeTypes {
ghost const maxUInt64 := 0x1_0000_0000_0000_0000
newtype{:nativeType "ulong"} uint64 = i:int | 0 <= i < maxUInt64}

module LinkedList {
import NativeTypes

class Node<T> {
  var v: T
  var next: Node?<T>
  ghost var repr: set<object>

  ghost predicate ReprInv()
    reads this, repr
    decreases repr
  {
    && (this in repr)
    && (next != null ==> (
      && next in repr
      && this !in next.repr
      && repr == {this} + next.repr
      && next.ReprInv()
    ))
  }

  function I(): seq<T>
    requires ReprInv()
    reads repr
    decreases repr
    ensures |I()| == 1 <==> next == null
  {
    [v] + if next != null then
      next.I()
    else
      []
  }

  constructor (v_: T, next_: Node?<T>)
    requires next_ != null ==> next_.ReprInv()
    ensures v == v_ && next == next_
    ensures ReprInv()
    ensures repr == {this} + if next_ != null then next_.repr else {}
  {
    v := v_;
    next := next_;
    repr := {this} + if next_ != null then next_.repr else {};
  }
}

class LinkedList<T> {
  var head: Node?<T>
  ghost var contents: seq<T>
  ghost var repr: set<object>

  ghost predicate Inv()
    reads this, repr
  {
    && this in repr
    && (|contents| == 0 <==> head == null)
    && |contents| <= NativeTypes.maxUInt64
    && (head != null ==> (
      && head in repr
      && this !in head.repr
      && repr == {this} + head.repr
      && head.ReprInv()
      && contents == head.I()
    ))
  }
// !!PUSH!!PRE_END!!

  constructor ()
    ensures contents == []
    ensures Inv()
    ensures forall o :: o in this.repr <==> fresh(o)
  {
    head := null;
    contents := [];
    repr := {this};
  }

  method Push(v: T)
    modifies this
    requires Inv()
    requires |contents| < NativeTypes.maxUInt64
    ensures [v] + old(contents) == contents
    ensures Inv()
    ensures forall o :: o in this.repr ==> (o in old(this.repr) || fresh(o))
    // ensures exists o :: fresh(o) && this.repr == old(this.repr) + o
    // ensures fresh(newO) && this.repr == old(this.repr) + {newO}
  {
    // assert(head != null ==> this !in head.repr);
    var newHead := new Node(v, head);
    // assert(this !in newHead.repr);
    // assert(newHead.Inv());
    // assume(false);
    head := newHead;
    contents := [v] + contents;
    repr := {this} + head.repr;
  }

  // method Push4(v: T)
  //   modifies this
  //   requires Inv()
  //   requires |contents| < NativeTypes.maxUInt64 - 3
  //   ensures [v, v, v, v] + old(contents) == contents
  //   ensures Inv()
  // {
  //   // assert(head != null ==> this !in head.repr);
  //   var newHead := new Node(v, head);
  //   // assert(this !in newHead.repr);
  //   // assert(newHead.Inv());
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;

  //   // assert(head != null ==> this !in head.repr);
  //   newHead := new Node(v, head);
  //   // assert(this !in newHead.repr);
  //   // assert(newHead.Inv());
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;

  //   // assert(head != null ==> this !in head.repr);
  //   newHead := new Node(v, head);
  //   // assert(this !in newHead.repr);
  //   // assert(newHead.Inv());
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;

  //   // assert(head != null ==> this !in head.repr);
  //   newHead := new Node(v, head);
  //   // assert(this !in newHead.repr);
  //   // assert(newHead.Inv());
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;

  //   assert([v, v, v, v] + old(contents) == contents);
  // }

  // method Push4(v: T)
  //   modifies this
  //   requires Inv()
  //   requires |contents| < NativeTypes.maxUInt64 - 3
  //   ensures [v, v, v, v] + old(contents) == contents
  //   ensures Inv()
  // {
  //   var newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;
  //   assert(head.I() == contents);
  //   assert([v] + old(contents) == contents);

  //   newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;
  //   assert(head.I() == contents);
  //   assert([v, v] + old(contents) == contents);

  //   newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;
  //   assert(head.I() == contents);
  //   assert([v, v, v] + old(contents) == contents);

  //   newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;
  //   repr := {this} + head.repr;
  //   assert(head.I() == contents);
  //   assert([v, v, v, v] + old(contents) == contents);
  // }

  // method Push4(v: T)
  //   modifies this
  //   requires Inv()
  //   requires |contents| < NativeTypes.maxUInt64 - 3
  //   ensures [v, v, v, v] + old(contents) == contents
  //   ensures Inv()
  // {
  //   var newHead := new Node(v, head);
  //   head := newHead;

  //   newHead := new Node(v, head);
  //   head := newHead;

  //   newHead := new Node(v, head);
  //   head := newHead;

  //   newHead := new Node(v, head);
  //   head := newHead;

  //   contents := [v, v, v, v] + contents;
  //   assert([v, v, v, v] + old(contents) == contents);
  //   repr := {this} + head.repr;
  // }

  // method Push4(v: T)
  //   modifies this
  //   requires Inv()
  //   requires |contents| < NativeTypes.maxUInt64 - 3
  //   ensures [v, v, v, v] + old(contents) == contents
  //   ensures Inv()
  // {
  //   var newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;

  //   newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;

  //   newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;

  //   newHead := new Node(v, head);
  //   head := newHead;
  //   contents := [v] + contents;

  //   assert([v, v, v, v] + old(contents) == contents);
  //   repr := {this} + head.repr;
  // }
  
  method Pop() returns (res: T)
    modifies this
    // /* !!SCRIPT!!ERRORS!!1!! */ requires |contents| > 0
    requires Inv()
    ensures res == old(contents[0])
    ensures contents == old(contents[1..])
    ensures Inv()
  {
    // assert(head != null);
    res := head.v;
    head := head.next;
    // if |contents| > 1 {
    //   assert(head != null);
    // } else {
    //   assert(head == null);
    // }
    contents := contents[1..];
    repr := {this} + if head != null then head.repr else {};
  }

  // TODO use machine integers
  method Index(ix: NativeTypes.uint64) returns (res: T)
    requires Inv()
    /* !!SCRIPT!!ERRORS!!2!! */ requires ix as nat < |contents|
    decreases *
  {
    assert(ix as nat < NativeTypes.maxUInt64);
    var cur := this.head;
    var i: NativeTypes.uint64 := 0;
    while true
      decreases *
      invariant i <= ix
    {
      if i == ix {
        res := cur.v;
        break;
      }

      i := i + 1;
    }
  }
// !!PUSH!!POST_BEGIN!!
}

export LinkedList provides NativeTypes, LinkedList.contents, LinkedList.Inv, LinkedList.repr, LinkedList.Push, LinkedList.Pop, LinkedList.Index reveals LinkedList
}
// !!PUSH!!POST_END!!
// !!MAIN!!PRE_END!!

module Main {
import opened LinkedList
method Main() {
  var l := new LinkedList();
  // TODO use machine integers
  l.Push(1);
  l.Push(0);
  assert(|l.contents| == 2);
  var v := l.Pop();
  assert(|l.contents| == 1);
  // assert(v == 0);
}
}

