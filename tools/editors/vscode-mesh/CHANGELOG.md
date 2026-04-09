# Changelog

All notable changes to the Mesh Language extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Repaired the shared VS Code/docs TextMate grammar so `#{...}` and `${...}` interpolation scopes now match in double- and triple-quoted strings, including nested-brace expressions.

## [0.3.0] - 2026-02-28

### Added

- `json` keyword highlighting for JSON object literal syntax (`json { key: value }`)
- Atom literal highlighting (`:name`, `:asc`, `:email`) as constants
- Regex literal highlighting (`~r/pattern/flags`) as strings
- `json` keyword in LSP completion suggestions
- Snippets for `type` alias (`type Alias = ExistingType`) and `json {}` block

### Changed

- Slot pipe operator (`|2>`, `|3>`) now highlights as a pipe operator
- `nil` now highlights as a constant value alongside `true` and `false`

## [0.2.0] - 2026-02-14

### Added

- Published to VS Code Marketplace and Open VSX
- Extension icon and marketplace metadata
- Completion suggestions with snippet support
- Signature help for function calls
- Document symbols (Outline view)

### Changed

- Enhanced TextMate grammar with comprehensive scope coverage

## [0.1.0] - 2026-02-07

### Added

- Initial release
- TextMate grammar for syntax highlighting
- LSP client connecting to meshc
- Hover information
- Go-to-definition
- Diagnostics
