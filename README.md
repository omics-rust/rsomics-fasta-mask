# rsomics-fasta-mask

Mask FASTA sequences over BED regions — soft-mask (lowercase) or hard-mask
(replace with `N`). A Rust port of `bedtools maskfasta`.

## Install

```
cargo install rsomics-fasta-mask
```

## Usage

```
# soft-mask (lowercase) the BED regions
rsomics-fasta-mask ref.fa -b repeats.bed -o masked.fa

# hard-mask with N
rsomics-fasta-mask ref.fa -b repeats.bed --hard -o masked.fa
```

- `-b, --bed` — BED file of regions to mask (required).
- `--hard` — hard-mask with `N` instead of soft-masking to lowercase.
- `-o, --output` — output path (`-` = stdout).

## Origin

Independent Rust reimplementation of `bedtools maskfasta`, based on the BED and
FASTA format specs and black-box comparison against `bedtools`: soft-mask
(lowercase) and hard-mask (`N`) output over the same BED intervals is verified
against the upstream binary.

License: MIT OR Apache-2.0.
Upstream credit: [bedtools](https://github.com/arq5x/bedtools2) (MIT).
