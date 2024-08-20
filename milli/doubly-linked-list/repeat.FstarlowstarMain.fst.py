def main_code(n):
    num_lists = 4

    prefix = """
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
"""
    dlists = "\n".join(
        [
            f"  let d{i} : DLL.dll UInt32.t = DLL.dll_new () in"
            for i in range(1, num_lists + 1)
        ]
    )
    nodes = "\n".join(
        [
            f"  let n{i}_{j} = DLL.node_of {j}ul in"
            for i in range(1, num_lists + 1)
            for j in range(n)
        ]
    )
    inserts = "\n".join(
        [
            f"  DLL.dll_insert_at_tail d{i} n{i}_{j};"
            for i in range(1, num_lists + 1)
            for j in range(n)
        ]
    )
    suffix = """
  HST.pop_frame ()
"""

    return (prefix + dlists + "\n" + nodes + "\n" + inserts + suffix).strip()


if __name__ == "__main__":
    print(main_code(2))
