def main_code(n):
    lsf = lambda j: "; ".join([f"{k}l" for k in range(j)])

    p = lambda i: """
  DLL.push #Int32.t #(G.hide [{l}]) l1 {i}l;
  DLL.push #Int32.t #(G.hide [{l}]) l2 {i}l;
  DLL.push #Int32.t #(G.hide [{l}]) l3 {i}l;
  DLL.push #Int32.t #(G.hide [{l}]) l4 {i}l;
    """.format(i=i, l=lsf(i))
    return ("""
module FstarlowstarMain
open LowStar.BufferOps

module DLL = Fstarlowstar

module B = LowStar.Buffer
module HS = FStar.HyperStack
module G = FStar.Ghost
module L = FStar.List.Tot
module U32 = FStar.UInt32
module MO = LowStar.Modifies

open FStar.HyperStack.ST

#push-options "--z3rlimit 600 --fuel 2 --ifuel 1"

val main2: unit -> ST (Prims.unit) (fun _ -> true) (fun _ _ _ -> true)

let main2 () =
  let l1: B.pointer_or_null (DLL.t Int32.t) = B.malloc HS.root B.null 1ul in
  let l2: B.pointer_or_null (DLL.t Int32.t) = B.malloc HS.root B.null 1ul in
  let l3: B.pointer_or_null (DLL.t Int32.t) = B.malloc HS.root B.null 1ul in
  let l4: B.pointer_or_null (DLL.t Int32.t) = B.malloc HS.root B.null 1ul in""" +
    "".join(p(i) for i in range(n)) + """
  ()
"""
    )