import os
import tempfile
import subprocess
import sys
import shutil

from explib import *

REPEAT_OUT_PATH = "repeat-creusot"

def load_module(filename):
    import importlib.util
    spec = importlib.util.spec_from_file_location("m", filename)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def extract_code(filename, begin_marker, end_marker):
    with open(filename, "r") as f:
        lines = f.readlines()
        start = next(i for i, line in enumerate(lines) if begin_marker in line)
        end = next(i for i, line in enumerate(lines) if end_marker in line)
        return "".join(lines[start+1:end])

def extract_pre(filename):
    pre_code = extract_code(filename, "!!MAIN!!PRE_BEGIN!!", "!!MAIN!!PRE_END!!")
    # post_code = extract_code(filename, "!!SCRIPT!!POST_BEGIN!!", "!!SCRIPT!!POST_END!!")
    return pre_code

def emit(tool, filename, suffix):
    assert(tool == "creusot")

    m = load_module("repeat." + filename + ".py")
    pre_code = extract_pre(filename)
    my_env = os.environ.copy()

    for i in range(1, 16 + 1):
        main_code = m.main_code(i)
        code = pre_code + main_code

        code_filename = write_file(tool, suffix, code)
        # script_dir = os.path.dirname(os.path.realpath(__file__))
        # subprocess.run(['bash', '-c', 'eval $(opam env) && export DISPLAY=\':1\' && cd ../creusot/why-3-driver && . ./venv3/bin/activate && python3 run.py {f}'.format(f=script_dir + "/" + code_filename)], capture_output=True)

        shutil.copy(code_filename, '../creusot/cargo-dir/src/lib.rs')
        subprocess.run(['bash', '-c', 'cd ../creusot/cargo-dir && eval $(opam env) && cargo creusot'])
        shutil.copy('../creusot/cargo-dir/target/debug/cargo_dir-rlib.mlcfg', f'creusot-sessions/linked-list-repeat-{i:02}.mlcfg')

        out_filename = REPEAT_OUT_PATH + "/" + f"{tool}-{i:02}{suffix}"
        with open(out_filename, "w") as f:
            f.write(code)
            f.flush()

if __name__ == "__main__":
    if os.path.exists(REPEAT_OUT_PATH):
        shutil.rmtree(REPEAT_OUT_PATH)
    os.mkdir(REPEAT_OUT_PATH)

    emit("creusot", "creusot.rs", ".rs")