# # Generate xc_funcs enum bindings for libxc

# This script parses `xc_funcs_v*.h` header files and generates:
# - xc_funcs/mod.rs: module structure with version-gated submodules
# - xc_funcs/v6_2.rs, v7_0.rs, v7_1.rs: enum definitions per version
#
# Each functional (e.g. XC_LDA_X = 1) becomes an enum variant with:
# - A numeric value (#[repr(u32)])
# - Serde derive for serialization/deserialization
# - FromStr and Display implementations for string conversion
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


def to_enum_variant_name(raw_name):
    """Convert a C macro name like LDA_X to a Rust enum variant name like LdaX.

    Rules:
    - Split on underscores
    - Capitalize the first letter of each word, lowercase the rest
    - Keep common abbreviations recognizable (e.g., VWN, PBE, LDA, GGA, MGGA)
    """
    # For libxc functional names, we keep them SCREAMING_SNAKE_CASE
    # since that's the standard convention in the libxc ecosystem
    # and matches the C defines directly
    return raw_name


def generate_enum_file(version_str, feature_name, functionals):
    """Generate a Rust file with enum definition for a specific version."""

    # Sort by number to ensure consistent ordering
    functionals_sorted = sorted(functionals, key=lambda x: x[1])

    variants = []
    for name, number, comment in functionals_sorted:
        variant_name = to_enum_variant_name(name)
        variants.append(f'    /// {comment}')
        variants.append(f'    {variant_name} = {number},')

    variants_str = '\n'.join(variants)

    content = f"""//! Libxc functional IDs for API version {version_str}.
//!
//! This file is generated automatically from `xc_funcs_v{version_str}.h`.
//! Do not edit manually.

#![allow(non_camel_case_types)]
#![allow(clippy::enum_clike_unportable_variant)]

use serde::{{Deserialize, Serialize}};

/// Libxc functional identifier numbers.
///
/// Each variant corresponds to a `XC_*` define in the libxc headers.
/// The numeric values match the C library exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
pub enum XcFuncId {{
{variants_str}
}}

impl XcFuncId {{
    /// Get the numeric value of this functional ID.
    pub fn as_u32(self) -> u32 {{
        self as u32
    }}

    /// Get the C-style name string (e.g., "LDA_X", "GGA_X_PBE").
    pub fn name(self) -> &'static str {{
        match self {{
{generate_name_match_arms(functionals_sorted)}
        }}
    }}

    /// Get the description of this functional.
    pub fn description(self) -> &'static str {{
        match self {{
{generate_desc_match_arms(functionals_sorted)}
        }}
    }}
}}

impl std::fmt::Display for XcFuncId {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{}}", self.name())
    }}
}}

impl std::str::FromStr for XcFuncId {{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        // Try direct name match (case-insensitive)
        // Accept both "LDA_X" and "XC_LDA_X" forms
        let lookup = s.trim().to_uppercase();
        let lookup = lookup.strip_prefix("XC_").unwrap_or(&lookup);
{generate_from_str_match_arms(functionals_sorted)}
        Err(format!("Unknown libxc functional: {{s}}"))
    }}
}}

impl From<XcFuncId> for u32 {{
    fn from(id: XcFuncId) -> u32 {{
        id as u32
    }}
}}

impl TryFrom<u32> for XcFuncId {{
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {{
        match value {{
{generate_try_from_match_arms(functionals_sorted)}
            _ => Err(format!("Unknown libxc functional number: {{value}}"))
        }}
    }}
}}
"""
    return content


def generate_name_match_arms(functionals):
    """Generate match arms for the name() method."""
    arms = []
    for name, number, _ in functionals:
        variant = to_enum_variant_name(name)
        arms.append(f'            XcFuncId::{variant} => "{name}",')
    return '\n'.join(arms)


def generate_desc_match_arms(functionals):
    """Generate match arms for the description() method."""
    arms = []
    for name, number, comment in functionals:
        variant = to_enum_variant_name(name)
        escaped_comment = comment.replace('"', '\\"')
        arms.append(f'            XcFuncId::{variant} => "{escaped_comment}",')
    return '\n'.join(arms)


def generate_from_str_match_arms(functionals):
    """Generate match arms for FromStr implementation."""
    arms = []
    for name, number, _ in functionals:
        variant = to_enum_variant_name(name)
        arms.append(f'        if lookup == "{name}" {{ return Ok(XcFuncId::{variant}); }}')
    return '\n'.join(arms)


def generate_try_from_match_arms(functionals):
    """Generate match arms for TryFrom<u32> implementation."""
    arms = []
    for name, number, _ in functionals:
        variant = to_enum_variant_name(name)
        arms.append(f'            {number} => Ok(XcFuncId::{variant}),')
    return '\n'.join(arms)


def generate_mod_rs():
    """Generate the xc_funcs/mod.rs module structure.

    Since API features are cumulative (api-v7_0 implies api-v6_2),
    we must ensure only one XcFuncId is re-exported at the module level.
    We use cfg conditions to pick the highest enabled version.
    """
    lines = []

    # Module declarations - always expose all enabled version submodules
    for version_str, module_name, feature_name in versions:
        lines.append(f'#[cfg(feature = "{feature_name}")]')
        lines.append(f'pub mod {module_name};')
        lines.append('')

    # Re-export only the highest version's XcFuncId
    # Build a chain: v7_1 if enabled, else v7_0 if enabled, else v6_2
    # Using cfg-if style nesting with proper not(feature = "...") syntax
    for i, (version_str, module_name, feature_name) in enumerate(versions):
        # All higher versions that would shadow this one
        higher_features = [versions[j][2] for j in range(i + 1, len(versions))]

        if higher_features:
            not_parts = ', '.join(f'not(feature = "{f}")' for f in higher_features)
            lines.append(f'#[cfg(all(feature = "{feature_name}", {not_parts}))]')
        else:
            # Highest version, no exclusions needed
            lines.append(f'#[cfg(feature = "{feature_name}")]')

        lines.append(f'pub use {module_name}::XcFuncId;')
        lines.append('')

    content = f"""//! Libxc functional identifier enums.
//!
//! This module provides versioned enums representing the functional IDs
//! defined in `xc_funcs_v*.h`. Each API version has its own submodule
//! containing the `XcFuncId` enum for that version.
//!
//! The enums derive `serde::Serialize` and `serde::Deserialize` for
//! easy serialization. They also implement `FromStr`, `Display`,
//! `From<XcFuncId> for u32`, and `TryFrom<u32> for XcFuncId`.
//!
//! Since API features are cumulative, `XcFuncId` is re-exported from
//! the highest enabled version's submodule.

{chr(10).join(lines)}
"""
    return content


def main():
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
    import subprocess
    main()
