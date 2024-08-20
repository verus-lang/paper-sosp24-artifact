module FstarlowstarMain
open LowStar.BufferOps

module DLL = FstarlowstarIface

module B = LowStar.Buffer
module HS = FStar.HyperStack
module HST = FStar.HyperStack.ST
module G = FStar.Ghost
module L = FStar.List.Tot
module U32 = FStar.UInt32
module MO = LowStar.Modifies

open FStar.HyperStack.ST

// A lower rlimit works, but give it more so that we can test scaling
#push-options "--z3rlimit 1000 --fuel 0 --ifuel 0"

val main2: unit -> ST (Prims.unit) (fun _ -> true) (fun _ _ _ -> true)

let main2 () =
  HST.push_frame ();
  let d1 : DLL.dll UInt32.t = DLL.dll_new () in
  let d2 : DLL.dll UInt32.t = DLL.dll_new () in
  let d3 : DLL.dll UInt32.t = DLL.dll_new () in
  let d4 : DLL.dll UInt32.t = DLL.dll_new () in
  let n1_0 = DLL.node_of 0ul in
  let n1_1 = DLL.node_of 1ul in
  let n2_0 = DLL.node_of 0ul in
  let n2_1 = DLL.node_of 1ul in
  let n3_0 = DLL.node_of 0ul in
  let n3_1 = DLL.node_of 1ul in
  let n4_0 = DLL.node_of 0ul in
  let n4_1 = DLL.node_of 1ul in
  DLL.dll_insert_at_tail d1 n1_0;
  DLL.dll_insert_at_tail d1 n1_1;
  DLL.dll_insert_at_tail d2 n2_0;
  DLL.dll_insert_at_tail d2 n2_1;
  DLL.dll_insert_at_tail d3 n3_0;
  DLL.dll_insert_at_tail d3 n3_1;
  DLL.dll_insert_at_tail d4 n4_0;
  DLL.dll_insert_at_tail d4 n4_1;
  HST.pop_frame ()
