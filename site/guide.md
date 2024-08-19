---
layout: md
title: "Verus: A Practical Foundation for Systems Verification<br/>Artifact Guide"
---

# Overview and technical requirements

There are three sets of experiments with different technical requirements.

### Set 1: verification statistics for macrobenchmarks and millibenchmarks, page table performance, mimalloc performance comparison, persistent memory log performance (emulated) — Figures 6, 7, 8, 11, 12, 13.

Set 1 requires Linux x86_64 (Ubuntu 22.04) with at least 8 physical cores on one CPU, although more cores may reduce scheduling noise (we recommend at least 10). Set 1 requires the Docker runtime (Docker-CE). We recommend CloudLab d6515, or if they are in short supply, CloudLab c220g2.

### Set 2: IronKV performance comparison — Figure 9.

Set 2 requires a Windows x86_64 machine with .NET 5 or newer (we tested .NET 8.0), rust 1.76.0, and python 3 (we will provide installation instructions). A reasonably recent laptop or desktop should be sufficient.

To run this experiment, take the following steps:

* Build the IronFleet version of IronSHT.
    * Install `dotnet`.
    * Install `scons` with `pip install scons`.
    * Download the Dafny 3.4.0 release, including its executable, from
      `https://github.com/dafny-lang/dafny/releases/download/v3.4.0/dafny-3.4.0-x64-win.zip`.
    * Sync to commit `2fe4dcdc323b92e93f759cc3e373521366b7f691` of the Ironclad repository at `git@github.com:microsoft/Ironclad.git`.
	* From the `ironfleet` directory in that repository, run `scons
      --dafny-path=<path>` where `<path>` is the path to the directory
      containing the Dafny 3.4.0 executable.
* Build the Verus version of IronSHT.
    * Download the Verus source code from commit `96957b633471e4d5a6bc267f9bf0e31555e888db`
      of the repo at `git@github.com:secure-foundations/verus.git`.
    * Build the Verus source code as the repo describes, making sure to use
      `--release` on the `vargo build`.
    * Download the Verus version of IronSHT from commit
      `ea501b56ef92290329ba434fb8b675a5f467de65` of the Verus systems code
      repository at `git@github.com:verus-lang/verus-systems-code.git`.
    * Make a small local update to that repository to make it operate on
      Windows, as follows:  In the file
      `ironfleet-comparison/ironsht/csharp/IronSHTClient/Client.cs`, change
      all references to `../liblib.so` to `lib.dll`.
    * From the `ironfleet-comparison` directory, run `scons
         --verus-path=<path>` where `<path>` is the path to the root directory
      for the Verus repository.
    * Copy the resulting `lib.dll` from the `ironfleet-comparison` directory
      to the directory containing this `README.md` (`sys/eval/ironsht`).
* Prepare to run the experiment.
    * Change directory to the directory containing this `README.md`
      (`sys/eval/ironsht`).
    * Generate certificates with `dotnet <path>/ironsht/bin/CreateIronServiceCerts.dll
      outputdir=certs name=MySHT type=IronSHT addr1=127.0.0.1 port1=4001
      addr2=127.0.0.1 port2=4002 addr3=127.0.0.1 port3=4003` where `<path>` is
      the path to either the Dafny or Verus IronSHT code.
    * With `pip`, install `numpy` and `scipy`.
    * Update the hardcoded paths `VERUS_PATH` and `DAFNY_PATH` in the script
      `compare.py` to match where those directories are on your machine.
    * Prepare your machine for the experiment by telling Windows to never
      sleep, by telling it to use the "best performance" power mode (to
      disable SpeedStep), and by plugging it into a real charging outlet (not
      just a USB-C connector to a monitor).
* Run the experiment by running `python compare.py` from this directory. This will
  overwrite the file `raw-data.txt` with its output.
* Generate the graph by running `python gengraph.py > ..\..\paper\ironfleet-port-plot.tex`.
  This uses the data stored in `raw-data.txt`.

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

```sh
sudo chown $USER /mydata
cd /mydata
git clone -b main --single-branch https://github.com/verus-lang/paper-sosp24-artifact.git verus-sosp24-artifact
```

and run the script `setup/cloudlab-1.sh`


```sh
cd verus-sosp24-artifact
sudo bash setup/cloudlab-1.sh $USER
```

Log out and log in again to ensure the current user is part of the `docker` group.

**Step 2. Run the macrobenchmark verification statistics (Figure 8).**

**TODO** describe hand-tuned numbers and hard-coded baselines.

The automation scripts to produce the statistics in Figure 8 are in `macro-stats`.
The scripts make no changes to the system outside of the repository, other than spawning
containers.  `run.sh` will run all the necessary experiments.

```sh
cd /mydata/verus-sosp24-artifact/macro-stats
bash run.sh
```

This will produce output in the `results/` directory (`macro-stats/results`).
`results.json` are machine-readable results, which are also rendered as a pdf with the
same structure as the figure in the paper, `results/macro-table.pdf`.

From the local machine, copy the results off the cloudlab instance. On Linux you can use something like the following:

```sh
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/macro-stats/results/results.json' .
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/macro-stats/results/macro-table.pdf' .
```

