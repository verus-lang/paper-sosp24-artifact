---
layout: md
title: "Verus: A Practical Foundation for Systems Verification<br/>Artifact Guide"
---

**This file is rendered at https://verus-lang.github.io/paper-sosp24-artifact/guide.html, we recommend reading it there.** This page's source is at [https://github.com/verus-lang/paper-sosp24-artifact/blob/main/site/guide.md](https://github.com/verus-lang/paper-sosp24-artifact/blob/main/site/guide.md).

The paper draft is at [https://verus-lang.github.io/paper-sosp24-artifact/assets/paper-20240821-212701-e33099e.pdf](https://verus-lang.github.io/paper-sosp24-artifact/assets/paper-20240821-212701-e33099e.pdf). For artifact evaluators, a list of the changes from the accepted version is at [https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ae/paper-draft-changes.md](https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ae/paper-draft-changes.md).

**This artifact references external repositories with open-source versions of Verus and the use cases presented. The artifact uses fixed commits (or "refspecs" / SHAs) which are also listed here: [https://github.com/verus-lang/paper-sosp24-artifact/blob/main/external-repository-versions.md](https://github.com/verus-lang/paper-sosp24-artifact/blob/main/external-repository-versions.md).**

# Overview and technical requirements

There are three sets of experiments with different technical requirements.

### Set 1: verification statistics for macrobenchmarks and millibenchmarks, page table performance, mimalloc benchmark suite, persistent memory log performance (emulated) — Figures 6, 7, 8, 11, 12, 13.

Set 1 requires Linux x86_64 (Ubuntu 22.04) with at least 8 physical cores on one CPU, although more cores may reduce scheduling noise (we recommend 10). Set 1 requires the Docker runtime (Docker-CE). We recommend CloudLab d6515, or if they are in short supply, CloudLab c220g2.

### Set 2: IronKV performance comparison — Figure 9.

Set 2 requires a Windows x86_64 machine with .NET 5 or newer (we tested .NET 8.0), rust 1.76.0, and python 3. A reasonably recent laptop or desktop should be sufficient.

### Set 3: node replication performance comparison — Figure 10.

Set 3 used a Linux Intel-based x86_64 4-CPU NUMA system with 24 cores per CPU. However, a smaller Linux Intel-based x86_64 NUMA system with at least 2 CPUs should reproduce a similar performance pattern. We recommend CloudLab r650.

# Experimental Sets

## Set 1

### Claims

This experimental set corresponds to the results in Figures 6, 7, 8, 11, 12, and 13. The instructions steps will
refer back to the claims listed here.

**Claim A**. In small-scale benchmarks,
Verus verification times compare favorably to other verification tools used to verify complex
properties of large-scale systems, and which offer a large degree of automation "out of the box".
This favorable comparison includes successful verification of data structures (Figure 6a), verification of
program code when the amount of required memory reasoning increases (Figure 6b), and time to report verification
failure (Figure 7).

**Claim B**. Verus can efficiently verify a wide range of systems projects. Compared to Dafny and Linear Dafny,
Verus verifies code more quickly, with a lower proof-to-code ratio (indicating developer burden),
and produces smaller SMT queries.
The evaluation compares Verus with Dafny on the implementation of IronKV, Verus with Linear Dafny on the proof for
Node Replication, and on three new systems: a page table implementation, a concurrent memory allocator, and a persistent
memory log (all in Figure 8).

**Claim C**. The performance of the verified page table implementation is comparable to the corresponding unverified
implementation (except for eager directory reclamation, which the unverified implementation does not do). (Figure 11)

**Claim D**. The prototype verified memory allocator can complete 8 out of 19 benchmarks from mimalloc’s benchmark suite, though it does not reach performance parity. (Figure 12)

**Claim E**. The initial version of the verified persistent memory log provided low throughput on small appends due to its extra copying; the latest version eliminates this overhead and achieves comparable throughput to the baseline, libpmemlog. (Figure 13)

### Note: Millibenchmark programs -- Reusability

The programs used for Millibenchmarks are in the `milli/` directory. Some are adapted from
existing open source examples or projects, typically from one of the other verifiers' examples.

If you would like to reuse these programs for further evaluation or to improve tooling, refer
to the files in `linked-list` and `doubly-linked-list` with the name corresponding to the tool. These are the programs used for Figure 6a.

Programs for the memory reasoning benchmark (Figure 6b) are generated with the `repeat*py` scripts, while programs for evaluating verification time of finding errors (Figure 7) are adapted from the linked list programs by removing necessary preconditions: these are in `linked-list/errors`.

At the time of paper submission Creusot required interactive sessions in the Why3 prover environment. These have been stored in `creusot-*` directories in `linked-list`, `linked-list/errors`, and `doubly-linked-list`.

### Instructions

Start a Linux x86_64 machine, with at least 8 physical cores on one CPU, and Ubuntu 22.04. **We recommend CloudLab d6515.**

If you run on CloudLab, you can follow the instructions below. If you start a different machine or VM, the only requirement
to follow the same instructions is that `/mydata` is a directory.
Note that the commands and scripts in the following will manipulate the permissions of `/mydata`. The machine-level setup (the `setup/cloudlab-1.sh` script) installs
Docker-CE and gives permission to the current user to connect to the container daemon. Other container runtimes compatible with the docker CLI should work too.

On CloudLab, use the default small-lan profile, and change parameters to select Ubuntu 22.04,
the correct node type (e.g. `d6515`) and (expand "Advanced") enable "Temp Filesystem Max Space"
(for the `/mydata` mount).

If you run on CloudLab, ssh into the node. We recommend running the following in `tmux` (or similar)
so that the experiment can continue if the ssh connection drops. In that case, you can reattach to the tmux session
by ssh-ing into the node, and running `tmux attach`.

If you do not run on Cloudlab, at the end of the experiment you may want to clean up the the following container images,
that are pulled as part of the following steps in this experimental set.

```
ubuntu                                        22.04          77.9MB
kjarosh/latex                                 2024.2-small   400MB
ghcr.io/utaal/ubuntu-essentials-rust-1.76.0   latest         2.97GB
ghcr.io/utaal/ironsync-osdi2023-artifact      latest         2.53GB
```

You can remove them at the end of the Set 1 with `docker rmi <image_name>`.

#### 1. Clone artifact repository, and set up container environment.

Clone the repository

```shell
sudo chown $USER /mydata
cd /mydata
git clone -b main --single-branch https://github.com/verus-lang/paper-sosp24-artifact.git verus-sosp24-artifact
cd verus-sosp24-artifact
```

Optionally, if you would like to obtain the exact version of this artifact at submission time,
check out the exact commit "refspec" (SHA) as indicated on the Artifact Evaluation review site
(replace `<sha>` with the commit refspec/SHA):

```shell
git checkout <sha>
```

and run the script `setup/cloudlab-1.sh`


```shell
sudo bash setup/cloudlab-1.sh $USER
```

This will install Docker-CE and disable Simultaneous Multithreading (SMT, also known as Hyperthreading).
To re-enable after the experiments (if necessary) you can reboot, or use `sudo bash -c "echo off > /sys/devices/system/cpu/smt/control"`.

Log out and log in again to ensure the current user is part of the `docker` group.

#### 2. Run the millibenchmark verification statistics (Figures 6, 7).

*This step refers to Set 1 - Claim A.*

*Running this step will take roughly 1.5 hours.*

The automation scripts to produce the statistics in Figures 6 and 7 are in the `milli` directory.
The scripts make no changes to the system outside of the repository, other than spawning
containers. `run.sh` will run all the necessary experiments.

The paper uses the median of 20 samples for each datapoint, but that takes a long time.
We recommend using 4 samples (they should be quite repeatable). You can change the number of
samples by changing the `4` to the number of desired samples in the following code.

```shell
cd /mydata/verus-sosp24-artifact/milli
bash run.sh 4
```

When the experiments complete, stop and delete the container:

```shell
docker rm -f verus-sosp24-milli
```

This will produce results in the `/mydata/verus-sosp24-artifact/milli/results` directory,
including plots.

For all the following results, the absolute values may be different, but the relative performance
should be roughly similar to the results in the paper.

First, inspect the verification times for the singly linked list and the doubly linked list as follows.
Still in the `/mydata/verus-sosp24-artifact/milli` directory, run:

```shell
cat results/linked-list-oneshot.txt
```

to see the results which correspond to the "Single" column of Figure 6a.

Then run:

```shell
cat results/doubly-linked-list-oneshot.txt
```

to see the results which correspond to the "Double" column of Figure 6a.

On your local machine, copy the plots from the CloudLab instance. You can use something like the following:

```shell
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/milli/results/linked-list-memory-reasoning.pdf' .
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/milli/results/doubly-linked-list-memory-reasoning.pdf' .
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/milli/results/error-times-1.pdf' .
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/milli/results/error-times-2.pdf' .
```

The `linked-list-memory-reasoning.pdf` plot corresponds to Figure 6b, and `doubly-linked-list-memory-reasoning.pdf` confirms that the doubly-linked list follows a similar pattern.

The `error-times-1.pdf` and `error-times-2.pdf` plots correspond to Figure 7.

Again on the CloudLab machine, complete this step by cleaning up the Why 3 sessions that are modified when replaying Creusot proofs, as follows.

```shell
cd /mydata/verus-sosp24-artifact/milli
git checkout -- linked-list doubly-linked-list
```

#### 3. Run the macrobenchmark verification statistics (Figure 8).

*This step refers to Set 1 - Claim B.*

*Running this step will take roughly half an hour.*

The automation scripts to produce the statistics in Figure 8 are in the `macro-stats` directory.
The scripts make no changes to the system outside of the repository, other than spawning
containers. `run.sh` will run all the necessary experiments.

```shell
cd /mydata/verus-sosp24-artifact/macro-stats
bash run.sh
```

This will produce output in the `results/` directory (`macro-stats/results`).
`results.json` are machine-readable results, which are also rendered as a pdf with the
same structure as the figure in the paper, `results/macro-table.pdf`.

On your local machine, copy the results from the CloudLab instance. You can use something like the following:

```shell
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/macro-stats/results/results.json' .
scp '<username>@<node>.cloudlab.us:/mydata/verus-sosp24-artifact/macro-stats/results/macro-table.pdf' .
```

The table should closely match Figure 8. Small discrepancies are because the
artifact uses a version of Verus at the time of artifact evaluation and uses published
versions of the case studies, that have received minimal changes to clean them up and to
keep them compatible with the current version of Verus.

All the values in the table are obtained by the script except for the values in
`macro-stats/summarize/manual.json`:

* the size of the Dafny SMT encoding for IronKV was computed with a series of manual steps that
  we were not able to repackage in the artifact;
* the line counts for the Linear Dafny version of Node Replication are from the corresponding paper:
  [Sharding the State Machine: Automated Modular Reasoning for Complex Concurrent Systems](https://www.usenix.org/system/files/osdi23-hance.pdf)
* the page table line counts are adjusted by hand-picked deltas (<100 lines) due to the line
  counting tool misattributing certain lines.

#### 3. Build a copy of Verus for the performance evaluation.

This will clone a copy of Verus to use for the macrobenchmark performance experiments in this experimental set.
The scripts make no changes to the system outside of the repository other than spawning
containers.

```shell
cd /mydata/verus-sosp24-artifact
bash setup/perf-build-verus.sh
```

#### 4. Run the page table benchmark (Figure 11).

*This step refers to Set 1 - Claim C.*

*Running this step will take a few minutes.*

Clone the verified-nrkernel repository:

```shell
cd /mydata
git clone https://github.com/utaal/verified-nrkernel
cd verified-nrkernel; git checkout a861a090d79f092d6aa5e6d30927327b70504c2d
```

Start a Ubuntu 22.04 container with Rust using the pre-made image, and run the experiments
using the following commands.
The scripts make no changes to the system outside of the repository, other than spawning
containers. `entry-page-table.sh` will run all the necessary experiments.

```shell
cd /mydata
docker run --platform=linux/amd64 -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash verus-sosp24-artifact/macro-perf/entry-page-table.sh
```

This will output something like the following:

```
Time Verified Map: 23.08858679 ns
Time Verified Unmap: 303.26841955 ns
Time Verified Unmap (no reclaim): 7.66253143 ns
Time Base Map: 34.33901477 ns
Time Base Unmap: 8.37488886 ns
```

where each line corresponds to one column in Figure 11 (in a different order).

Note that "Time Verified Unmap" is expected to be much higher than "Time Base Unmap" as there
is additional work done to 1) check whether the directory is empty, and 2) reclaim the directory memory.
This is not done by the unverified implementation.


#### 5. Run the mimalloc benchmark suite (Figure 12).

*This step refers to Set 1 - Claim D.*

*Running this step will take a few minutes.*

Clone the verified-memory-allocator repository:

```shell
cd /mydata
git clone https://github.com/verus-lang/verified-memory-allocator.git
cd verified-memory-allocator; git checkout 6ee4b4fc8ac107f10d3ad420a2c42e26e3033ba7
```

Start a Ubuntu 22.04 container with Rust using the pre-made image, and run the experiments
using the following commands.
The scripts make no changes to the system outside of the repository, other than spawning
containers. `entry-mimalloc.sh` will run all the necessary experiments.

```shell
cd /mydata
docker run --platform=linux/amd64 -it -v .:/root/eval -w /root/eval ghcr.io/utaal/ubuntu-essentials-rust-1.76.0 /bin/bash verus-sosp24-artifact/macro-perf/entry-mimalloc.sh
```

This should only take a couple of minutes.

Note many benchmarks are expected to fail, and you'll probably see indications of it
in the intermediate output. The end will summarize the results in tabular form.
The last table, formatted in LaTeX, only contains the benchmarks that succeeded.
The output should resemble Figure 12.

#### 6. Run the persistent memory log experiment (Figure 13).

*This step refers to Set 1 - Claim E.*

*Running this step will take about half an hour.*

Run: 

```shell
cd /mydata/verus-sosp24-artifact
./setup/pm_vm_setup.sh
sudo ./setup/pm_vm_boot.sh
```

To create and boot a VM to run the PM experiments in. The username and password are both 
set to `ubuntu`.

You may be prompted to select an OS when booting the VM; if so, hit Enter to select Ubuntu.
The VM will take a minute or so to boot. 
Once the boot has completed, the terminal will show a login prompt; leave this prompt alone and
open a new terminal (on CloudLab, ssh into the CloudLab machine again, from a new terminal).
In the new terminal, run `ssh ubuntu@localhost -p 2222` to SSH into the VM. Enter password `ubuntu`.
*All subsequent steps for this experiment will be run in the VM*.

In the VM, run the following commands to install dependencies, clone the experiment repo, and set up emulated persistent memory.

```shell
sudo apt update
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.76.0 -y
sudo apt install -y linux-generic llvm-dev libclang-dev clang libpmem1 libpmemlog1 libpmem-dev libpmemlog-dev build-essential python3-pip
pip3 install matplotlib scipy
git clone -b generic_trait_serialization --single-branch https://github.com/microsoft/verified-storage.git
sudo sed -i 's/GRUB_CMDLINE_LINUX=""/GRUB_CMDLINE_LINUX="memmap=8G!4G"/' /etc/default/grub
sudo update-grub
```

Reboot the VM (`sudo reboot -h now`) and SSH in again (`ssh ubuntu@localhost -p 2222`).
There should now be a file `/dev/pmem0` on the VM; this is the emulated persistent memory.

Run the following commands in the VM to start the experiment:

```shell
cd verified-storage/artifact_eval/experiment/verif-storage-eval
cargo build --release
sudo mkdir /mnt/pmem
cd ../
./run.sh /dev/pmem0 /mnt/pmem results
```
The experiment takes 20-30 minutes. The results will be placed in `verified-storage/artifact_eval/experiment/results`.

Once the experiment finishes, run:

```shell
python3 plot_verif_comparison.py results 8192
```

(8192 is the total MiB appended to the log in each iteration of the experiment).
This command will produce a PDF at `verified-storage/artifact_eval/experiment/results.pdf` with a
graph resembling Figure 13. 

You can copy the pdf to the host with (enter `ubuntu` when prompted for a password).
Close the ssh session with the VM and run this on the host, not inside the VM:

```
cd /mydata
scp -P 2222 ubuntu@localhost:/home/ubuntu/verified-storage/artifact_eval/experiment/results.pdf verified-storage-results.pdf
```

Then shut down the VM:

```
ssh ubuntu@localhost -p 2222
sudo shutdown -h now
```

From the local machine, copy the results off the CloudLab instance. On Linux you can use something like the following:

```shell
scp '<username>@<node>.cloudlab.us:/mydata/verified-storage-results.pdf' .
```

*These results refer to Set 1 - Claim E:*

We expect the general pattern in the graph generated from these instructions to remain the same as 
that in the graph in the paper: PMDK and the latest verified version have similar throughput on all 
workloads, whereas the initial verified log has lower throughput due to its higher serialization
overhead. However, we do expect some noticeable differences, as the experimental data in the paper 
was obtained on Intel Optane PM and these instructions use emulated PM on regular DRAM. The results 
obtained by following these instructions are expected to differ from the paper results in the 
following ways.

1. The overall throughput for all three logs will be higher in the DRAM results. 
Optane PM has lower bandwidth and higher latency than DRAM. The actual values 
depend on the machine the experiment is run on; for example, on a c220g2 CloudLab 
instance, we observe a maximum throughput of approximately 3000 MiB/s for PMDK and 
the current verified log, and approximately 2300 MiB/s for the original verified log, 
and we would expect higher throughput on a more powerful machine.
1. The initial verified log will have comparatively worse performance even as append 
sizes increase on DRAM. In the paper results, all three logs obtain similar performance 
on append sizes 64KiB and up, but we expect the initial log to consistently achieve lower 
throughput on all append sizes when run on DRAM. This is because the initial log has 
higher software overhead than the other two logs due to its non-optimal serialization approach 
that performs extra in-DRAM copying, which is dominated by the higher latency on Optane PM but 
has a bigger impact on performance when run on DRAM.
1. We expect larger error bars on the graph generated from these instructions than the one in 
the paper, as the results in the paper were obtained from experiments run on baremetal, whereas 
these instructions obtain results on VM.
1. On PM, the highest throughputs are obtained on append sizes of 4KiB and 8KiB, with larger 
append sizes plateauing a bit lower; on DRAM, we expect the highest throughputs to be obtained 
on append sizes 64KiB, 128KiB, and 256KiB. We attribute this to differences in maximum write 
bandwidth of the different hardware.


## Set 2

### Claims

Our claim is that our Verus version of IronKV has similar performance to the
Dafny version of IronKV (also called IronSHT in some contexts) from IronFleet.

The methodology is that we benchmark the Dafny and Verus versions of IronKV
using the test harness from IronFleet's repository. The experiments for the
paper reflect a run on Windows 11 Enterprise on a 2.4 GHz Intel Core i9-10885H
CPU 8-core laptop with 64 GB of RAM. The resulting figure (Figure 9 in the
paper) confirms our claim.

These instructions describe how to generate your own version of Figure 9 on
the Windows machine of your choice. The figure you generate will be in LaTeX
format, in a file named `ironfleet-port-plot.tex`. The generator script also
outputs data in textual form in the terminal so it can be more readily inspected
without the need to render the latex plot.

Unless you use exactly the
same type of machine we use, your results may be quantitatively different from
ours. For example, you may find higher or lower absolute throughput. But your
results should still (hopefully) confirm our claim of similar performance.

### Screen Capture

Due to potential difficulties in accessing a suitable Windows machine, we also
provide a screen capture of the experiment performed on an Amazon AWS c5.4xlarge
machine running Windows: [https://zenodo.org/records/13689054/files/Verus-Artifact-Set-2.mp4?download=1](https://zenodo.org/records/13689054/files/Verus-Artifact-Set-2.mp4?download=1). The first
21 minutes contain the set up, followed by the command `python compare.py $VERUS_AE`
which starts the experiment, lasting till 1h23. The last few minutes summarize the
results with `gengraph.py` and print the MD5 hash of the `raw-data.txt` output from
the experiment and the generated `ironfleet-port-plot.tex` files.

The files generated in this run are available in this repository:

* [https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ironkv/recording/raw-data.txt](https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ironkv/recording/raw-data.txt) (md5 hash: `0ce875c99ed5763c103250528f981c53`) and
* [https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ironkv/recording/ironfleet-port-plot.tex](https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ironkv/recording/ironfleet-port-plot.tex) (md5 hash: `ac9734133d8e076b822e9c76423327c0`)

alongside a compiled version of the LaTeX file as PDF: [https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ironkv/recording/ironfleet-port-plot.pdf](https://github.com/verus-lang/paper-sosp24-artifact/blob/main/ironkv/recording/ironfleet-port-plot.pdf)

The experiment in the screen capture was performed by following exactly the instructions below.

### Instructions

Set 2 requires a Windows x86_64 machine with .NET 6.0, Rust 1.76.0, Python 3,
and Git.
 A reasonably recent laptop or desktop should be sufficient.
 For
commands that require a terminal, note that you can open a terminal by
opening the start menu, typing `powershell`, then pressing the enter key.

#### Setup

Below you'll find instructions for installing various dependencies. If you
already have a dependency installed, feel free to skip its instructions. But
you may need to reconfigure or reinstall Python 3 or Git to make sure that
it's available in the PATH (i.e., that it can be run from a terminal).

(Note for artifact evaluators who requested a Windows machine from us: We've
already installed all these dependencies for you.)

**.NET 6.0**. To install .NET 6.0, go to [https://dotnet.microsoft.com/en-us/download](https://dotnet.microsoft.com/en-us/download) and download **.NET 6.0** for Windows x64. Run the installer.

**Python 3**. To install Python 3, go to [https://www.python.org/downloads/windows/](https://www.python.org/downloads/windows/), download the latest stable 64-bit installer, and run the installer.
Before selecting "Install now", make sure to choose "Add python.exe to PATH"
at the bottom of the dialog.

**Rust 1.76.0**. To install Rust 1.76.0, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and download "rustup-init.exe (64 bit)".
Run the installer. In the terminal window that appears, if you're asked to
install C++ prerequisites, choose a suitable option. For academic purposes,
you likely want to select Option 1: "Quick Install via the Visual Studio
Community Installer". Install both components: MSVC with C++ build tools, and
the Windows SDK. You can close Visual Studio Community if it opens.
 In the
terminal window for the Rust setup, select "Customize installation", then
confirm the default triple (`x86_64-pc-windows-msvc`), then choose `1.76.0` when
asked to choose a toolchain. Accept the default profile, accept the request to
modify the PATH, then proceed by pressing enter. When you get the message
"Rust is installed", press the Enter key to exit the setup.

**Git**. To install Git, go to
[https://git-scm.com/download/win](https://git-scm.com/download/win) and
download "64-bit Git for Windows Setup". Run the installer. You can disable
the "Windows Explorer Integration" feature if you prefer.

Leave the rest of the configuration as is. In particular, make sure that "Git
from the command line and from 3rd party software" is selected in the dialog
"Adjusting your PATH environment". In the dialog "Configuring the line ending
conversions", make sure "Checkout Windows-style, commit Unix-style line
endings" is selected. You can also choose not to install MinTTY and instead
use the default Windows console. 

#### Additional dependencies and configuration

Start Powershell by opening the start menu and typing "Powershell" followed by
the Enter key.

Install `scons` (a build system) by running `pip install scons`. This will only work if python is in the path.

Make sure git is configured to replace line endings with Windows line endings: `git config --global core.autocrlf true`.

*Important: Now make a new directory, e.g. "VerusAE", and run the following:*
```
Set-Variable -Name "VERUS_AE" -Value "C:\Users\Administrator\VerusAE"
```

where you replace the path following `-Value` with the path of the newly created "VerusAE" directory.

#### Clone the repository for this artifact:

```
cd $VERUS_AE
git clone -b main --single-branch https://github.com/verus-lang/paper-sosp24-artifact.git verus-sosp24-artifact
cd verus-sosp24-artifact
```

#### Build the IronFleet version of IronKV

* Download the Dafny 3.4.0 release, including its executable, using
  ```
  cd $VERUS_AE
  $download_url = "https://github.com/dafny-lang/dafny/releases/download/v3.4.0/dafny-3.4.0-x64-win.zip"
  Invoke-WebRequest -Uri $download_url -OutFile "$filename.zip"
  Expand-Archive -Path "$filename.zip" -DestinationPath "."
  Remove-Item "$filename.zip"
  ```

* Sync to commit `2fe4dcdc323b92e93f759cc3e373521366b7f691` of the
  Ironclad repository at `https://github.com/microsoft/Ironclad.git`.
  To do this, run the following from a terminal:
  ```
  cd $VERUS_AE
  git clone https://github.com/microsoft/Ironclad.git 
  cd Ironclad
  git checkout 2fe4dcdc323b92e93f759cc3e373521366b7f691
  ```

* Copy the updated SConstruct file from the Artifact Evaluation repo to the ironfleet
  directory to make it work with python 3.12, as follows (still in the `Ironclad` directory):
  ```
  cp ..\verus-sosp24-artifact\ironkv\SConstruct-dafny .\ironfleet\SConstruct
  ```

* To build ironclad run the following:
  ```
  cd $VERUS_AE/Ironclad/ironfleet
  scons --dafny-path=$VERUS_AE\dafny --no-verify
  ```

#### Build Verus

* Download the Verus source code from commit
  `097ac7ed283ae60375cd9b2b6017b3c629883b2b`
  of the repo at `https://github.com/verus-lang/verus`.
  To do this, run the following:
  ```
  cd $VERUS_AE
  git clone https://github.com/verus-lang/verus
  cd verus
  git checkout 097ac7ed283ae60375cd9b2b6017b3c629883b2b
  ```

* Build the Verus source code:
  ```
  cd source
  ..\tools\activate.ps1
  .\tools\get-z3.ps1
  vargo build --release
  ```

#### Build the Verus version of IronKV

* Download the Verus version of IronKV from commit
  `a593cce3c63c3cb64cbc36039eab217660bc172d` of the
  repository at `https://github.com/verus-lang/verified-ironkv.git`.
  To do this, run the following:
  ```
  cd $VERUS_AE
  git clone https://github.com/verus-lang/verified-ironkv.git
  cd verified-ironkv
  git checkout a593cce3c63c3cb64cbc36039eab217660bc172d
  ```

* Copy the updated SConstruct file from the Artifact Evaluation repo to the ironfleet
  directory to make it work with python 3.12, as follows (still in the `Ironclad` directory):
  ```
  cp ..\verus-sosp24-artifact\ironkv\SConstruct-verus SConstruct
  ```

* We will replace one of the IronKV files to make it operate on
  Windows (the main distribution is built for Linux, which exhibits sub-par performance for
  the baseline). Proceed as follows:
  ```
  cp ..\verus-sosp24-artifact\ironkv\Program.cs ironsht/csharp/IronSHTServer/Program.cs
  ```
  You can verify that the only change is for Windows compatibility by running `git diff` here.

* Build the Verus version of IronKV as follows:

  ```
  scons --verus-path=$VERUS_AE/verus
  ```
  This will create a `lib.dll` file in the top-level directory of that repository.

* Copy that `lib.dll` file to the `ironkv` subdirectory of the repository
  for this artifact, as follows:
  ```
  cp .\lib.dll ..\verus-sosp24-artifact\ironkv\.
  ```

#### Prepare to run the experiment.

* Change directory to the `ironkv` subdirectory of the repository for
  this artifact:
  ```
  cd $VERUS_AE/verus-sosp24-artifact/ironkv
  ```

* Generate certificates by running

  ```
  dotnet $VERUS_AE/Ironclad/ironfleet/bin/CreateIronServiceCerts.dll outputdir=certs name=MySHT type=IronSHT addr1=127.0.0.1 port1=4001 addr2=127.0.0.1 port2=4002 addr3=127.0.0.1 port3=4003
  ```

* With `pip`, install `numpy` and `scipy`:

  ```
  pip install numpy scipy
  ```

* If you are on a laptop, prepare your machine for the experiment by
  telling Windows to never sleep, by telling it to use the "best performance" power mode
  (to disable SpeedStep), and by plugging it into a real charging outlet (not
  just a USB-C connector to a monitor).

* Run the experiment by running
  ```
  cd $VERUS_AE/verus-sosp24-artifact/ironkv
  python compare.py $VERUS_AE
  ```
  This will overwrite the file `raw-data.txt` with its output.

* Generate the graph by running
  ```
  python gengraph.py
  ```
  which will output the results corresponding to Figure 9 in textual form.
  using the data stored in `raw-data.txt` by the experiment.
  The script will also generate a graph, in LaTeX
  format, in the file `ironfleet-port-plot.tex`. It is a self-contained latex
  document that depends on the packages `tikz` and `pgfplots`: you can download
  it and render it to obtain a plot corresponding to the one in the figure (however
  this contains the same data as what's printed on the terminal at the end of the
  experiment).


## Set 3 - Node Replication

### Claims

The three node-replication implementations (unverified Rust, IronSync and Verus) have similar
performance and scaling behavior (throughput).

On different hardware from what was used in the
paper the result may differ due to the changes in the number of replicas (one per node)
and ratio of threads per replica.
For example, compared to the hardware used for the paper,
on CloudLab r650 one can expect more contention on the reader-writer lock,
while there are less replicas to maintain (2 instead of 4). This will have different relative effects
on the three implementations: baseline, verified with Linear Dafny, and verified with Verus.
Despite this, the performance characteristics should still remain sufficiently similar.

### Instructions

Start a Linux x86_64 machine, with at least 2 NUMA nodes, and Ubuntu 22.04. **We recommend CloudLab r650.**

On CloudLab, use the default small-lan profile, and change parameters to select Ubuntu 22.04,
the correct node type (e.g. `r650`) and (expand "Advanced") enable "Temp Filesystem Max Space"
(for the `/mydata` mount).

If you run on CloudLab, you can follow the instructions that follow. If you start a different machine or VM, the only requirement
to follow the same instructions is that `/mydata` is a directory.
Note that the commands and scripts in the following will manipulate the permissions of `/mydata`.

#### 1. Installing Dependencies

The following instructions will install all dependencies required to build and run the benchmarks.

```shell
sudo apt-get update
sudo apt-get install -y curl wget liburcu-dev libhwloc-dev python3-venv texlive-xetex texlive-fonts-extra pkg-config clang make g++
```

Linear Dafny requires a specific version of libssl. You can install this with the following command:

```shell
sudo chown $USER /mydata
cd /mydata
wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb
sudo dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb
rm -rf libssl1.1_1.1.0g-2ubuntu4_amd64.deb

```

Install Rust using rustup toolchain installer (note that the last command starts with a `.`,
i.e. it sources the environment in the current shell):

```shell
(curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused -fsSL "https://sh.rustup.rs" \
  | sh -s -- -y)
# source the Rust environment
. "$HOME/.cargo/env"
```

#### 2. Repository Setup

Clone the Verified Node Replication repository and checkout commit `0391d6c3fae1501766d6d833ba317323d8b55b06` as follows:

```shell
cd /mydata
git clone https://github.com/verus-lang/verified-node-replication.git
cd verified-node-replication
git checkout 0391d6c3fae1501766d6d833ba317323d8b55b06
```

Initialize the submodules.

```shell
# inside verified-node-replication repo
git submodule update --init
```

 This should initialize three submodules:
   - `verus`
   - `benchmarks/ironsync/ironsync-osdi2023`
   - `benchmarks/lib/node-replication`


The repository should now be ready and we can build the binaries and run the benchmark.

#### 3. Running the Benchmark

To run the benchmarks, navigate into the `benchmarks` directory and execute the `run_benchmarks.sh`
script:

```shell
# inside verified-node-replication repo
cd benchmarks
bash run_benchmarks.sh
```

*Note: Running the entire benchmark may take a few hours. The more cores the longer it runs.*

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

#### 4. Obtaining the Results

You can view the results of the benchmark by opening the automatically generated plots.
From the local machine, copy the results off the CloudLab instance. On Linux you can use something like the following:

```shell
scp '<username>@<node>.cloudlab.us:/mydata/verified-node-replication/benchmarks/nr-results-throughput-vs-cores-numa-fill.pdf' .
```
(There is also a PNG version)

Each of the three subplots should have three lines (for Verus NR, IronSync NR and Upstream NR) that
are similar in performance and scaling behavior.
