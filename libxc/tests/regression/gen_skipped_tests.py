#!/usr/bin/env python3
"""Generate skipped_tests from libxc's xc-generate_tests.py skipped_cases list.

Parses the ``skipped_cases`` triple-quoted string in xc-generate_tests.py,
extracts (category, xc_name, species) from each entry, and writes them
one-per-line in ``category.xc_name.species`` format.

Usage:
    python gen_skipped_tests.py [path_to_xc_generate_tests.py] [output_file]

If no source path is given, uses $LIBXC_REPO_PATH/testsuite/xc-generate_tests.py.
Output defaults to skipped_tests in the script's directory.
"""

import os
import re
import sys
from collections import OrderedDict
from pathlib import Path
from typing import Set, Tuple


# Category prefixes, longest first to avoid partial matches
CATEGORY_PREFIXES = [
    "hyb_lda_xc", "hyb_gga_xc", "hyb_mgga_xc",
    "hyb_lda_x", "hyb_gga_x", "hyb_mgga_x",
    "hyb_lda_c", "hyb_gga_c", "hyb_mgga_c",
    "lda_xc", "lda_x", "lda_c",
    "gga_xc", "gga_x", "gga_c",
    "mgga_xc", "mgga_x", "mgga_c",
]

# Species suffixes, longest first to avoid partial matches
SPECIES_LIST = [
    "BrOH_cation_restr",
    "BrOH_cation",
    "BrOH",
    "H_restr",
    "Li_restr",
    "H",
    "Li",
]


def parse_skip_list(generate_tests_path: Path) -> Set[str]:
    """Read the skipped_cases list from xc-generate_tests.py."""
    source = generate_tests_path.read_text()
    m = re.search(r"skipped_cases\s*=\s*'''(.*?)'''", source, re.DOTALL)
    if not m:
        return set()
    return set(m.group(1).split())


def parse_case(case: str) -> Tuple[str, str, str]:
    """Parse a skipped_cases entry into (category, xc_name, species).

    Example:
        test_mgga_x_2d_prhg07_prp10_BrOH_cation_2_vrho
        -> ("mgga_x", "2d_prhg07_prp10", "BrOH_cation")
    """
    # strip "test_"
    case = case[5:]
    # strip last two parts (spin and output property like "_2_vrho")
    case = "_".join(case.split("_")[:-2])

    # extract category
    kind = None
    for prefix in CATEGORY_PREFIXES:
        if case.startswith(prefix):
            kind = prefix
            case = case[len(prefix):]
            break
    if kind is None:
        raise ValueError(f"Cannot determine category for: {case}")

    # extract species (longest match first)
    species = None
    for sp in SPECIES_LIST:
        if case.endswith(sp):
            species = sp
            case = case[: -len(sp)]
            break
    if species is None:
        raise ValueError(f"Cannot determine species for: {case}")

    # the rest is xc_name
    xc_name = case.strip("_")
    if not xc_name:
        raise ValueError(f"Empty xc_name for case: {case}")

    return kind, xc_name, species


def main() -> None:
    script_dir = Path(__file__).resolve().parent
    default_output = script_dir / "skipped_tests"

    if len(sys.argv) > 1:
        generate_tests_path = Path(sys.argv[1])
    else:
        libxc_repo = os.environ.get("LIBXC_REPO_PATH")
        if not libxc_repo:
            sys.exit("Set LIBXC_REPO_PATH or pass the xc-generate_tests.py path as argument")
        generate_tests_path = Path(libxc_repo) / "testsuite/xc-generate_tests.py"
    output_path = Path(sys.argv[2]) if len(sys.argv) > 2 else default_output

    skip_set = parse_skip_list(generate_tests_path)
    print(f"Loaded {len(skip_set)} skip entries from {generate_tests_path}")

    # Deduplicate by (category, xc_name, species) — multiple output keys
    # for the same functional/species collapse to one entry
    data = OrderedDict()
    for case in sorted(skip_set):
        kind, xc_name, species = parse_case(case)
        key = (kind, xc_name, species)
        if key not in data:
            data[key] = None

    lines = [f"{kind}.{xc_name}.{species}" for (kind, xc_name, species) in data.keys()]
    output_path.write_text("\n".join(lines) + "\n")
    print(f"Wrote {len(lines)} unique (category, xc_name, species) entries to {output_path}")


if __name__ == "__main__":
    main()
