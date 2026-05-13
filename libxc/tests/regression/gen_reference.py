#!/usr/bin/env python3
"""Generate reference.toml from libxc testsuite/regression Python test files.

Parses each test_<func>_<species>.py file to extract the reference values
embedded in ``ref_tgt = ns.asarray([...])`` expressions, then writes them
as a nested TOML table keyed by [category.xc_name.species].

Usage:
    python gen_reference.py [path_to_libxc_testsuite_regression] [output.toml]

If no source path is given, uses $LIBXC_REPO_PATH/testsuite/regression.
Output defaults to reference.toml in the script's directory.
"""

import math
import os
import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# Known species suffixes (must be stripped from the end of the functional+species
# string to separate functional name from species).
SPECIES_LIST = [
    "BrOH_cation_restr",
    "BrOH_cation",
    "BrOH",
    "H_restr",
    "Li_restr",
    "H",
    "Li",
]


def parse_ref_values(source: str) -> Dict[str, List[float]]:
    """Extract output_key -> reference_values from a Python test file.

    Each test function contains a line like:
        ref_tgt = ns.full_like(tgt, ns.asarray([1.0e+00, 2.0e+00, ...]))
    or for single-value cases:
        ref_tgt = ns.asarray([1.0e+00, ...])

    Returns a dict mapping output key (zk, vrho, vsigma, vtau, vlapl) to
    the list of reference float values.
    """
    result = {}
    # Match test function definitions to get the output key
    # Pattern: def test_<func>_<species>_<nspin>_<key>():
    func_pattern = re.compile(r"def test_(\w+)_(\d+)_(\w+)\(\):")
    # Pattern for ref_tgt array values
    ref_pattern = re.compile(
        r"ref_tgt\s*=\s*(?:ns\.full_like\([^,]+,\s*)?ns\.asarray\(\[([^\]]+)\]\)",
    )

    lines = source.split("\n")
    current_key = None
    for line in lines:
        fm = func_pattern.match(line.strip())
        if fm:
            current_key = fm.group(3)  # zk, vrho, vsigma, vtau, vlapl
            continue
        rm = ref_pattern.search(line)
        if rm and current_key:
            raw = rm.group(1)
            values = [float(v.strip()) for v in raw.split(",") if v.strip()]
            result[current_key] = values
            current_key = None
    return result


def split_func_species(stem: str) -> Optional[Tuple[str, str]]:
    """Split a filename stem like ``test_gga_c_acgga_Li`` into
    (functional, species) where functional is ``gga_c_acgga``.

    Returns None if no known species suffix is found.
    """
    if not stem.startswith("test_"):
        return None
    rest = stem[len("test_"):]
    for species in SPECIES_LIST:
        if rest.endswith("_" + species):
            func = rest[: -(len(species) + 1)]
            if func:
                return func, species
    return None


def split_category_xcname(func: str) -> Tuple[str, str]:
    """Split a functional like ``gga_c_acgga`` or ``hyb_mgga_xc_tpssh``
    into (category, xc_name).

    Category prefixes: lda_*, gga_*, mgga_*, hyb_lda_*, hyb_gga_*, hyb_mgga_*
    """
    hyb_prefixes = ["hyb_lda_", "hyb_gga_", "hyb_mgga_"]
    for prefix in hyb_prefixes:
        if func.startswith(prefix):
            # category includes the x/c/xc component: e.g. hyb_mgga_xc, hyb_gga_x
            parts = func[len(prefix):].split("_", 1)
            if len(parts) == 2:
                category = prefix.rstrip("_") + "_" + parts[0]
                xc_name = parts[1]
            else:
                category = prefix.rstrip("_")
                xc_name = parts[0]
            return category, xc_name

    plain_prefixes = ["lda_", "gga_", "mgga_"]
    for prefix in plain_prefixes:
        if func.startswith(prefix):
            # category is the first two components: e.g. gga_c, mgga_x
            parts = func[len(prefix):].split("_", 1)
            if len(parts) == 2:
                category = prefix.rstrip("_") + "_" + parts[0]
                xc_name = parts[1]
            else:
                category = prefix.rstrip("_")
                xc_name = parts[0]
            return category, xc_name

    # Fallback: first component is category
    parts = func.split("_", 1)
    return parts[0], parts[1] if len(parts) > 1 else parts[0]


def write_toml(
    data: Dict[str, Dict[str, Dict[str, Dict[str, List[float]]]]],
    path: Path,
) -> None:
    """Write the nested dict as TOML.

    Structure: {category: {xc_name: {species: {key: [values]}}}}
    Output: [category.xc_name.species]
            key = [v1, v2, ...]
    """
    lines = []
    # Sort categories, then xc_names, then species for deterministic output
    for category in sorted(data):
        for xc_name in sorted(data[category]):
            for species in sorted(data[category][xc_name]):
                lines.append(f"[{category}.{xc_name}.{species}]")
                entries = data[category][xc_name][species]
                for key in ("zk", "vrho", "vsigma", "vtau", "vlapl"):
                    if key in entries:
                        formatted = ", ".join(
                            f"{v:.15e}" for v in entries[key]
                        )
                        lines.append(f"{key} = [{formatted}]")
                lines.append("")
    path.write_text("\n".join(lines))


def main() -> None:
    script_dir = Path(__file__).resolve().parent
    default_output = script_dir / "reference.toml"

    if len(sys.argv) > 1:
        regression_dir = Path(sys.argv[1])
    else:
        libxc_repo = os.environ.get("LIBXC_REPO_PATH")
        if not libxc_repo:
            sys.exit("Set LIBXC_REPO_PATH or pass the regression dir as argument")
        regression_dir = Path(libxc_repo) / "testsuite/regression"
    output_path = Path(sys.argv[2]) if len(sys.argv) > 2 else default_output

    # Collect reference data
    # {category: {xc_name: {species: {key: [values]}}}}
    ref_data: Dict[str, Dict[str, Dict[str, Dict[str, List[float]]]]] = defaultdict(
        lambda: defaultdict(lambda: defaultdict(dict))
    )
    parsed = 0
    nans_skipped = 0

    for category_dir in sorted(regression_dir.iterdir()):
        if not category_dir.is_dir():
            continue
        for test_file in sorted(category_dir.glob("test_*.py")):
            stem = test_file.stem
            result = split_func_species(stem)
            if result is None:
                continue
            func, species = result

            # Parse the Python test file
            source = test_file.read_text()
            ref_values = parse_ref_values(source)
            if not ref_values:
                continue

            cat, xc_name = split_category_xcname(func)

            for key, values in ref_values.items():
                # Skip NaN values
                if any(math.isnan(v) for v in values):
                    nans_skipped += 1
                    continue
                ref_data[cat][xc_name][species][key] = values
                parsed += 1

    print(f"Parsed {parsed} reference entries")
    if nans_skipped:
        print(f"Skipped {nans_skipped} entries with NaN values")

    write_toml(ref_data, output_path)
    print(f"Wrote {output_path}")


if __name__ == "__main__":
    main()
