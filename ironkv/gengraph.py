import collections
import numpy
import os
import re
import scipy.stats
import sys

RAW_DATA_FILE = "raw-data.txt"
VALUE_SIZES = [ 128, 256, 512 ]
CONFIDENCE_QUANTILE = 0.95

def get_header_map(header_line):
    column_headings = header_line.strip().split('\t')
    num_columns = len(column_headings)
    which_column = {}
    for col, heading in enumerate(column_headings):
        which_column[heading] = col
    return (which_column, num_columns)

data = collections.defaultdict(list)
raw_data_in = open(RAW_DATA_FILE, "r")
(header_map, num_columns) = get_header_map(raw_data_in.readline())

for line in raw_data_in.readlines():
    values = line.strip().split('\t')
    if len(values) < num_columns:
        continue
    language = values[header_map['Language']]
    seconds = int(values[header_map['Seconds']])
    workload = values[header_map['Workload']]
    value_size = int(values[header_map['Value size']])
    num_requests_completed = int(values[header_map['Requests completed']])
    kops_per_sec = num_requests_completed * 0.001 / seconds
    key = f"{language}_{workload}_{value_size}"
    data[key].append(kops_per_sec)

print(r"""
\documentclass{article}
\usepackage{tikz}
\usepackage{pgfplots}
\begin{document}

\begin{figure}
  \begin{tikzpicture}
  \centering
  \begin{axis}[
        ybar,
        ymin = 0,
        ymax = 6,
        height=4.5cm, width=\columnwidth,
        legend image code/.code={ \draw [#1] (0cm,-0.1cm) rectangle (0.1cm,0.2cm); },
        legend style={at={(0.275, 0.95)}},
        bar width=0.4cm,
        ymajorgrids, tick align=inside,
        enlarge y limits={value=.1,upper},
        axis x line*=bottom,
        axis y line*=left,
        x tick label style={rotate=10,anchor=east,xshift=16pt,yshift=-8pt,font=\scriptsize},
        tickwidth=0pt,
        enlarge x limits=true,
        xlabel={Workload type, bytes per value},
        ylabel={Throughput (kop/s)},
        symbolic x coords={
           Get 128,Get 256,Get 512,Set 128,Set 256,Set 512
        },
       xtick=data
    ]
""")

for language in ['dafny', 'verus']:
    printable_language = "Verus" if language == "verus" else "IronFleet"
    printable_color = "teal" if language == "verus" else "red"
    printable_pattern = "" if language == "verus" else ",postaction={pattern=north east lines}"
    print(r"\addplot [draw=none, fill=%s!100%s,error bars/.cd, y dir=both, y explicit] coordinates {" % (printable_color, printable_pattern))
    for workload in ['g', 's']:
        printable_workload = 'Get' if workload == 'g' else 'Set'
        for value_size in VALUE_SIZES:
            printable_value_size = value_size
            key = f"{language}_{workload}_{value_size}"
            if len(data[key]) == 0:
                print(f"Could not find data for key {key}")
                sys.exit(-1)
            a = numpy.array(data[key])
            mean = numpy.mean(a)
            std_err = scipy.stats.sem(a)
            (conf95_l, conf95_r) = scipy.stats.t.interval(confidence=CONFIDENCE_QUANTILE, df=len(a) - 1, loc=mean, scale=std_err)
            conf95_diff = conf95_r - mean
            xlabel=f"{printable_workload} {printable_value_size}"
            print(f"({xlabel},{mean}) += (0, {conf95_diff}) -= (0, {conf95_diff})")
    print("};")

print(r"""
    \legend{IronFleet,Verus}
  \end{axis}
  \end{tikzpicture}
\caption{Throughput comparison of IronFleet and Verus versions of IronSHT. Workload varies between Get/Set and in how many bytes are in each value. Each bar shows the mean of 100 trials; error bars show 95\% confidence intervals.\label{fig:ironsht-throughput-comparison}}
\end{figure}

\end{document}
""")
