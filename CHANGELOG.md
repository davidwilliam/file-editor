# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.0] – 2025-04-29

### Added
- Opt-in `regex` feature; all pattern methods now accept `&Regex` behind `features = ["regex"]`.  
- New `Pattern<'a>` abstraction, zero-dependency by default.  
- Updated docs and examples to show `regex` usage.

### Changed
- Made `Pattern` enum public to allow ergonomic `impl Into<Pattern>` bounds.

## [0.1.0] – 2025-04-27

### Added
- Initial `Editor` API:  
  - File ops: `create`, `open`, `rename`, `save`  
  - Content ops: `prepend`, `append`, `insert_before`, `insert_after`, `replace_marker`  
  - Search & transform: `find_lines`, `erase`, `replace`, `mask`  
- Full test coverage, examples, CI, Clippy, formatting.