import os
import tempfile
import subprocess
import sys
import shutil
import statistics

CARGO_CREUSOT = 2
PRUSTI_ENV = 4

COMMANDS = {
    "dafny":  ([os.environ.get('EVAL_DAFNY_EXE'), "verify", "--cores", "1"], []),
    "verus":  ([os.environ.get('EVAL_VERUS_EXE'), "--num-threads", "1"], []),
    "prusti": ([os.environ.get('EVAL_PRUSTI_EXE'), "--edition=2018"], [PRUSTI_ENV]),
    "Fstarlowstar": ([os.environ.get('EVAL_FSTARLOWSTAR_EXE'),
        "--include", os.environ.get('EVAL_FSTARLOWSTAR_KRML'), "--cache_dir", os.environ.get('EVAL_FSTARLOWSTAR_KRML_OBJ')], []),
    "creusot": ([os.environ.get('EVAL_CREUSOT_EXE'),], [CARGO_CREUSOT]),
}

from explib import *

def collect(tool, filename, suffix, success_text):
    cmd, opt = COMMANDS[tool]
    my_env = os.environ.copy()
    if PRUSTI_ENV in opt:
        my_env["DEFAULT_PRUSTI_SERVER_MAX_CONCURRENCY"] = "1"

    times = []
    for r in range(SAMPLES):
        if CARGO_CREUSOT in opt:
            elapsed_time = run_command(cmd, 'creusot-sessions/linked-list-oneshot', success_text, my_env)
        else:
            elapsed_time = run_command(cmd, filename, success_text, my_env)
        if elapsed_time is None:
            times.append(float('inf'))
            break
        else:
            times.append(elapsed_time)

        print(f"{tool},{r},{elapsed_time}", file=sys.stderr)

    tool = tool.capitalize()
    result = statistics.median(times)
    print(f"\\newcommand{{\\evalLinkedList{tool}}}{{{result:.2f}}}")
    print(f"> \\newcommand{{\\evalLinkedList{tool}}}{{{result:.2f}}}", file=sys.stderr)

if __name__ == "__main__":
    collect("dafny", "dafny.dfy", ".dfy", b'0 errors')
    collect("verus", "verus.rs", ".rs", b'0 errors')
    collect("prusti", "prusti.rs", ".rs", b'Successful verification')
    collect("Fstarlowstar", "Fstarlowstar.fst", ".fst", b'All verification conditions discharged successfully')
    collect("creusot", "creusot.rs", ".rs", b'replay OK')
