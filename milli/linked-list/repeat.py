import os
import tempfile
import subprocess
import sys
import shutil
import statistics

PRUSTI_CACHE = 1
CARGO_CREUSOT = 2
PRUSTI_ENV = 4
FSTAR_MODULES = 8

COMMANDS = {
    "dafny":  ([os.environ.get('EVAL_DAFNY_EXE'), "verify", "--cores", "1", "--boogie-filter", "*Main*"], []),
    "verus":  ([os.environ.get('EVAL_VERUS_EXE'), "--num-threads", "1", "--verify-module=main", "--crate-type=lib"], []),
    "prusti": ([os.environ.get('EVAL_PRUSTI_EXE'), "--edition=2018"], [PRUSTI_ENV]),
    "Fstarlowstar": ([os.environ.get('EVAL_FSTARLOWSTAR_EXE'),
        "--include", os.environ.get('EVAL_FSTARLOWSTAR_KRML'), "--cache_dir", os.environ.get('EVAL_FSTARLOWSTAR_KRML_OBJ')], [FSTAR_MODULES]),
    "creusot": ([os.environ.get('EVAL_CREUSOT_EXE'),], [CARGO_CREUSOT]),
}

from explib import *

REPEAT_OUT_PATH = "repeat-out"

PRUSTI_CACHE_PATH = "prusti-cache"

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

def collect(tool, filename, suffix, success_text):
    m = load_module("repeat." + filename + ".py")
    pre_code = ""
    
    cmd, opt = COMMANDS[tool]
    if FSTAR_MODULES in opt:
        base_code = extract_pre(filename.replace('Main', ''))
        with open(REPEAT_OUT_PATH + '/Fstarlowstar.fst', 'w') as f:
            f.write(base_code)
            f.flush()
    else:
        pre_code = extract_pre(filename)
    my_env = os.environ.copy()

    if PRUSTI_ENV in opt:
        my_env["DEFAULT_PRUSTI_SERVER_MAX_CONCURRENCY"] = "1"

    out_filename = REPEAT_OUT_PATH + "/" + f"{tool}{suffix}"
    # if PRUSTI_CACHE in opt:
    #     main_code = m.main_code(0)
    #     code = pre_code + main_code
    #     my_env["DEFAULT_PRUSTI_CACHE_PATH"] = PRUSTI_CACHE_PATH
    #     run_command_on_code(tool, suffix, code, cmd, success_text, my_env)
    #     shutil.copy(PRUSTI_CACHE_PATH, "prusti-cache-copy")

        # with open(out_filename, "w") as f:
        #     f.write(code)
        #     f.flush()
        # result = subprocess.run(cmd + [out_filename], capture_output = True, env=my_env)
        # if (not success_text in result.stdout) and (not success_text in result.stderr):
        #     print(result.stdout)
        #     print(result.stderr)
        #     sys.exit(-1)
        # result = subprocess.run(cmd + [out_filename], capture_output = True, env=my_env)
        # if (not success_text in result.stdout) and (not success_text in result.stderr):
        #     print(result.stdout)
        #     print(result.stderr)
        #     sys.exit(-1)

    for i in range(1, 16 + 1):
        main_code = m.main_code(i)
        code = pre_code + main_code

        code = code.replace('// !!SCRIPT!!TRUSTED!! ', '')
        # print(code)

        times = []
        for r in range(SAMPLES):
            # if PRUSTI_CACHE in opt:
            #     shutil.copy("prusti-cache-copy", PRUSTI_CACHE_PATH)
            if CARGO_CREUSOT in opt:
                elapsed_time = run_command(cmd, f"creusot-sessions/linked-list-repeat-{i:02}", success_text, my_env)
            elif FSTAR_MODULES in opt:
                out_filename = REPEAT_OUT_PATH + '/FstarlowstarMain.fst'
                with open(out_filename, 'w') as f:
                    f.write(code)
                    f.flush()
                elapsed_time = run_command(cmd, out_filename, success_text, my_env)
            else:
                elapsed_time = run_command_on_code(tool, suffix, code, cmd, success_text, my_env)

            print(f"{i},{r},{tool},{elapsed_time}", file=sys.stderr)
            # print(elapsed_time)
            if elapsed_time is None:
                times.append(float('inf'))
                break
            else:
                times.append(elapsed_time)

        ptool = tool.capitalize()
        result = statistics.median(times)
        print(f"{i},{ptool},{result}")
        print(f"> {i},{ptool},{result}", file=sys.stderr)
        if result == float('inf'):
            break

if __name__ == "__main__":
    if os.path.exists(PRUSTI_CACHE_PATH):
        os.remove(PRUSTI_CACHE_PATH)

    if os.path.exists(REPEAT_OUT_PATH):
        shutil.rmtree(REPEAT_OUT_PATH)
    os.mkdir(REPEAT_OUT_PATH)

    collect("dafny", "dafny.dfy", ".dfy", b'0 errors')
    collect("verus", "verus.rs", ".rs", b'0 errors')
    collect("prusti", "prusti.rs", ".rs", b'Successful verification')
    collect("Fstarlowstar", "FstarlowstarMain.fst", ".fst", b'All verification conditions discharged successfully')
    collect("creusot", "creusot.rs", ".rs", b'replay OK')
