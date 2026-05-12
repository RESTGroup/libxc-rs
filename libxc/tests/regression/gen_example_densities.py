#!/usr/bin/env python3
"""Generate example_densities.toml from pylibxc/example_densities.py.

Parses the Python source to extract raw density arrays and writes them
as TOML. The TOML stores flat arrays in row-major order with 9 variables
per grid point: (rho_a, rho_b, sigma_aa, sigma_ab, sigma_bb, lapl_a, lapl_b, tau_a, tau_b).

Usage:
    python gen_example_densities.py [path_to_example_densities.py] [output.toml]

If no source path is given, uses $LIBXC_REPO_PATH/pylibxc/example_densities.py.
Output defaults to example_densities.toml in the script's directory.
"""

import os
import re
import sys
from pathlib import Path

# Map from Python variable name to TOML key
NAME_MAP = {
    "BrOH": "BrOH",
    "BrOHc": "BrOH_cation",
    "H": "H",
    "Li": "Li",
}


def parse_python_arrays(source: str) -> dict[str, list[float]]:
    """Extract ``<Name>_data = asarray([...])`` arrays from the Python source."""
    pattern = re.compile(
        r'(\w+)_data\s*=\s*asarray\(\[([^\]]+)\]\)',
        re.DOTALL,
    )
    result = {}
    for m in pattern.finditer(source):
        name = m.group(1)
        if name not in NAME_MAP:
            continue
        numbers = [float(x.strip()) for x in m.group(2).split(",") if x.strip()]
        result[NAME_MAP[name]] = numbers
    return result


def write_toml(data: dict[str, list[float]], path: Path) -> None:
    lines = [
        "# Density of this file is in format of",
        "# row-major, [grids, nvar]",
        "# where nvar = 9, corresponds to (rho_a/b, sigma_aa/ab/bb, tau_a/b, lapl_a/b)",
        "",
    ]
    for key in ("BrOH", "BrOH_cation", "H", "Li"):
        values = data[key]
        formatted = ", ".join(f"{v:.16e}" for v in values)
        lines.append(f"{key} = [{formatted}]")
    path.write_text("\n".join(lines) + "\n")


def main() -> None:
    script_dir = Path(__file__).resolve().parent
    default_output = script_dir / "example_densities.toml"

    if len(sys.argv) > 1:
        source_path = Path(sys.argv[1])
    else:
        libxc_repo = os.environ.get("LIBXC_REPO_PATH")
        if not libxc_repo:
            sys.exit("Set LIBXC_REPO_PATH or pass the source file path as argument")
        source_path = Path(libxc_repo) / "pylibxc/example_densities.py"
    output_path = Path(sys.argv[2]) if len(sys.argv) > 2 else default_output

    source = source_path.read_text()
    data = parse_python_arrays(source)
    missing = [k for k in NAME_MAP.values() if k not in data]
    if missing:
        sys.exit(f"Missing data for keys: {missing}")
    write_toml(data, output_path)
    print(f"Wrote {output_path}")


if __name__ == "__main__":
    main()
