# Multi-Language Context Pruning

The context pipeline parses Python, Rust, C++, and TypeScript with native
tree-sitter grammars. Source documents retain their original bytes, and each
parsed tree exposes a deterministic S-expression for diagnostics.

Language-specific structural queries target classes, interfaces, types, and
function declarations. The summarizer traverses matching syntax nodes, retains
their names and signatures in source order, and excludes implementation bodies.
Duplicate captures are removed before rendering.

Compression metrics report original and summarized byte counts plus the
resulting reduction ratio. These summaries provide dense structural context for
later injection without including function implementation payloads.
