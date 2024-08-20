import os
import re
import subprocess
import sys
import time

VERUS_PATH = "C:/Apps/verus-systems-code/ironfleet-comparison/ironsht/bin/"
DAFNY_PATH = "C:/Apps/ironclad-microsoft/ironfleet/bin/"
RAW_DATA_PATH = "raw-data.txt"
NUM_THREADS = 10
SECONDS = 30
NUM_KEYS = 1000
NUM_EXPERIMENT_ITERATIONS = 100
CONFIDENCE_QUANTILE = 0.95
VALUE_SIZES = [ 128, 256, 512 ]

raw_data_out = open(RAW_DATA_PATH, "w")
raw_data_out.write("Language\tThreads\tSeconds\tWorkload\tNumber of keys\tValue size\tRequests completed\n")

def launch_server(verus, which_server):
    server_exe = os.path.join(VERUS_PATH if verus else DAFNY_PATH, "IronSHTServer.dll")
    cmd = [
              "dotnet",
              server_exe,
              "certs/MySHT.IronSHT.service.txt",
              f"certs/MySHT.IronSHT.server{which_server}.private.txt"
          ]
    server = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    while True:
        line = server.stdout.readline().decode('utf-8').strip()
        if line == "[[READY]]":
            return server

def measure_client(verus, workload, value_size):
    server1 = launch_server(verus, 1)
    server2 = launch_server(verus, 2)
    server3 = launch_server(verus, 3)

    client_exe = os.path.join(VERUS_PATH if verus else DAFNY_PATH, "IronSHTClient.dll")
    cmd = [
              "dotnet",
              client_exe,
              "certs/MySHT.IronSHT.service.txt",
              f"nthreads={NUM_THREADS}",
              f"duration={SECONDS}",
              f"workload={workload}",
              f"numkeys={NUM_KEYS}",
              f"valuesize={value_size}"
          ]
    client = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    num_requests_completed = 0
    while True:
        line = client.stdout.readline()
        if not line:
            break
        line = line.decode('utf-8').strip()
        if re.search('^#req', line):
            num_requests_completed = num_requests_completed + 1
    kilo_requests_per_second = num_requests_completed * 0.001 / SECONDS
    raw_data_out.write("%s\t%s\t%s\t%s\t%s\t%s\t%s\n" % (
                           "verus" if verus else "dafny",
                           NUM_THREADS,
                           SECONDS,
                           workload,
                           NUM_KEYS,
                           value_size,
                           num_requests_completed
                      ))
    raw_data_out.flush()

    server1.kill()
    server2.kill()
    server3.kill()

    return kilo_requests_per_second

def do_experiments():
    for i in range(NUM_EXPERIMENT_ITERATIONS):
        for verus in [True, False]:
             language = ("Verus" if verus else "Dafny")
             for workload in [ 'g', 's' ]:
                 for value_size in VALUE_SIZES:
                      print(f"Performing experiment #{i} for {language} with workload {workload} and value size {value_size}")
                      kops = measure_client(verus, workload, value_size)

do_experiments()

raw_data_out.close()
