# AILS Named Sum Type Layout

## 1. Status

Abstract canonical model only.
This document does not yet define a concrete backend ABI.

## 2. Abstract model

A named sum type value consists of:
- a discriminant identifying the active `case`
- zero or more payload fields associated with that `case`

## 3. Canonical semantic guarantees

Implementations must preserve:
- exact case identity
- declared field order within a case
- payload field type identity

## 4. What is intentionally not fixed yet

Not yet canonical:
- concrete byte layout
- discriminant size
- niche optimization
- field padding strategy
- backend ABI lowering

## 5. Reason

These details belong to the later backend contract.
At the language level, only discriminant+payload semantics are canonical.
