# Changelog

## v0.1.2 -- 2026-05-25

Enhancement:

- Enhance to layout handling
  - use IndexMap instead of Vec for storing components
  - make `components` field public
  - add method `iter_to_range`

- Docsrs features updated

- Add Send and Sync to `LibXCFunctional`.

## v0.1.1 -- 2026-05-15

Enhancement:

- Added function `set_ext_param_by_name`.

## v0.1.0 -- 2026-05-15

First version.

Supports libxc v6.2 (cpu), v7.0 (cpu, cuda), devel-branch at this stage (cpu and cuda).
