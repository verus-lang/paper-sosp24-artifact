import time
import os
import subprocess
import sys

SAMPLES = int(os.environ.get('EVAL_SAMPLES'))

REPEAT_OUT_PATH = "repeat-out"

def run_command(cmd, filenames, success_text, my_env, timeout=None):
    start_time = time.time()
    try:
        result = subprocess.run(cmd + filenames, capture_output = True, env=my_env, timeout=timeout)
        # print(result)
    except subprocess.TimeoutExpired:
        return None
    end_time = time.time()
    if (not success_text in result.stdout) and (not success_text in result.stderr):
        print(result.stdout, file=sys.stderr)
        print(result.stderr[:100], file=sys.stderr)
        return None
    elapsed_time = end_time - start_time
    return elapsed_time

def write_file(tool, suffix, code):
    out_filename = REPEAT_OUT_PATH + "/" + f"{tool}{suffix}"
    with open(out_filename, "w") as f:
        f.write(code)
        f.flush()
    return out_filename

def run_command_on_code(tool, suffix, code, cmd, success_text, my_env, timeout=None):
    out_filename = write_file(tool, suffix, code)
    elapsed_time = run_command(cmd, [out_filename], success_text, my_env, timeout=timeout)
    return elapsed_time
