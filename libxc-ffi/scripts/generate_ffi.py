# # Generate FFI bindings for libxc (xc.h)

# This script generates:
# - ffi_xc_static.rs: static linking FFI bindings
# - ffi_xc_dynamic/: dynamic loading module files

import subprocess
import os
import shutil
import re
from tree_sitter import Language, Parser
import tree_sitter_rust

path_cwd = os.path.abspath(os.getcwd())

# ## Bindgen configuration

# Source headers directory
path_header = f"{path_cwd}/../headers"

# Path for temporary files
path_temp = f"{path_cwd}/../tmp"

# Path for bindgen crate root (libxc-ffi/src)
path_out = f"{path_cwd}/../src"

# ## API version configuration

# Available API versions and their cargo feature names
# Versions are cumulative: api-v7_0 includes api-v6_2
api_versions = [
    ("v6_2", "api-v6_2"),
    ("v7_0", "api-v7_0"),
    ("v7_1", "api-v7_1"),
]

# Default API version
default_api_version = "api-v7_0"

# Functions introduced in each version (from versioning_xc.md)
# v6_2 is the base - all functions not listed below belong to v6_2
version_new_functions = {
    "v7_0": [
        "xc_func_set_fhc_enforcement",
        "xc_func_get_ext_params",
        "xc_func_get_ext_params_name",
        "xc_func_get_ext_params_value",
        "xc_mgga_new",
    ],
    "v7_1": [
        "xc_func_info_get_default_flags",
        "xc_func_info_set_default_flags",
        "xc_func_init_flags",
    ],
}

# Struct fields that differ between versions
# v7_0 added `ext_params` field to `xc_func_type`
version_struct_changes = {
    "v7_0": {
        "xc_func_type": ["ext_params"],
    },
}


# ## Parse version information from versioning_xc.md

def parse_versioning_md(path):
    """Parse versioning_xc.md to extract function-to-version mapping."""
    version_map = {}
    current_version = None

    with open(path, "r") as f:
        for line in f:
            line = line.strip()

            # Match version headers like "# v7.0" or "## Introduced in v7.0"
            version_match = re.match(r'^#+\s*(?:Introduced in\s+)?v(\d+)\.(\d+)', line)
            if version_match:
                major, minor = version_match.group(1), version_match.group(2)
                current_version = f"v{major}_{minor}"
                continue

            # Match function names (identifiers starting with xc_)
            if current_version:
                func_match = re.match(r'^(xc_\w+)', line)
                if func_match:
                    func_name = func_match.group(1)
                    version_map[func_name] = current_version

    return version_map


# ## Static FFI generation functions

def get_feature_for_version(version_str):
    """Convert version string (v7_0) to cargo feature name (api-v7_0)."""
    for v_suffix, feature_name in api_versions:
        if v_suffix == version_str:
            return feature_name
    return default_api_version


def build_func_version_map(version_map):
    """Build a mapping from function name to the earliest feature that includes it."""
    func_cfg_map = {}
    for func_name, version_suffix in version_map.items():
        feature = get_feature_for_version(version_suffix)
        func_cfg_map[func_name] = feature
    return func_cfg_map


def remove_xc_version_constants(token):
    """Remove constants from xc_version.h that leaked into bindgen output.

    Since xc.h includes <xc_version.h>, bindgen picks up:
      - XC_VERSION, XC_MAJOR_VERSION, XC_MINOR_VERSION, XC_MICRO_VERSION
    These are version-specific and should not be in the FFI bindings.
    """
    # Remove the #define constants from xc_version.h
    lines = token.split('\n')
    result = []
    skip_patterns = [
        'pub const XC_VERSION:',
        'pub const XC_MAJOR_VERSION:',
        'pub const XC_MINOR_VERSION:',
        'pub const XC_MICRO_VERSION:',
    ]
    for line in lines:
        if any(line.strip().startswith(p) for p in skip_patterns):
            continue
        result.append(line)
    return '\n'.join(result)


def add_struct_version_attributes(token, struct_changes):
    """Add #[cfg(feature = "...")] attributes to struct fields that differ between versions.

    struct_changes is a dict like:
        {"v7_0": {"xc_func_type": ["ext_params"]}}

    For each field listed, a #[cfg(feature = "api-vX_Y")] attribute is added
    before the field declaration.
    """
    # Build a flat map: (struct_name, field_name) -> feature_name
    field_cfg_map = {}
    for version_suffix, structs in struct_changes.items():
        feature = get_feature_for_version(version_suffix)
        for struct_name, fields in structs.items():
            for field_name in fields:
                field_cfg_map[(struct_name, field_name)] = feature

    if not field_cfg_map:
        return token

    lines = token.split('\n')
    result_lines = []
    current_struct = None

    for i, line in enumerate(lines):
        stripped = line.strip()

        # Track which struct we're inside
        struct_match = re.match(r'^pub struct (\w+)', stripped)
        if struct_match:
            current_struct = struct_match.group(1)

        # Detect closing brace of struct
        if current_struct and stripped == '}':
            current_struct = None

        # Check if this line is a struct field
        if current_struct:
            field_match = re.match(r'^(\s+)pub (\w+):', line)
            if field_match:
                indent = field_match.group(1)
                field_name = field_match.group(2)
                key = (current_struct, field_name)
                if key in field_cfg_map:
                    feature = field_cfg_map[key]
                    result_lines.append(f'{indent}#[cfg(feature = "{feature}")]')

        result_lines.append(line)

    return '\n'.join(result_lines)


def add_version_attributes(token, func_cfg_map):
    """Add #[cfg(feature = "api-vX_Y")] attributes to extern functions."""

    lines = token.split('\n')
    result_lines = []
    processed_funcs = set()

    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check if this line is a doc comment followed by a function
        if stripped.startswith('#[doc ='):
            # Collect consecutive doc lines
            doc_lines = [i]
            j = i + 1
            while j < len(lines) and lines[j].strip().startswith('#[doc ='):
                doc_lines.append(j)
                j += 1

            if j < len(lines):
                func_line = lines[j]
                func_match = re.match(r'\s*pub fn (\w+)\s*\(', func_line)
                if func_match:
                    func_name = func_match.group(1)
                    if func_name in func_cfg_map and func_name not in processed_funcs:
                        feature = func_cfg_map[func_name]
                        indent = len(line) - len(line.lstrip())
                        cfg_line = ' ' * indent + f'#[cfg(feature = "{feature}")]'
                        result_lines.append(cfg_line)
                        processed_funcs.add(func_name)

        # Also handle functions without doc comments
        func_match = re.match(r'^(\s*)pub fn (\w+)\s*\(', line)
        if func_match and not stripped.startswith('#[doc ='):
            indent_str = func_match.group(1)
            func_name = func_match.group(2)
            if func_name in func_cfg_map and func_name not in processed_funcs:
                feature = func_cfg_map[func_name]
                cfg_line = f'{indent_str}#[cfg(feature = "{feature}")]'
                result_lines.append(cfg_line)
                processed_funcs.add(func_name)

        result_lines.append(line)
        i += 1

    return '\n'.join(result_lines)


def generate_static_ffi(token, func_cfg_map):
    """Generate ffi_xc_static.rs content from bindgen output."""
    token = token.replace("::core::ffi::", "")
    token = remove_xc_version_constants(token)
    token = add_struct_version_attributes(token, version_struct_changes)
    token = add_version_attributes(token, func_cfg_map)

    feature_docs = """//! FFI bindings for libxc (xc.h).
//!
//! # API Version Features
//!
//! This crate provides versioned FFI bindings through cargo features:
//!
//! - `api-v6_2`: Base API (libxc 6.2.2)
//! - `api-v7_0`: Extends api-v6_2, adds ext_params, mgga_new, fhc_enforcement (libxc 7.0.0)
//! - `api-v7_1`: Extends api-v7_0, adds default flags, func_init_flags (libxc 7.1.0)
//!
//! Features are cumulative: enabling `api-v7_1` also enables all functions from
//! earlier versions (api-v6_2, api-v7_0).
//!
//! Note: Constants from `xc_version.h` (XC_VERSION, XC_MAJOR_VERSION, etc.)
//! are excluded from these bindings since they are version-specific.

#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use core::ffi::{c_char, c_int, c_longlong, c_void};
"""

    return feature_docs + "\n\n" + token


# ## Dynamic loading generation functions

def dyload_parse_file(token):
    """Parse the FFI file and extract the extern block."""
    parser = Parser(Language(tree_sitter_rust.language()))
    token_transformed = token.replace("unsafe extern \"C\"", "extern \"C\"")
    parsed = parser.parse(bytes(token_transformed, "utf8"))
    parsed_ffi = []
    for node in parsed.root_node.children:
        if node.type == "foreign_mod_item":
            parsed_ffi.append(node)
    return parsed, parsed_ffi


def dyload_remove_extern(parsed, node_extern):
    """Remove the extern block from the parsed file."""
    return parsed.root_node.text.decode("utf8").replace(node_extern.text.decode("utf8"), "")


def dyload_get_ffi_fn(node):
    """Get all function signatures from an extern block."""
    assert node.type == "foreign_mod_item"
    return [n for n in node.children[-1].children if n.type == "function_signature_item"]


def dyload_fn_split(node):
    """Split a function signature into its components."""
    assert node.type == "function_signature_item"
    keys = ["visibility_modifier", "identifier", "parameters", "return_type"]
    result = {key: None for key in keys}
    for (idx, child) in enumerate(node.children):
        if child.type == "->":
            result["return_type"] = node.children[idx + 1]
        elif child.type in keys:
            result[child.type] = child
    assert result["identifier"] is not None
    assert result["parameters"] is not None
    return result


def normalize_ffi_types(text):
    """Replace ::core::ffi::c_int and similar with short names."""
    text = text.replace("::core::ffi::c_int", "c_int")
    text = text.replace("::core::ffi::c_char", "c_char")
    text = text.replace("::core::ffi::c_void", "c_void")
    text = text.replace("::core::ffi::", "")
    return text


def dyload_main(token):
    """
    Generate dynamic loading files from bindgen output.

    Returns a dict with keys:
    - ffi_base: base types and imports
    - dyload_struct: struct with Option<extern fn> fields
    - dyload_initializer: DyLoadLib::new implementation
    - dyload_compatible: wrapper functions calling through dyload_lib()
    """
    parsed, parsed_ffi = dyload_parse_file(token)

    token_ffi_base = token

    nodes_fn = []
    for node_extern in parsed_ffi:
        nodes_fn.extend(dyload_get_ffi_fn(node_extern))

    token_dyload_struct = ""
    token_dyload_initializer = ""
    token_dyload_compatible = ""

    for node_fn in nodes_fn:
        dict_fn = dyload_fn_split(node_fn)

        visibility_modifier = dict_fn["visibility_modifier"].text.decode("utf8") if dict_fn["visibility_modifier"] else "pub"
        identifier = dict_fn["identifier"].text.decode("utf8")

        return_type_string = ""
        if dict_fn["return_type"] is not None:
            return_type_string = " -> " + normalize_ffi_types(dict_fn["return_type"].text.decode("utf8"))

        nodes_para = [n for n in dict_fn["parameters"].children if n.type == "parameter"]
        parameters = "(" + ", ".join([normalize_ffi_types(n.text.decode("utf8")) for n in nodes_para]) + ")"
        parameters_called = ", ".join([n.children[0].text.decode("utf8") for n in nodes_para])

        part_dyload_struct = f"""
            {visibility_modifier} {identifier}: Option<unsafe extern "C" fn{parameters}{return_type_string}>,
        """.strip()

        part_dyload_initializer = f"""
            {identifier}: get_symbol(&libs, b"{identifier}\\0").map(|sym| *sym),
        """.strip()

        part_dyload_compatible = f"""
            {visibility_modifier} unsafe fn {identifier}{parameters}{return_type_string} {{
                dyload_lib().{identifier}.unwrap()({parameters_called})
            }}
        """.strip()

        token_dyload_struct += part_dyload_struct + "\n"
        token_dyload_initializer += part_dyload_initializer + "\n"
        token_dyload_compatible += part_dyload_compatible + "\n\n"

    for node_extern in parsed_ffi:
        token_ffi_base = dyload_remove_extern(parsed, node_extern)

    # Remove the import line since ffi_base re-exports types
    import_patterns = [
        r'use core::ffi::\{c_char, c_int, c_void\};',
        r'use core::ffi::\{c_char, c_int\};',
        r'use core::ffi::\{c_int, c_void\};',
        r'use core::ffi::\{c_char, c_void\};',
    ]
    for pattern in import_patterns:
        token_ffi_base = re.sub(pattern, '', token_ffi_base)

    # Remove xc_version.h constants from ffi_base too
    token_ffi_base = remove_xc_version_constants(token_ffi_base)

    # Add version-gated cfg attributes to struct fields
    token_ffi_base = add_struct_version_attributes(token_ffi_base, version_struct_changes)

    output_ffi_base = f"""//! Base types and imports for FFI.
//!
//! This file is generated automatically.

#![allow(non_camel_case_types)]

{token_ffi_base}
    """

    output_dyload_struct = f"""//! Library struct definition for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime. Runtime panic occurs if a function
//! is not found in the loaded library.

use super::*;
use core::ffi::{{c_char, c_int}};

pub struct DyLoadLib {{
    pub __libraries: Vec<libloading::Library>,
    pub __libraries_path: Vec<String>,
    pub __error: Option<String>,
{token_dyload_struct}
}}
    """

    output_dyload_initializer = f"""//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically.

use super::*;
use libloading::{{Library, Symbol}};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {{
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}}

impl DyLoadLib {{
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {{
        let mut result = DyLoadLib {{
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
{token_dyload_initializer}
        }};
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }}
}}
    """

    output_dyload_compatible = f"""//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically.
//!
//! Note: For dynamic loading, API version features are ignored.
//! All functions are available at runtime.

use super::*;
use core::ffi::{{c_char, c_int}};

{token_dyload_compatible}
    """

    return {
        "ffi_base": output_ffi_base,
        "dyload_struct": output_dyload_struct,
        "dyload_initializer": output_dyload_initializer,
        "dyload_compatible": output_dyload_compatible,
    }


DYLOAD_MOD_TEMPLATE = """//! FFI module for libxc (dynamic loading).
//!
//! This module provides dynamic loading support.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub const MOD_NAME: &str = module_path!();
pub const LIB_NAME: &str = "XC";
pub const LIB_NAME_SHOW: &str = "libxc";
pub const LIB_NAME_LINK: &str = "xc";

#[cfg(feature = "dynamic_loading")]
mod dynamic_loading_specific {
    use super::*;
    use libloading::Library;
    use std::fmt::Debug;
    use std::sync::OnceLock;

    use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};

    /// Detect Python interpreter path and return the corresponding lib directory.
    /// Uses OnceLock pattern for lazy initialization.
    static PYTHON_LIB_PATH: OnceLock<Option<String>> = OnceLock::new();

    fn detect_python_lib_path() -> Option<String> {
        PYTHON_LIB_PATH.get_or_init(|| {
            // 1. Check explicit environment variable first
            if let Ok(python_path) = std::env::var("LIBXC_PYTHON_PATH") {
                if let Some(lib_path) = extract_lib_from_python_bin(&python_path) {
                    return Some(lib_path);
                }
            }

            // 2. Try to find python in PATH
            if let Ok(paths) = std::env::var("PATH") {
                for path in paths.split(":") {
                    for python_name in ["python3", "python"] {
                        let python_bin = format!("{path}/{python_name}");
                        if std::path::Path::new(&python_bin).exists() {
                            if let Some(lib_path) = extract_lib_from_python_bin(&python_bin) {
                                return Some(lib_path);
                            }
                        }
                    }
                }
            }

            None
        }).clone()
    }

    fn extract_lib_from_python_bin(python_bin: &str) -> Option<String> {
        // If python is at /path/to/bin/python, library should be at /path/to/lib/
        let bin_path = std::path::Path::new(python_bin);
        if let Some(parent) = bin_path.parent() {
            if let Some(base) = parent.parent() {
                let lib_path = base.join("lib");
                if lib_path.exists() {
                    return Some(lib_path.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    fn get_lib_candidates() -> Vec<String> {
        let mut candidates = vec![];

        // User-defined candidates via environment variables
        for env_var in [format!("LIBXC_DYLOAD_{LIB_NAME}").as_str(), "LIBXC_DYLOAD"] {
            if let Ok(path) = std::env::var(env_var) {
                candidates.extend(path.split(":").map(|s| s.to_string()));
            }
        }

        // Also check DFTD4_DYLOAD for backward compatibility (shared env convention)
        if let Ok(path) = std::env::var("DFTD4_DYLOAD") {
            // Only use if it looks like a libxc path
            for p in path.split(":") {
                if p.contains("libxc") {
                    candidates.push(p.to_string());
                }
            }
        }

        // LD_LIBRARY_PATH style discovery
        for env_var in ["LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"] {
            if let Ok(paths) = std::env::var(env_var) {
                for path in paths.split(":") {
                    candidates.push(format!("{path}/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"));
                }
            }
        }

        // Python interpreter path discovery (cached)
        if let Some(lib_path) = detect_python_lib_path() {
            candidates.push(format!("{lib_path}/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"));
        }

        // Standard system candidates
        candidates.extend(vec![
            format!("{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("{DLL_PREFIX}xc{DLL_SUFFIX}"),
            format!("/usr/lib/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("/usr/local/lib/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
            format!("/lib/{DLL_PREFIX}{LIB_NAME_LINK}{DLL_SUFFIX}"),
        ]);
        candidates
    }

    fn check_lib_loaded(lib: &DyLoadLib) -> bool {
        lib.xc_version.is_some()
    }

    fn panic_no_lib_found<S: Debug>(candidates: &[S], err_msg: &str) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Unable to dynamically load the {LIB_NAME_SHOW} (`{LIB_NAME_LINK}`) shared library.
Candidates: {candidates:#?}

Please check:
- If dynamic-loading is not desired, disable the `dynamic_loading` feature in Cargo.toml.
- Use environment variable `LIBXC_DYLOAD_{LIB_NAME}` or `LIBXC_DYLOAD` to specify the library path.
- If `lib{LIB_NAME_LINK}.so` is installed on your system.
- If `LD_LIBRARY_PATH` is set correctly.
- Python interpreter path discovery: if Python is at `/path/bin/python`,
  the library is expected at `/path/lib/libxc.so`.

Error message(s):
{err_msg}
"#
        )
    }

    fn panic_condition_not_met<S: Debug>(candidates: &[S]) -> ! {
        panic!(
            r#"
This happens in module `{MOD_NAME}`.
Library loaded but condition not met: `xc_version` not found.
Found libraries: {candidates:#?}

Please check that the loaded library is a valid libxc library.
"#
        )
    }

    pub unsafe fn dyload_lib() -> &'static DyLoadLib {
        static LIB: OnceLock<DyLoadLib> = OnceLock::new();

        LIB.get_or_init(|| {
            let candidates = get_lib_candidates();
            let (mut libraries, mut libraries_path) = (vec![], vec![]);
            let mut err_msg = String::new();
            for candidate in &candidates {
                match Library::new(candidate) {
                    Ok(l) => {
                        libraries.push(l);
                        libraries_path.push(candidate.to_string());
                    },
                    Err(e) => err_msg.push_str(&format!("Failed to load `{candidate}`: {e}\n")),
                }
            }
            let lib = DyLoadLib::new(libraries, libraries_path);
            if lib.__libraries.is_empty() {
                panic_no_lib_found(&candidates, &err_msg);
            }
            if !check_lib_loaded(&lib) {
                panic_condition_not_met(&lib.__libraries_path);
            }
            lib
        })
    }
}

#[cfg(feature = "dynamic_loading")]
pub use dynamic_loading_specific::*;

/* #region general configuration */

pub(crate) mod ffi_base;
pub use ffi_base::*;

#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_compatible;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_initializer;
#[cfg(feature = "dynamic_loading")]
pub(crate) mod dyload_struct;

#[cfg(feature = "dynamic_loading")]
pub use dyload_compatible::*;
#[cfg(feature = "dynamic_loading")]
pub use dyload_struct::*;

/* #endregion */
"""


# ## Main execution

def main():
    # Ensure output directories exist
    os.makedirs(path_temp, exist_ok=True)
    os.makedirs(f"{path_out}/ffi_xc_dynamic", exist_ok=True)

    # Copy headers to temp, fixing the include for bindgen
    shutil.rmtree(path_temp, ignore_errors=True)
    shutil.copytree(path_header, path_temp)

    # Fix: change <xc_version.h> to "xc_version.h" for bindgen
    xc_h_path = f"{path_temp}/xc.h"
    with open(xc_h_path, "r") as f:
        content = f.read()
    content = content.replace('#include <xc_version.h>', '#include "xc_version.h"')
    with open(xc_h_path, "w") as f:
        f.write(content)

    os.chdir(path_temp)

    # Run bindgen
    subprocess.run([
        "bindgen",
        "xc.h", "-o", "ffi.rs",
        "--no-layout-tests",
        "--use-core",
        "--merge-extern-blocks",
    ], check=True)

    # Read bindgen output
    with open("ffi.rs", "r") as f:
        bindgen_output = f.read()

    # Parse version information from versioning_xc.md
    versioning_path = f"{path_header}/versioning_xc.md"
    version_map = parse_versioning_md(versioning_path)
    func_cfg_map = build_func_version_map(version_map)

    # Generate static FFI (ffi_xc_static.rs)
    static_ffi = generate_static_ffi(bindgen_output, func_cfg_map)
    with open(f"{path_out}/ffi_xc_static.rs", "w") as f:
        f.write(static_ffi)

    # Generate dynamic loading files (ffi_xc_dynamic/)
    dyload_files = dyload_main(bindgen_output)

    with open(f"{path_out}/ffi_xc_dynamic/ffi_base.rs", "w") as f:
        f.write(dyload_files["ffi_base"])

    with open(f"{path_out}/ffi_xc_dynamic/dyload_struct.rs", "w") as f:
        f.write(dyload_files["dyload_struct"])

    with open(f"{path_out}/ffi_xc_dynamic/dyload_initializer.rs", "w") as f:
        f.write(dyload_files["dyload_initializer"])

    with open(f"{path_out}/ffi_xc_dynamic/dyload_compatible.rs", "w") as f:
        f.write(dyload_files["dyload_compatible"])

    with open(f"{path_out}/ffi_xc_dynamic/mod.rs", "w") as f:
        f.write(DYLOAD_MOD_TEMPLATE)

    # Run cargo fmt
    os.chdir(f"{path_out}/..")
    subprocess.run(["cargo", "fmt"])

    print(f"Generated:")
    print(f"  - {path_out}/ffi_xc_static.rs")
    print(f"  - {path_out}/ffi_xc_dynamic/")


if __name__ == "__main__":
    main()
