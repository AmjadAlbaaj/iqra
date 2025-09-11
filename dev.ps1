# Dev helper for Windows PowerShell
param(
    [string]$task = "all"
)

switch ($task) {
    "fmt" { cargo fmt --all; break }
    "clippy" { cargo clippy --all-targets -- -D warnings; break }
    "test" { cargo test --all --verbose; break }
    default { cargo fmt --all; cargo clippy --all-targets -- -D warnings; cargo test --all --verbose }
}
