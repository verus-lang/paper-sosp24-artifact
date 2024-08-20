#!/usr/bin/python3
#
# jonh 2023-12-06 ported from
# Ironclad/ironfleet/tools/scripts/dafny-line-count.py
# which evidently hasn't been touched in 8 years.
# I updated it and then tuned the counting to the IronKV-host-only
# task needed for the verus osdi 2023 submission.
# In particular, note that DafnyFile.is_spec relabels
# certain files, mostly Protocol layer definitions, as spec,
# since the Verus IronKV treats the protocol layer as spec.
#
# jonh 2024-08-20 adjusted data flow to fit into verus sosp 2024 paper
# artifact.

import sys
import os
import time
import fileinput
import shutil
import re
import argparse
import subprocess
import pickle
import json

script_dir = os.path.dirname(__file__)

class DafnyFile:
  def __init__(self, filename):
    self.filename = os.path.normpath(filename.strip().replace('\\', '/'))
    self.spec = 0
    self.impl = 0
    self.proof = 0

  def __repr__(self):
    return "%s %s spec %s impl %s proof" % (
        #"SPEC" if self.is_spec() else "    ",
        self.filename,
        self.spec,
        self.impl,
        self.proof)

  def is_spec(self):
    if self.filename.endswith(".s.dfy"): return True
    if "Protocol" in self.filename: return True
    if "Unsendable" in self.filename: return True
    return False

def parse_fileset(fileset_filenames):
  dafny_files = []
  def remove_prefix(l):
      # Andrea's txt files are relative to Ironclad/, but we want them down one directory.
      assert(l.startswith("ironfleet/"))
      return l[len("ironfleet/"):]
  for fileset_filename in fileset_filenames:    ##XXX
      dafny_files += [DafnyFile(remove_prefix(l)) for l in open(fileset_filename, "r").readlines()]
  return dafny_files

def run_dafny(dafny_executable, iron_base, show_ghost, dafny_filename, tmp_filename):
  args  = [] 
  args += ["/rprint:-"]
  args += ["/noAutoReq"]
  args += ["/noVerify"]
  #args += ["/nologo"]
  args += ["/env:0"]
  if show_ghost:
    args += ["/printMode:NoIncludes"]
  else:
    args += ["/printMode:NoGhost"]
  args += [dafny_filename]

  tmp_file = open(tmp_filename, "w")
  #print [dafny_executable] + args
  subprocess.call([dafny_executable] + args, shell=False, stdout=tmp_file,
                  cwd = iron_base)
  tmp_file.close()

# Remove detritus from running Dafny
def clean_dafny_output(filename):
  file = open(filename, "r")
  clean = ""
  for line in file.readlines():
    if line.startswith("Dafny program verifier finished"):
      pass
    else:
      clean += line + "\n"
  file.close()
  file = open(filename, "w")
  file.write(clean)
  file.close()

def run_sloccount(sloccount_executable, iron_base, tmp_dir):
  args  = [] 
  args += ["--details"]
  args += [tmp_dir]

  my_env = os.environ
  my_env["PATH"] = os.path.join(script_dir, "sloccount-2.26") + ":" + my_env["PATH"]
  my_env["LC_ALL"] = "C"

  sloc = -1
  #print(" ".join([sloccount_executable] + args))
  output = subprocess.check_output([sloccount_executable] + args, env=my_env) #, shell=True)
  print(f"jonh CMD is: {sloccount_executable} {args}")
  output = output.decode("utf-8")
  print("jonh DEBUG output is: "+output)
  for line in output.split('\n'):
    result = re.search("(\d+)\s+cs", line)
    if result:
      sloc = result.group(1)
  if sloc == -1:
    raise Exception("Failed to find sloccount result!")
    sloc = "1000000"
  return sloc

def compute_sloc(args, iron_base, show_ghost, dafny_file, tmp_dir):
  tmp_file = tmp_dir + "/tmp.dfy"

  run_dafny(args.dafny_executable, iron_base, show_ghost, dafny_file, tmp_file)
  clean_dafny_output(tmp_file)
  sloc = run_sloccount(args.sloccount_executable, iron_base, tmp_dir)
  os.remove(tmp_file)

  return int(sloc)

def collect_line_counts(args, iron_base, dafny_files):
  tmp_dir = iron_base + "/tmp/linecounts/"

  try: shutil.rmtree(tmp_dir)
  except FileNotFoundError: pass
  os.makedirs(tmp_dir)
  
  for f in dafny_files:
    print("Processing %s" % f.filename)
    path = os.path.join(iron_base, f.filename)
    if not os.path.exists(path):
        print("MISSING "+path)
        raise Exception("Missing file")
    ghost_sloc = compute_sloc(args, iron_base, True, f.filename, tmp_dir)

    if f.is_spec():
      f.spec = ghost_sloc
    else:
      impl_sloc = compute_sloc(args, iron_base, False, f.filename, tmp_dir)
      f.impl = impl_sloc
      f.proof = ghost_sloc - impl_sloc
    print("..."+repr(f))

def report_results_json(files, results_filename):
    total = DafnyFile("total")
    for f in files:
        total.spec += f.spec
        total.impl += f.impl
        total.proof += f.proof

    proof_code_ratio = "%.1f" % (total.proof/total.impl)

    message = {
        "dafny-baseline": {
            "linecount": {
                "trusted": total.spec,
                "proof": total.proof,
                "exec": total.impl,
                "proof-code-ratio": proof_code_ratio
            },
        }
    }
    json.dump(message, open(results_filename, "w"), indent=4)

def report_results_latex(files, latex_filename):
    with open(latex_filename, "w") as fp:
        total = DafnyFile("total")
        for f in files:
            fp.write(f"% {f}\n")
            total.spec += f.spec
            total.impl += f.impl
            total.proof += f.proof
        fp.write(f"% ------------\n")
        fp.write(f"% {total}\n")
        fp.write("\\newcommand{\\evalDafnyIronshtSpecLines}{"+str(total.spec)+"}\n")
        fp.write("\\newcommand{\\evalDafnyIronshtImplLines}{"+str(total.impl)+"}\n")
        fp.write("\\newcommand{\\evalDafnyIronshtProofLines}{"+str(total.proof)+"}\n")
        proof_code_ratio = "%.1f" % (total.proof/total.impl)
        fp.write("\\newcommand{\\evalDafnyIronshtProofCodeRatio}{"+proof_code_ratio+"}\n")

parser = argparse.ArgumentParser()
parser.add_argument("--dafny_executable", help="Location of Dafny", required=True)
parser.add_argument("--sloccount_executable", help="Location of dafny-patched sloccount", required=True)
parser.add_argument("--ironfleet_root", help="Location of checkout of IronClad Ironfleet repository", required=True)
parser.add_argument("--fileset", help='File listing dafny filenames to count', action="append", required=True)
parser.add_argument("--results", help='Where to write results', required=True)
parser.add_argument("--cache", help='Cache of sloccount output (for faster iteration on output transform)')
args = parser.parse_args()

def main():
  files = None
  if args.cache == None or not os.path.exists(args.cache):
    print(f"{args.fileset=}")
    files = parse_fileset(args.fileset)
    collect_line_counts(args, args.ironfleet_root, files)
    if args.cache:
        pickler = open(args.cache, "wb")
        pickle.dump(files, pickler)
        pickler.close()
  else:
    pickler = open(args.cache, "rb")
    files = pickle.load(pickler)
    pickler.close()
  report_results_json(files, args.results)
  #cats = categorize_files(files)
  #build_table(cats, latex_filename)

main()
