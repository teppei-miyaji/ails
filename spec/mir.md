# AILS MIR Specification

## Role of MIR

MIR is the control-flow-oriented intermediate representation.

MIR is responsible for:
- basic blocks
- terminators
- explicit control-flow edges
- representation suitable for later validation and backend lowering

## Terminators currently include

- `Return`
- `Goto`
- `If`
- `Match`
- `Unreachable`

## Planned validation

A future MIR validation pass must check:
- terminator completeness
- reachable block structure
- return type consistency
- later, ownership/drop consistency
