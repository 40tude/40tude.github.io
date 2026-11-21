module

public import Mathlib

/-!
# The set of even natural numbers is infinite

Statement: `{n : ℕ | Even n}.Infinite`, where `Even n` is Mathlib's
definition `∃ r, n = r + r`.

Two proofs are given, mirroring the informal ones:
1. By contradiction, using the maximum of a finite nonempty set.
2. Directly, via the injection `k ↦ 2 * k` from `ℕ` into the even numbers.
-/

public section

namespace EvenInfinite

/-- The set `P` of even natural numbers. -/
def P : Set ℕ := {n : ℕ | Even n}

/-- Proof 1: by contradiction, using the maximum of a finite nonempty set. -/
theorem P_infinite_by_max : P.Infinite := by
  -- Step 1: assume `P` is finite.
  intro hfin
  -- Step 2: `P` is nonempty, since `0` is even.
  have hne : hfin.toFinset.Nonempty := ⟨0, hfin.mem_toFinset.mpr Even.zero⟩
  -- Step 3 (the lemma): a finite nonempty subset of `ℕ` has a maximum `M`.
  have hMmem := Finset.max'_mem hfin.toFinset hne
  have hMeven : Even (hfin.toFinset.max' hne) := hfin.mem_toFinset.mp hMmem
  -- Write `M = k + k`.
  obtain ⟨k, hk⟩ := hMeven
  -- Step 4: `M + 2 = (k + 1) + (k + 1)` is even, hence belongs to `P`.
  have hN : hfin.toFinset.max' hne + 2 ∈ hfin.toFinset :=
    hfin.mem_toFinset.mpr ⟨k + 1, by omega⟩
  -- Step 5: since `M` is the maximum, `M + 2 ≤ M`, a contradiction.
  have hle := Finset.le_max' hfin.toFinset _ hN
  omega

/-- Proof 2: the map `k ↦ 2 * k` injects `ℕ` (an infinite type) into `P`. -/
theorem P_infinite_by_injection : P.Infinite :=
  Set.infinite_of_injective_forall_mem (f := fun k : ℕ => 2 * k)
    -- Injectivity: `2 * a = 2 * b → a = b`.
    (fun _ _ h => Nat.eq_of_mul_eq_mul_left (by norm_num) h)
    -- Every `2 * k` is even.
    (fun k => even_two_mul k)

end EvenInfinite
