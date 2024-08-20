#!/bin/bash
#
# jonh: my vague recollection is that we hacked sloccount to count Dafny
# as C#, however the repo doesn't record how that hack was implemented.
# Here I just add an alias in the break_files table, but the output
# is still labeled "cs", which isn't what the ironfleet repo's dafny-line-count
# was expecting. I'm hoping there wasn't other clever stuff in the
# ancient sloccount patch that we're not reproducing. Sands of time.
#
rm -rf sloccount-2.26*
wget --quiet https://dwheeler.com/sloccount/sloccount-2.26.tar.gz
tar xzf sloccount-2.26.tar.gz
rm sloccount-2.26.tar.gz
(cd sloccount-2.26; patch -p1 < ../../../dafny-ironfleet/sloccount.patch)
(cd sloccount-2.26; make)
