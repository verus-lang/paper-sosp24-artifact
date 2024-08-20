import os
import tempfile
import subprocess
import sys
import shutil
import statistics
import time

PRUSTI_CACHE = 1
CARGO_CREUSOT = 2
PRUSTI_ENV = 4
FSTAR_PRE = 8
DAFNY_FILTER = 16
VERUS_FILTER = 32

COMMANDS = {
    "dafny":  ([os.environ.get('EVAL_DAFNY_EXE'), "verify", "--cores", "8"], [DAFNY_FILTER]),
    "verus":  ([os.environ.get('EVAL_VERUS_EXE'), "--num-threads", "8"], [VERUS_FILTER]),
    "prusti": ([os.environ.get('EVAL_PRUSTI_EXE'), "--edition=2018"], [PRUSTI_ENV]),
    "Fstarlowstar": ([os.environ.get('EVAL_FSTARLOWSTAR_EXE'),
        "--include", os.environ.get('EVAL_FSTARLOWSTAR_KRML'), "--cache_dir", os.environ.get('EVAL_FSTARLOWSTAR_KRML_OBJ')], [FSTAR_PRE]),
    "creusot": ([os.environ.get('EVAL_CREUSOT_EXE'),], [CARGO_CREUSOT]),
}

ERRORS_OUT_PATH = "errors-out"

PRUSTI_CACHE_PATH = "prusti-cache"

def run_command(cmd, filename, success_text, my_env):
    # print(cmd, file=sys.stderr)
    start_time = time.time()
    try:
        # print(' '.join(cmd + [filename]))
        result = subprocess.run(cmd + [filename], capture_output = True, timeout=600, env=my_env)
    except subprocess.TimeoutExpired:
        return None
    end_time = time.time()
    # print(result.stdout, file=sys.stderr)
    # print(result.stderr, file=sys.stderr)
    if (not success_text in result.stdout) and (not success_text in result.stderr):
        print(result.stdout)
        print(result.stderr[:100])
        sys.exit(-1)
    elapsed_time = end_time - start_time
    return elapsed_time

def write_file(tool, suffix, code):
    out_filename = ERRORS_OUT_PATH + "/" + f"{tool}{suffix}"
    with open(out_filename, "w") as f:
        f.write(code)
        f.flush()
    return out_filename

def run_command_on_code(tool, suffix, code, cmd, success_text, my_env):
    out_filename = write_file(tool, suffix, code)
    elapsed_time = run_command(cmd, out_filename, success_text, my_env)
    return elapsed_time

def collect(tool, filename, suffix, success_text):
    with open(filename, "r") as f:
        base_code = f.read()

    cmd, opt = COMMANDS[tool]
    base_cmd = list(cmd)
    my_env = os.environ.copy()
    if PRUSTI_ENV in opt:
        my_env["DEFAULT_PRUSTI_SERVER_MAX_CONCURRENCY"] = "8"

    out_filename = ERRORS_OUT_PATH + "/" + f"{tool}{suffix}"

    for mode in ['base', 'error']:
        for i in [1, 2]:
            # code = base_code.replace(f'/* !!SCRIPT!!ERRORS!!{i}!! */', '//')
            # print(code)

            times = []
            # if tool != "creusot":
            #     repetitions = 20
            # else:
            #     repetitions = 4
            for r in range(20):
                cmd = list(base_cmd)
                if CARGO_CREUSOT in opt:
                    if mode == 'base':
                        elapsed_time = run_command(cmd, f"errors/creusot-base-{i}", success_text, my_env)
                    elif mode == 'error':
                        elapsed_time = run_command(cmd, f"errors/creusot-requires-{i}", success_text, my_env)
                    else:
                        assert(False)
                elif FSTAR_PRE in opt:
                    if i == 1:
                        cmd += ["--admit_except", "Fstarlowstar1.pop"]
                    elif i == 2:
                        cmd += ["--admit_except", "Fstarlowstar2.index"]
                    else:
                        assert(False)
                    if mode == 'base':
                        elapsed_time = run_command(cmd, f"Fstarlowstar.fst", success_text, my_env)
                    elif mode == 'error':
                        elapsed_time = run_command(cmd, f"errors/Fstarlowstar{i}.fst", success_text, my_env)
                    else:
                        assert(False)
                elif PRUSTI_ENV in opt:
                    if mode == 'base':
                        elapsed_time = run_command(cmd, f"errors/{tool}base{i}{suffix}", success_text, my_env)
                    elif mode == 'error':
                        elapsed_time = run_command(cmd, f"errors/{tool}{i}{suffix}", success_text, my_env)
                    else:
                        assert(False)
                else:
                    if DAFNY_FILTER in opt:
                        if i == 1:
                            cmd += ["--boogie-filter", "*Pop*"]
                        elif i == 2:
                            cmd += ["--boogie-filter", "*Index*"]
                        else:
                            assert(False)
                    if VERUS_FILTER in opt:
                        if i == 1:
                            cmd += ["--verify-module", "linked_list", "--verify-function", "LinkedList::pop"]
                        elif i == 2:
                            cmd += ["--verify-module", "linked_list", "--verify-function", "LinkedList::index"]
                        else:
                            assert(False)
                        
                    # print(cmd)
                    if mode == 'base':
                        elapsed_time = run_command(cmd, f"{tool}{suffix}", success_text, my_env)
                    elif mode == 'error':
                        elapsed_time = run_command(cmd, f"errors/{tool}{i}{suffix}", success_text, my_env)
                    else:
                        assert(False)
                    # elapsed_time = run_command_on_code(tool, suffix, code, cmd, success_text, my_env)

                print(f"{mode},{i},{r},{tool},{elapsed_time}", file=sys.stderr)
                # print(elapsed_time)
                if elapsed_time is None:
                    times.append(float('inf'))
                    break
                else:
                    times.append(elapsed_time)

            ptool = tool.capitalize()
            result = statistics.median(times)
            print(f"{mode},{i},{ptool},{result}")
            print(f"> {mode},{i},{ptool},{result}", file=sys.stderr)

if __name__ == "__main__":
    if os.path.exists(ERRORS_OUT_PATH):
        shutil.rmtree(ERRORS_OUT_PATH)
    os.mkdir(ERRORS_OUT_PATH)
    if os.path.exists(PRUSTI_CACHE_PATH):
        os.remove(PRUSTI_CACHE_PATH)

    collect("dafny", "dafny.dfy", ".dfy", b'errors')
    collect("verus", "verus.rs", ".rs", b'errors')
    collect("prusti", "prusti.rs", ".rs", b'')
    collect("Fstarlowstar", "Fstarlowstar.fst", ".fst", b'')
    collect("creusot", "creusot.rs", ".rs", b'replay')
