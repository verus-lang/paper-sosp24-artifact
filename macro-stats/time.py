import sys
import subprocess
import time

output_file = sys.argv[1]
time_file = sys.argv[2]
command = sys.argv[3:]

start_time = time.time()

with open(output_file, 'w') as f:
    process = subprocess.run(command, stdout=f)

end_time = time.time()
execution_time = end_time - start_time

with open(time_file, 'w') as f:
    f.write(str(execution_time))
