# Claude Code Instructions for Life Game WASM

## Project Overview
This is a high-performance Conway's Game of Life implementation using Rust/WebAssembly and TypeScript.

## Build Commands
- `npm run build` - Build the entire project (WASM + TypeScript + Vite)
- `npm run build:wasm` - Build only the WASM module
- `npm run build:ts` - Compile TypeScript
- `npm run dev` - Start development server

## Code Quality Commands
Always run these before considering a task complete:
- `npm run typecheck` - Check TypeScript types
- `npm run lint` - Run ESLint
- `cargo clippy -- -D warnings` - Run Rust linter
- `cargo fmt -- --check` - Check Rust formatting

## Test Commands
- `cargo test` - Run Rust tests
- `npm run test` - Run all tests (Rust + WASM)

## Project Structure
- `/src` - TypeScript frontend code
- `/src/*.rs` - Rust/WASM source files
- `/pkg` - Generated WASM output (do not edit)
- `/dist` - Built application (do not edit)

## Important Notes
- The WASM module name is `life_game_wasm` (not `rust_webpack_template`)
- Always use proper error handling in Rust (Result types)
- Include both English and Japanese comments in new code
- Follow existing code patterns and conventions