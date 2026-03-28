# v0.25 Suggested Test Commands

Use these after the repository is synced locally and build execution is available.

## Valid fixture checks

```powershell
cd D:\src\ails
cargo run -p ails-driver -- validate-mir-structure tests/mir/valid/minimal_function.ails
cargo run -p ails-driver -- validate-mir-structure tests/mir/valid/branch_function.ails
```

## MIR dump checks

```powershell
cd D:\src\ails
cargo run -p ails-driver -- mir tests/mir/valid/minimal_function.ails
cargo run -p ails-driver -- mir tests/mir/valid/branch_function.ails
```

## Next expected expansion

After executable confirmation:
- add Rust-side invalid MIR constructor tests
- assert structural error categories
- add golden output checks
```
