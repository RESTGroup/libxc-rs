# # Generate xc_funcs enum bindings for libxc

# This script parses `xc_funcs_v*.h` header files and generates:
# - xc_funcs/mod.rs: module structure with version-gated submodules
# - xc_funcs/v6_2.rs, v7_0.rs, v7_1.rs: enum definitions per version
#
# Each functional (e.g. XC_LDA_X = 1) becomes an enum variant with:
# - A numeric value (#[repr(u32)])
# - A doc comment from the header

import os
import re

path_cwd = os.path.abspath(os.getcwd())
path_header = f"{path_cwd}/../headers"
path_out = f"{path_cwd}/../src/xc_funcs"

# Version configuration
versions = [
    ("6.2", "v6_2", "api-v6_2"),
    ("7.0", "v7_0", "api-v7_0"),
    ("7.1", "v7_1", "api-v7_1"),
]


def parse_xc_funcs_header(header_path):
    """Parse an xc_funcs header file and extract functional definitions.

    Returns a list of (name, number, comment) tuples.
    e.g. [("LDA_X", 1, "Slater exchange"), ("LDA_C_WIGNER", 2, "Wigner"), ...]
    """
    functionals = []
    pattern = re.compile(
        r'#define\s+XC_(\w+)\s+(\d+)\s*/\*\s*(.*?)\s*\*/'
    )

    with open(header_path, "r") as f:
        for line in f:
            match = pattern.match(line.strip())
            if match:
                name = match.group(1)
                number = int(match.group(2))
                comment = match.group(3)
                functionals.append((name, number, comment))

    return functionals


def generate_enum_file(version_str, feature_name, functionals):
    """Generate a Rust file with enum definition for a specific version."""

    # Sort by number to ensure consistent ordering
    functionals_sorted = sorted(functionals, key=lambda x: x[1])

    variants = []
    for name, number, comment in functionals_sorted:
        variants.append(f'    /// {comment}')
        variants.append(f'    {name} = {number},')

    variants_str = '\n'.join(variants)

    content = f"""//! Libxc functional IDs for API version {version_str}.
//!
//! This file is generated automatically from `xc_funcs_v{version_str}.h`.
//! Do not edit manually.

#![allow(non_camel_case_types)]
#![allow(clippy::enum_clike_unportable_variant)]
use strum::EnumIter;

/// Libxc functional identifier numbers.
///
/// Each variant corresponds to a `XC_*` define in the libxc headers.
/// The numeric values match the C library exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
#[repr(u32)]
pub enum XcFuncId {{
{variants_str}
}}
"""
    return content


def generate_mod_rs():
    """Generate the xc_funcs/mod.rs module structure.

    Since API features are cumulative (api-v7_0 implies api-v6_2),
    we must ensure only one submodule and one XcFuncId re-export are active.
    We use cfg(all(feature, not(higher))) to pick the highest enabled version.
    """
    lines = []

    # Module declarations - mutually exclusive: only the highest enabled version.
    # Since features are cumulative, we only need to exclude the immediate next
    # higher version (e.g. api-v7_1 implies api-v7_0, so not(api-v7_0) suffices).
    for i, (version_str, module_name, feature_name) in enumerate(versions):
        next_feature = versions[i + 1][2] if i + 1 < len(versions) else None

        if next_feature:
            lines.append(f'#[cfg(all(feature = "{feature_name}", not(feature = "{next_feature}")))]')
        else:
            lines.append(f'#[cfg(feature = "{feature_name}")]')

        lines.append(f'pub mod {module_name};')
        lines.append('')

    # Re-export only the highest version's XcFuncId.
    # Same logic: only exclude the immediate next higher version.
    for i, (version_str, module_name, feature_name) in enumerate(versions):
        next_feature = versions[i + 1][2] if i + 1 < len(versions) else None

        if next_feature:
            lines.append(f'#[cfg(all(feature = "{feature_name}", not(feature = "{next_feature}")))]')
        else:
            lines.append(f'#[cfg(feature = "{feature_name}")]')

        lines.append(f'pub use {module_name}::XcFuncId;')
        lines.append('')

    content = f"""//! Libxc functional identifier enums.
//!
//! This module provides versioned enums representing the functional IDs
//! defined in `xc_funcs_v*.h`. Each API version has its own submodule
//! containing the `XcFuncId` enum for that version.
//!
//! Since API features are cumulative, `XcFuncId` is re-exported from
//! the highest enabled version's submodule.

{chr(10).join(lines)}
"""
    return content


def main():
    import subprocess

    os.makedirs(path_out, exist_ok=True)

    # Generate mod.rs
    with open(f"{path_out}/mod.rs", "w") as f:
        f.write(generate_mod_rs())

    # Generate per-version enum files
    for version_str, module_name, feature_name in versions:
        header_path = f"{path_header}/xc_funcs_v{version_str}.h"
        if not os.path.exists(header_path):
            print(f"Warning: {header_path} not found, skipping {module_name}")
            continue

        functionals = parse_xc_funcs_header(header_path)
        print(f"Parsed {len(functionals)} functionals from xc_funcs_v{version_str}.h")

        content = generate_enum_file(version_str, feature_name, functionals)
        with open(f"{path_out}/{module_name}.rs", "w") as f:
            f.write(content)

        print(f"Generated: {path_out}/{module_name}.rs")

    # Run cargo fmt
    os.chdir(f"{path_out}/../..")
    subprocess.run(["cargo", "fmt"])

    print("Done generating xc_funcs enums.")


if __name__ == "__main__":
    main()
