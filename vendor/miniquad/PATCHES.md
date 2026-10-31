# Local Miniquad patch

Upstream: Miniquad 0.4.11, commit
`4f13d4a70caf6a8cbf0187a95461658e7812b566`, from the crates.io release.
Licenses: LICENSE-MIT and LICENSE-APACHE.

Changes:
- `src/native/apple/apple_util.rs`: check nil NSString receivers and null/empty
  UTF8 buffers before messaging or constructing a Rust slice.
- `src/native/macos.rs`: use the executable name for the Quit menu when Cocoa
  provides no localized application name.
- `src/lib.rs`: allow the backend-specific display helper to be unused on macOS.

This fixes the observed nil-name crash. It does not provide a graphics context
when the host denies access to OpenGL/Metal. Cargo uses this copy through
`[patch.crates-io]`; remove the override when an upstream release includes the fix.
