#!/usr/bin/env python3
import os

table = r"""\begin{tabular}{l|rrr|r|rr|r}
  \multirow{2}{1.6cm}{\vfill{}System \\ $\rightarrow$ Verifier}
  & \multicolumn{3}{c|}{Line Count}
  & \multirow{2}{.2cm}{\tablerot{P/C Ratio}}
  & \multicolumn{2}{c|}{Time (s)}
  & \multirow{2}{.2cm}{\tablerot{SMT (MB)}} \\

   & \tablerot{trusted}
   & \tablerot{proof}
   & \tablerot{code}
   &
   & \tablerot{1 core}
   & \tablerot{\evalParallelNumThreads\xspace cores}
   & \\ \hline
"""

HLINE = "hline"
class Row:
    def __init__(self, label, time1, time8, trusted, proof, exec, pcratio, smt):
        self.label = label
        self.time1 = time1
        self.time8 = time8
        self.trusted = trusted
        self.proof = proof
        self.exec = exec
        self.pcratio = pcratio
        self.smt = smt

# ha ha ha we can't compute the pcratio because we don't have the data! only symbols
#    def compute_pcratio(self):
#        if self.proof == "":
#            return ""
#        return ".1f" % (float(self.proof)/float(self.exec))


def empty(label):
    return Row(label, " ", " ", " ", " ", " ", " ", " ")

rows = [
    empty("IronKV"),
    Row(r"$\rightarrow$ \name",
        r"\evalVerusIronshtSinglethreadWallTime",
        r"\evalVerusIronshtParallelWallTime",
        r"\evalVerusIronshtLineCountTrusted",
        r"\evalVerusIronshtLineCountProof",
        r"\evalVerusIronshtLineCountExec",
        r"\evalVerusIronshtLineCountProofCodeRatio",
        r"\evalVerusIronshtEncodingSizeMB",
        ),
    Row(r"$\rightarrow$ Dafny",
        r"\evalDafnyIronshtSinglethreadWallTime",
        r"\evalDafnyIronshtParallelWallTime",
        r"\evalDafnyIronshtSpecLines",
        r"\evalDafnyIronshtProofLines",
        r"\evalDafnyIronshtImplLines",
        r"\evalDafnyIronshtProofCodeRatio",
        r"\ironKVDafnySMTMB",
        ),
    HLINE,
    empty("NR"),
    Row(r"$\rightarrow$ \name",
        r"\evalVerusNrSinglethreadWallTime",
        r"\evalVerusNrParallelWallTime",
        r"\evalVerusNrLineCountTrusted",
        r"\evalVerusNrLineCountProof",
        r"\evalVerusNrLineCountExec",
        r"\evalVerusNrLineCountProofCodeRatio",
        r"\evalVerusNrEncodingSizeMB",
        ),
    Row(r"$\rightarrow$ L.Dafny",
		r"\evalLinearDafnyNrSinglethreadWallTime",
		r"\evalLinearDafnyNrParallelWallTime",
		r"\evalLinearDafnyNrLineCountTrusted",
		r"\evalLinearDafnyNrLineCountProof",
		r"\evalLinearDafnyNrLineCountExec",
		r"\evalLinearDafnyNrLineCountProofCodeRatio",
        r"\nrDafnySMTMB",
        ),
    HLINE,
    HLINE,
    Row("Page table",
        r"\evalVerusPageTableSinglethreadWallTime",
        r"\evalVerusPageTableParallelWallTime",
        r"\evalVerusPageTableLineCountTrustedAdjusted",
        r"\evalVerusPageTableLineCountProofAdjusted",
        r"\evalVerusPageTableLineCountExecAdjusted",
        r"\evalVerusPageTableLineCountProofCodeRatioAdjusted",
        r"\evalVerusPageTableEncodingSizeMB",
        ),
    HLINE,
    Row("Mimalloc",
        r"\evalVerusMimallocSinglethreadWallTime",
        r"\evalVerusMimallocParallelWallTime",
        r"\allocatorTotalTrusted",
        r"\allocatorProof",
        r"\allocatorExec",
        r"\allocatorProofToCodeRatio",
        r"\evalVerusMimallocEncodingSizeMB",
        ),
    HLINE,
    Row("P. log",
        r"\evalVerusPmemlogSinglethreadWallTime",
        r"\evalVerusPmemlogParallelWallTime",
        r"\evalVerusPmemlogLineCountTrusted",
        r"\evalVerusPmemlogLineCountProof",
        r"\evalVerusPmemlogLineCountExec",
        r"\evalVerusPmemlogLineCountProofCodeRatio",
        r"\evalVerusPmemlogEncodingSizeMB",
        ),
    HLINE,
    HLINE,
    Row("Verus total",
        r"",
        r"",
        r"\totalVerusLinesTrusted",
        r"\totalVerusLinesProof",
        r"\totalVerusLinesExec",
        r"\totalVerusLinesProofCodeRatio",
        r"",
        ),
    ]

def strip_right(cols):
    if len(cols)==0: return cols
    if cols[-1] != "": return cols
    return strip_right(cols[:-1])

#  % computed with ../eval/totals_data.py
for row in rows:
    if row == HLINE:
        table += r" \hline"
        continue
    cols = [
        row.label,
        row.trusted,
        row.proof,
        row.exec,
        row.pcratio,
        row.time1,
        row.time8,
        row.smt,
        ]
    cols = strip_right(cols)
    table += "  " + (" & ".join(cols)) + r" \\"
    table += "\n"

#table += r"""  \multicolumn{3}{l|}{XXX Verus total} & 3852 & 30437 & 5757 & 5.3 &
table += r"""
\end{tabular}
"""

script_path = os.path.dirname(__file__)
out_path = os.path.join(script_path, "../paper/plots/monster-table.tex")
open(out_path, "w").write(table)


