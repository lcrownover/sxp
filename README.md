# sexpand

This tool should provide some of the basic hostname expansion capabilities
from the SLURM node notation.

For example:

From: `n1-n4`
To: `n1,n2,n3,n4`

From: `n[01-04]`
To: `n01,n02,n03,n04`

From: `n[02-03,09-11],n01`
To: `n01,n02,n03,n09,n10,n11`

## Install

```bash
curl https://github.com/lcrownover/sexpand/install.sh | bash
```

## Uninstall

```bash
curl https://github.com/lcrownover/sexpand/uninstall.sh | bash
```
