# Rust Style Guide

This document is a Rust coding style guide.

---

## 1. Low-Level Code

### Principles
- Minimize unsafe scope
- Wrap with safe APIs
- Raw pointers are allowed in low-level data structures

### Rules
- Limit unsafe blocks to the minimum necessary scope
- Write `// SAFETY:` comments for all unsafe blocks
- Wrap non-nullable pointers with `NonNull<T>`
- Validate immediately after raw pointer operations: null check, bounds check, cleanup on failure
- Use `*mut T`, `*const T` for self-referential structures (trees, graphs)
- Use safe code if it can replace unsafe code

### Decision Criteria: Raw Pointers vs Box/Rc/Arc
| Situation | Choice |
|-----------|--------|
| Self-referential structures (trees, linked lists) | `*mut T` + `NonNull` |
| Single ownership, heap allocation | `Box<T>` |
| Shared ownership, single thread | `Rc<T>` |
| Shared ownership, multi-thread | `Arc<T>` |
| FFI boundary | `*mut T` / `*const T` |

### Code Review Checklist
- [ ] Are unsafe blocks limited to the minimum necessary scope?
- [ ] Do all unsafe blocks have `// SAFETY:` comments?
- [ ] Are raw pointers used where `NonNull` could be used instead?
- [ ] Is there null/validity validation before raw pointer dereference?
- [ ] Is the safety rationale for `unsafe impl Send/Sync` documented?
- [ ] Is the unsafe code wrapped and exposed through a safe API?
- [ ] Are allocated resources cleaned up on failure paths?
- [ ] Is a raw pointer necessary instead of Box? (self-reference, FFI, etc.)

---

## 2. Error Handling

### Principles
- Propagate errors using the `?` operator
- Define specific error types
- Panic is a last resort

### Rules
- Default to error propagation: use the `?` operator
- Apply `#[derive(Error)]` from the `thiserror` crate to all error types
- Define error types as per-module enums
- Use `#[from]` when converting child errors to parent errors
- Error messages start with lowercase: `"out of memory"`, `"invalid argument"`
- Use explicit types when returning Result
- Define specific error types; do not use `Box<dyn Error>`
- Use enum variants instead of string errors (`String`, `&str`)

### Error Handling by Situation
| Situation | Approach |
|-----------|----------|
| General logic | `Result` + `?` propagation |
| Test code | `unwrap()` allowed |
| Internal invariant violation (bug) | `unreachable!` allowed |
| Required setup failure at program start | `panic!` allowed |

### Code Review Checklist
- [ ] Can `unwrap()`/`expect()` be replaced with `?` propagation?
- [ ] Do all error types derive `thiserror::Error`?
- [ ] Do error messages start with lowercase?
- [ ] Is `#[from]` properly configured for error conversion?
- [ ] Is panic usage within allowed scope?

---

## 3. Type System

### Principles
- Express state through types
- Catch errors at compile time

### Rules
- Express mutually exclusive states with enums
- Use enum variants instead of bool flags
- Use const or enum instead of magic numbers
- Simplify complex generic types with type aliases
- Define marker/token types as unit structs
- Use `#[cfg(feature = "...")]` for implementation branching

### Code Review Checklist
- [ ] Are mutually exclusive states expressed with enums?
- [ ] Are bool flags being used for state representation?
- [ ] Are magic numbers replaced with const or enum?
- [ ] Are type aliases applied to complex generic types?
- [ ] Is feature flag conditional compilation appropriate?

---

## 4. Trait Design

### Principles
- One trait handles one responsibility
- Express composite functionality through trait composition

### Rules
- Keep traits small with single responsibility
- Compose complex functionality through trait inheritance: `trait Stream: Readable + Writable {}`
- Use blanket implementations for common implementations: `impl<T: Readable + Writable> Stream for T {}`
- Only specify trait bounds that are actually used
- Use the Layer pattern (wrapper trait) when feature extension is needed

### Code Review Checklist
- [ ] Does the trait have a single responsibility?
- [ ] If the trait has too many methods, has splitting been considered?
- [ ] Do trait bounds only include what is actually used?
- [ ] Can blanket implementations reduce duplicate implementations?

---

## 5. Lifetimes

### Principles
- Elide lifetimes as much as possible
- Redesign with ownership when lifetimes become complex

### Rules
- Do not write explicit lifetimes where compiler elision rules apply
- Explicitly annotate lifetimes for references in structs
- Use `'static` for global state etc.
- If 3+ lifetime parameters are needed, redesign with ownership

### Code Review Checklist
- [ ] Are explicit lifetimes written where elision rules could apply?
- [ ] If 3+ lifetime parameters exist, has ownership-based redesign been considered?
- [ ] Is `'static` used only in global/thread-shared contexts?

---

## 6. Concurrency

### Principles
- Prefer lightweight synchronization: Atomic > Mutex
- Use `OnceLock` for global initialization

### Rules
- Use `OnceLock` for global initialization (instead of `lazy_static!`)
- Use `thread_local!` for per-thread state
- Use `Atomic*` for simple counters and flags
- Use `Mutex` for compound state changes

### Synchronization Primitive Selection Criteria
| Situation | Choice |
|-----------|--------|
| Simple value increment/decrement, flags | `AtomicUsize`, `AtomicBool` |
| Read-heavy, write-light | `RwLock` |
| Similar read/write frequency | `Mutex` |
| One-time global initialization | `OnceLock` |
| Per-thread independent state | `thread_local!` |

### Code Review Checklist
- [ ] Is `OnceLock` used instead of `lazy_static!`?
- [ ] Is the `Mutex` vs `RwLock` choice appropriate for the access pattern?
- [ ] Is `thread_local!` used for per-thread independent state?

---

## 7. Structs and Enums

### Principles
- Derive `Debug` for all types
- Choose field visibility according to purpose

### Rules
- Derive `Debug` for all types
- Mark FFI structs with `#[repr(C)]`
- Only protect fields requiring invariant validation with private + methods

### Code Review Checklist
- [ ] Is `Debug` derived for all types?
- [ ] Do FFI structs have `#[repr(C)]`?

---

## 8. Module Structure

### Principles
- Follow Rust 2018+ style module structure
- Explicitly re-export public APIs

### Rules
- Use `foo.rs` + `foo/bar.rs` structure for module files (instead of `mod.rs`)
- Re-export public APIs with `pub use`
- Define error types in an `error.rs` module
- Separate optional features with feature flags

### Example: Module Structure
```
src/
├── lib.rs
├── error.rs
├── stream.rs
└── stream/
    ├── writer.rs
    └── reader.rs
```

### Code Review Checklist
- [ ] Is the `filename.rs` pattern used instead of `mod.rs`?
- [ ] Are public APIs re-exported with `pub use`?
- [ ] Is feature flag conditional compilation appropriate?

---

## 9. Macros

### Principles
- Do not use macros if functions or generics can do the job
- Actively use standard library macros (`debug_assert!`, `unreachable!`, etc.)

### Rules
- Prefer functions/generics over custom macro definitions
- Use `debug_assert!` for internal invariant validation
- Use `unreachable!` for unreachable branches and provide a message
- Cases where macros are needed: variadic arguments, compile-time code generation, boilerplate removal

### Code Review Checklist
- [ ] Can the custom macro be replaced with functions/generics?
- [ ] Is `debug_assert!` used for internal invariant validation?
- [ ] Does `unreachable!` have a debuggable message?

---

## 10. Testing

### Principles
- Write unit tests in the same file; integration tests in `tests/` directory
- Test names should convey what is being verified
- Write only meaningful tests

### Rules
- Write unit tests in `#[cfg(test)] mod tests` blocks
- Write integration tests in the `tests/` directory
- Test names should include what is being verified
- Test edge cases (boundary values, empty input, overflow)
- Do not test trivial code (simple getters, trivial logic)
- Test behavior, not implementation details

### Code Review Checklist
- [ ] Can you tell what is being verified from the test name alone?
- [ ] Are edge cases tested?
- [ ] Is trivial code being tested unnecessarily?
- [ ] Are you testing behavior rather than implementation details?

---

## 11. Documentation

### Principles
- Comments explain "why" (code explains "what")
- Do not comment on self-explanatory code

### Rules
- Write `///` doc comments for public APIs
- In-code comments explain implementation rationale and non-obvious decisions
- Mark incomplete work with `// TODO: description`
- Keep comments in sync with code (delete outdated comments)

### Good Examples
```rust
/// Removes and returns an item from the queue.
/// Returns `None` if the queue is empty.
pub fn pop(&mut self) -> Option<T> { ... }

// Good: explains "why"
// Capacity must be a power of two to enable index calculation via bit operations
let capacity = len.next_power_of_two();
```

### Code Review Checklist
- [ ] Do public APIs have doc comments?
- [ ] Do comments explain "why"?
- [ ] Are comments consistent with the code?
- [ ] Do TODOs have sufficient context?

