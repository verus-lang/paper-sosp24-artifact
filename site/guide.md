---
layout: md
title: "Verus: A Practical Foundation for Systems Verification<br/>Artifact Guide"
---

# Overview and technical requirements

There are three sets of experiments with different technical requirements.

### Set 1: verification statistics for macrobenchmarks and millibenchmarks, page table performance, mimalloc benchmark suite, persistent memory log performance (emulated) — Figures 6, 7, 8, 11, 12, 13.

Set 1 requires Linux x86_64 (Ubuntu 22.04) with at least 8 physical cores on one CPU, although more cores may reduce scheduling noise (we recommend at least 10). Set 1 requires the Docker runtime (Docker-CE). We recommend CloudLab d6515, or if they are in short supply, CloudLab c220g2.

### Set 2: IronKV performance comparison — Figure 9.

Set 2 requires a Windows x86_64 machine with .NET 5 or newer (we tested .NET 8.0), rust 1.76.0, and python 3 (we will provide installation instructions). A reasonably recent laptop or desktop should be sufficient.

### Set 3: node replication performance comparison — Figure 10.

Set 3 used a Linux Intel-based x86_64 4-CPU NUMA system with 24 cores per CPU. However, a smaller Linux Intel-based x86_64 NUMA system with at least 2 CPUs should reproduce a similar performance pattern. We recommend CloudLab r650.

# Experimental Sets

## Set 1

### Claims

**TODO.**

### Instructions

Start a Linux x86_64 machine, with at least 8 physical cores on one CPU, and Ubuntu 22.04. **We recommend CloudLab d6515.**

If you run on CloudLab, you can follow the instructions that follow. If you start a different machine or VM, the only requirement
to follow the same instructions is that `/mydata` is a directory on a mount with at least 50GB of free space.
Note that the commands and scripts in the following will manipulate the permissions of `/mydata`. The machine-level setup installs
Docker-CE and gives permission to the current user to connect to the container daemon. Other container runtimes compatible with the docker CLI should work too.

**Step 1. Clone artifact repository, set up container environment.**

Clone the repository

```shell
sudo chown $USER /mydata
cd /mydata
git clone -b main --single-branch https://github.com/verus-lang/paper-sosp24-artifact.git verus-sosp24-artifact
```

and run the script `setup/cloudlab-1.sh`


```shell
cd verus-sosp24-artifact
sudo bash setup/cloudlab-1.sh $USER
```

Log out and log in again to ensure the current user is part of the `docker` group.

**Step 2. Run the macrobenchmark verification statistics (Figure 8).**

**TODO** describe hand-tuned numbers and hard-coded baselines.

The automation scripts to produce the statistics in Figure 8 are in `macro-stats`.
The scripts make no changes to the system outside of the repository, other than spawning
containers.  `run.sh` will run all the necessary experiments.

```shell
cd /mydata/verus-sosp24-artifact/macro-stats
bash run.sh
```

This will produce output in the `results/` directory (`macro-stats/results`).
`results.json` are machine-readable results, which are also rendered as a pdf with the
same structure as the figure in the paper, `results/macro-table.pdf`.

From the local machine, copy the results off the cloudlab instance. On Linux you can use something like the following:

```shell
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/macro-stats/results/results.json' .
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/macro-stats/results/macro-table.pdf' .
```

**Step 3. Run the mimalloc benchmark suite**

Navigate to the directory **(TODO where?)**

Run:

```
./build-benchmarks-and-allocators.sh
./compare-benchmarks.sh
```

This should only take a couple of minutes.

Note many benchmarks are expected to fail, and you'll probably see indications of it
in the intermediate output. The end will summarize the results in tabular form.
The last table, formatted in LaTeX, only contains the benchmarks that succeeded.
The output should resemble Figure 12.

## Set 2

### Claims

Our claim is that our Verus version of IronKV has similar performance to the
Dafny version of IronKV (also called IronSHT in some contexts) from IronFleet.

The methodology is that we benchmark the Dafny and Verus versions of IronKV
using the test harness from IronFleet's repository. The experiments for the
paper reflect a run on Windows 11 Enterprise on a 2.4 GHz Intel Core i9-10885H
CPU 8-core laptop with 64 GB of RAM. The resulting figure (Figure 7 in the
paper) confirms our claim.

These instructions describe how to generate your own version of Figure 7 on
the Windows machine of your choice. The figure you generate will be in LaTeX
format, in a file named `ironfleet-port-plot.tex`. Unless you use exactly the
same type of machine we use, your results may be quantitatively different from
ours. For example, you may find higher or lower absolute throughput. But your
results should still (hopefully) confirm our claim of similar performance.

### Instructions

To run this experiment, take the following steps:

* Build the IronFleet version of IronKV.
    * Install `dotnet`.
    * Install `scons` with `pip install scons`.
    * Download the Dafny 3.4.0 release, including its executable, from
      `https://github.com/dafny-lang/dafny/releases/download/v3.4.0/dafny-3.4.0-x64-win.zip`.
    * Sync to commit `2fe4dcdc323b92e93f759cc3e373521366b7f691` of the
      Ironclad repository at `https://github.com/microsoft/Ironclad.git`.
    * From the `ironfleet` directory in that repository, run
      `scons --dafny-path=<path>` where `<path>` is the path to the directory
      containing the Dafny 3.4.0 executable.
* Build the Verus version of IronKV.
    * Download the Verus source code from commit
      `96957b633471e4d5a6bc267f9bf0e31555e888db`
      of the repo at `https://github.com/verus-lang/verus`.
    * Build the Verus source code as the repo describes, making sure to use
      `--release` on the `vargo build`.
    * Download the Verus version of IronKV from commit
      `ea501b56ef92290329ba434fb8b675a5f467de65` of the
      repository at `https://github.com/verus-lang/verified-ironkv.git`.
    * Make a small local update to that repository's code to make it operate on
      Windows, as follows:  In the file
      `ironsht/csharp/IronSHTClient/Client.cs`, change
      all references to `../liblib.so` to `lib.dll`.
    * From the top-level directory of that repository, run
      `scons --verus-path=<path>` where `<path>` is the path to the root
      directory for the Verus repository. This will create a `lib.dll`
      file in the top-level directory of that repository.
    * Copy that `lib.dll` file to the `ironkv` subdirectory of the repository
      for this artifact. For instance, if this file you're reading now is
      `<path>/site/guide.md`, copy it to `<path>/ironkv/lib.dll`.
* Prepare to run the experiment.
    * Change directory to the `ironkv` subdirectory of the repository for
      this artifact. For instance, if this file you're reading now is
      `<path>/site/guide.md`, change directory to `<path>/ironkv/`.
    * Generate certificates by running
      `dotnet <path>/ironsht/bin/CreateIronServiceCerts.dll
     outputdir=certs name=MySHT type=IronSHT addr1=127.0.0.1 port1=4001
     addr2=127.0.0.1 port2=4002 addr3=127.0.0.1 port3=4003`
      where `<path>` is the path to either the Dafny or Verus IronKV code.
    * With `pip`, install `numpy` and `scipy`.
    * Update the hardcoded paths `VERUS_PATH` and `DAFNY_PATH` in the script
      `compare.py` to match where those directories are on your machine.
    * Prepare your machine for the experiment by telling Windows to never
      sleep, by telling it to use the "best performance" power mode (to
      disable SpeedStep), and by plugging it into a real charging outlet (not
      just a USB-C connector to a monitor).
* Run the experiment by running `python compare.py` from the `ironkv`
  subdirectory of the repository for this artifact. This will overwrite the
  file `raw-data.txt` with its output.
* Generate the graph by running `python gengraph.py > ironfleet-port-plot.tex`.
  This uses the data stored in `raw-data.txt` to generate a graph, in LaTeX
  format, in the file `ironfleet-port-plot.tex`.


## Set 3 - Node Replication

### Claims

The three node-replication implementations (unverified Rust, IronSync and Verus) have similar
performance and scaling behavior (throughput).

### Instructions

Start a Linux x86_64 machine, with at least 2 NUMA nodes, and Ubuntu 22.04. **We recommend CloudLab r650.**

If you run on CloudLab, you can follow the instructions that follow. If you start a different machine or VM, the only requirement
to follow the same instructions is that `/mydata` is a directory on a mount with at least 50GB of free space.
Note that the commands and scripts in the following will manipulate the permissions of `/mydata`.

#### 1. Installing Dependencies

The following instructions will install all dependencies required to build and run the benchmarks.

```shell
sudo apt-get update
sudo apt-get install -y curl wget liburcu-dev libhwloc-dev python3-venv texlive-xetex texlive-fonts-extra pkg-config clang make g++
```

Linear Dafny requires a specific version of libssl. You can install this with the following command:

```shell
cd /mydata
wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb
sudo dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb
rm -rf libssl1.1_1.1.0g-2ubuntu4_amd64.deb

```

Install Rust using rustup toolchain installer:

```shell
(curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" \
  | sh -s -- --default-toolchain none -y)
# source the Rust environment
. "$HOME/.cargo/env"
```

#### 2. Repository Setup

Clone the Verified Node Replication repository and checkout commit `a1cc5ceabff60d33b5809a988a325df8a15cda14`

```shell
cd /mydata
git clone https://github.com/verus-lang/verified-node-replication.git
cd verified-node-replication
git checkout a1cc5ceabff60d33b5809a988a325df8a15cda14
```

Initialize the submodules. This should initialize three submodules:
   - `verus`
   - `benchmarks/ironsync/ironsync-osdi2023`
   - `benchmarks/lib/node-replication`

```shell
# inside verified-node-replication repo
git submodule update --init
```

The repository should now be ready and we can build the binaries and run the benchmark.

### 3. Running the Benchmark

To run the benchmarks, navigate into the `benchmarks` directory and execute the `run_benchmarks.sh`
script:

```shell
# inside verified-node-replication repo
cd benchmarks
bash run_benchmarks.sh
```

The script will run three throughput scaling benchmarks. The more cores your system has, the longer
it will take. You should see intermediate prints on the terminal indicating progress.

The script does the following:
 1. Setup a Python environment for the `bench.py` script,
 2. Build and run the Verus NR benchmark, and
 3. Build and Run the IrondSync benchmark used for the unverified Rust implementation and the IronSync
implementation.


Note, that the benchmarks will change the following CPU settings:
 1. Disable DVFS
 2. Disable Turbo Boost
 3. Set the CPU governor to `performance`


Also note, that this will pull in the dependencies for building Linear Dafny automatically.


### 4. Obtaining the Results

You can view the results of the benchmark by opening the automatically generated plots:

```shell
open nr-results-throughput-vs-cores-numa-fill.pdf
```
(There is also a PNG version)

Each of the three subplots should have three lines (for Verus NR, IronSync NR and Upstream NR) that
are similar in performance and scaling behavior.
