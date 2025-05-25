/-
This is a test for markers inside code blocks within comments.

```lean
/-
## The type of positive natural numbers

We start by defining the type of positive natural numbers as a subtype of `Nat`.
-/

/-- The type of positive natural numbers is ... -/ --+
@[reducible] --#
def NatPos : Type :=
{n : Nat // 0 < n}
```

-/
