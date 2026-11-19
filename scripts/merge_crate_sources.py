#!/usr/bin/env python3
"""Merge the sources of the workspace's crates into one src/ tree, as the single crate had them.

The client is generated as a workspace (timvw-rust `splitCrates`): the facade's src/ re-exports the crates under
crates/, each with src/models/ and src/apis/. The scripts that read the generated sources (the public-API census)
expect one src/ tree. This writes the facade's src/ and every crate's src/models/ and src/apis/ into OUT, appending
files that more than one crate has (models/mod.rs, apis/mod.rs, an API module split between crates) in a fixed order:
the facade first, then the crates by name. A tree without crates/ (a release before the split) is copied as is.

usage: merge_crate_sources.py ROOT OUT   (ROOT holds src/ and, after the split, crates/)
"""
import sys
from pathlib import Path


def merge(root: Path, out: Path) -> None:
    trees = [root / "src"]
    crates = root / "crates"
    if crates.is_dir():
        trees += sorted(p / "src" for p in crates.iterdir() if (p / "src").is_dir())
    for tree in trees:
        for path in sorted(tree.rglob("*.rs")):
            rel = path.relative_to(tree)
            if tree != root / "src" and rel.parts[0] not in ("models", "apis"):
                continue  # a crate's lib.rs
            target = out / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open("a") as f:
                f.write(path.read_text())


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    merge(Path(sys.argv[1]), Path(sys.argv[2]))
