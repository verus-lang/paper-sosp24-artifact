tlmgr update --self 
tlmgr install multirow

(cd summarize/render; \
    pdflatex macro-table.tex; \
    pdflatex macro-table.tex; \
    pdflatex macro-table.tex)

mv summarize/render/macro-table.pdf results/.
