#! usr/bin/bash
# Author Gaurav Sablok,
# Email: codeprog@icloud.com
# Date: 2025-22-8
for i in *.id; do md5sum $i; done >> estimatetag.txt
rm -rf *.id
