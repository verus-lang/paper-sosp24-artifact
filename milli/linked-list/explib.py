import time
import os
import subprocess
import sys

SAMPLES = 20

REPEAT_OUT_PATH = "repeat-out"

def run_command(cmd, filename, success_text, my_env):
    start_time = time.time()
    try:
        # print(' '.join(cmd + [filename]))
        result = subprocess.run(cmd + [filename], capture_output = True, timeout=100, env=my_env)
    except subprocess.TimeoutExpired:
        return None
    end_time = time.time()
    # print(result.stdout)
    # print(result.stderr)
    if (not success_text in result.stdout) and (not success_text in result.stderr):
        print(result.stdout)
        print(result.stderr[:100])
        sys.exit(-1)
    elapsed_time = end_time - start_time
    return elapsed_time

def write_file(tool, suffix, code):
    out_filename = REPEAT_OUT_PATH + "/" + f"{tool}{suffix}"
    with open(out_filename, "w") as f:
        f.write(code)
        f.flush()
    return out_filename

def run_command_on_code(tool, suffix, code, cmd, success_text, my_env):
    out_filename = write_file(tool, suffix, code)
    elapsed_time = run_command(cmd, out_filename, success_text, my_env)
    return elapsed_time
