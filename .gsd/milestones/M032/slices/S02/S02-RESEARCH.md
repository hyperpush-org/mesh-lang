# S02 Research: Cross-module and inferred-export blocker retirement

## Summary

S02 should target **generic function lowering/codegen**, not import resolution folklore.

What the investigation proved:

- The S01 blocker is real, but the **module boundary is only where it becomes loud**. A minimal imported `pub fn identity(x) do x end` still fails on the real CLI path with LLVM verifier output:
  - `Call parameter type does not match function signature!`
  - `{}  %call = call {} @identity(i64 7)`
- The same unconstrained function is also broken **inside one module**:
  - single-file `identity(7)` builds, then prints `0`
  - single-file `identity("poly")` builds, then aborts with a null-pointer panic in `mesh-rt` string printing
- The typechecker/export side already does the right thing for cross-module schemes:
  - exports are normalized with `Scheme::normalize_from_ty(...)`
  - imported functions are re-instantiated from those schemes at call sites
- The loss happens during lowering/codegen:
  - `compiler/mesh-codegen/src/mir/types.rs` still maps unresolved `Ty::Var` to `MirType::Unit`
  - `lower_fn_def(...)` in `compiler/mesh-codegen/src/mir/lower.rs` has a narrow recovery path for **parameter** types only, based on same-module usage-site function types
  - that recovery does **not** recover return types and does **not** see importer-side usage for exported functions
- `e2e_cross_module_polymorphic` and `e2e_cross_module_service` still pass. That means S02 should stay scoped to **unconstrained inferred exports / generic fn lowering**, not “all cross-module behavior.”
- On the mesher side, the live S02 surface is mostly **comment truth**, not a blocked product path:
  - `mesher/services/writer.mpl` already imports `Storage.Writer.insert_event`
  - `mesher/storage/writer.mpl:3-5` is stale about “service definition live in main.mpl” even before the compiler fix
  - the honest S02 mesher edit is likely surgical comment cleanup unless execution finds a natural product-neutral dogfood extraction

## Recommendation

Take the slice in three steps:

1. **Freeze the real root cause in tests before changing compiler code.**
   - Keep the existing S01 failure proof as the historical repro.
   - Add passing-target coverage for the repaired behavior:
     - imported unconstrained identity with one concrete call
     - imported unconstrained identity with two concrete call types
     - local single-file unconstrained identity returning the right value
2. **Fix lowering, not the verifier symptom.**
   - The smallest honest seam is the driver + MIR lowering boundary:
     - after all modules are type-checked, aggregate concrete function-usage types
     - thread that usage map into lowering
     - recover both unresolved parameter types **and** unresolved return types for generic fn defs
3. **Keep mesher dogfood minimal and truthful.**
   - Rewrite `mesher/storage/writer.mpl:3-5` around current reality.
   - Leave the raw-SQL boundary comments intact.
   - Do not invent synthetic mesher product code just to say the repaired path was “used.” If a real product-neutral extraction does not present itself naturally, comment surgery plus existing cross-module imports are the honest low-risk dogfood surface.

This recommendation follows the loaded skills:

- `debug-like-expert`: **verify, don’t assume**; do not patch the verifier error without explaining why the definition ABI became wrong.
- `rust-best-practices`: prefer the **smallest explicit data-flow change** over broad new generic infrastructure when the architecture already exposes a narrow seam.
- `llvm`: inspect the emitted IR when the symptom is ABI-level mismatch instead of guessing from the Rust alone.

## Requirements Targeted

- **R013** — this slice owns the “real blocker fixed in Mesh and then used in mesher” work
- **R011** — the blocker is real backend dogfood pressure, not speculative language work
- **R035** — the `mesher/storage/writer.mpl` limitation wording must become current and precise
- **Supports R010** — the repo’s public truth about Mesh capability should match the real CLI path

## Skills Discovered

- **Loaded:** `debug-like-expert`
  - Applied: verify exact failure surfaces first; no speculative “fixes” during research
- **Loaded:** `rust-best-practices`
  - Applied: look for the smallest explicit ownership/data-flow seam instead of proposing a large compiler redesign
- **Installed during this slice:** `llvm`
  - Command: `npx skills add mohitmishra786/low-level-dev-skills@llvm -g -y`
  - Why it matters here: the blocker is an LLVM function-signature mismatch, and IR inspection exposed the actual ABI drift

## Implementation Landscape

### A. The real bug is broader than cross-module import, but S02 can still stay scoped

**Neighboring green controls**

These already pass and should remain green during S02:

```bash
cargo test -p meshc --test e2e e2e_cross_module_polymorphic -- --nocapture
cargo test -p meshc --test e2e e2e_cross_module_service -- --nocapture
```

Observed during research:

- `e2e_cross_module_polymorphic ... ok`
- `e2e_cross_module_service ... ok`

Why that matters:

- Cross-module support in general is not broken.
- “Inferred export” is also not globally broken.
- The failing family is narrower: **exported functions whose own definition stays unconstrained enough that lowering still sees `Ty::Var`**.

### B. Minimal repros and exact observed symptoms

#### 1. Cross-module minimal repro can be reduced to one call

S01’s retained fixture uses two call sites, but one is enough.

Research control:

```mesh
# utils.mpl
pub fn identity(x) do
  x
end
```

```mesh
# main.mpl
from Utils import identity

fn main() do
  println("#{identity(7)}")
end
```

Observed CLI result:

```text
error: LLVM module verification failed: "Call parameter type does not match function signature!\ni64 7\n {}  %call = call {} @identity(i64 7)\n"
```

The existing durable repo proof remains:

```bash
cargo test -p meshc --test e2e e2e_m032_limit_xmod_identity -- --nocapture
cargo run -q -p meshc -- build .tmp/m032-s01/xmod_identity
```

#### 2. Local single-file control proves the root cause is not import resolution alone

Research controls:

```mesh
fn identity(x) do
  x
end

fn main() do
  println("#{identity(7)}")
end
```

Observed result:

- build succeeds
- binary prints `0`

String control:

```mesh
fn identity(x) do
  x
end

fn main() do
  println(identity("poly"))
end
```

Observed result:

- build succeeds
- binary aborts with a null-pointer panic in `compiler/mesh-rt/src/string.rs`

Planning consequence:

- S02 should not stop at “make imported identity compile.”
- The real repair needs to cover the same underlying lowering bug locally too, otherwise the slice just converts one honest compile failure into silent runtime corruption.

### C. LLVM IR shows the bad ABI directly

Using `meshc build --emit-llvm` on the local single-file control produced:

```llvm
define {} @Main__identity(i64 %0) {
entry:
  %x = alloca i64, align 8
  store i64 %0, ptr %x, align 8
  %x1 = load i64, ptr %x, align 8
  ret {} zeroinitializer
}

define {} @mesh_main() {
entry:
  %call = call {} @Main__identity(i64 7)
  call void @mesh_reduction_check()
  %call1 = call ptr @mesh_int_to_string(i64 0)
  call void @mesh_println(ptr %call1)
  ret {} zeroinitializer
}
```

That is the cleanest root-cause evidence gathered in this slice:

- parameter recovered to `i64`
- return type still lowered to `{}` / unit
- body loads `%x1` then throws it away
- caller turns the “result” into `0`

The artifact from this session is at:

- `.tmp/m032-s02-localemit.JBh6S4/m032-s02-localemit.ll`

This is disposable session evidence, not a durable verification surface. S02 should encode the same truth in tests.

### D. Where the compiler currently gets it right

#### Typechecker/export side looks correct

- `compiler/mesh-typeck/src/lib.rs`
  - `collect_exports(...)` stores function exports with `Scheme::normalize_from_ty(...)`
- `compiler/mesh-typeck/src/infer.rs`
  - `from Module import name` clones that scheme into the importer env
  - call inference instantiates the scheme at each use site

Why this matters:

- S02 should not start by rewriting export collection or import lookup.
- The old “type variable scoping limitation” wording is partly stale. Type scheme normalization is already there.

### E. Where the compiler currently loses the type

#### 1. `Ty::Var` still becomes `Unit` in MIR

- `compiler/mesh-codegen/src/mir/types.rs`
  - `resolve_type(...)` maps `Ty::Var(_)` to `MirType::Unit`

That fallback is survivable only when some later recovery path repairs the missing type.

#### 2. `lower_fn_def(...)` recovers too little

- `compiler/mesh-codegen/src/mir/lower.rs`
  - `lower_fn_def(...)` recovers unresolved **parameter** types via `resolve_param_from_usage(...)`
  - return type still comes straight from `resolve_type(ret, ...)`
  - if the return is still `Ty::Var`, it stays `MirType::Unit`

This exactly matches the local `identity(7)` IR.

#### 3. The current usage map is narrower than the slice needs

- `compiler/mesh-codegen/src/mir/lower.rs`
  - `build_fn_value_usage_types(...)`
  - `resolve_param_from_usage(...)`

Important nuance:

- the name and comments talk about “functions passed as values”
- the implementation already scans current-module `NAME_REF`s broadly enough that direct local call sites helped recover `identity`’s parameter type
- but the map is limited to `self.user_fn_defs`, so **imported** functions never benefit from importer-side concrete usage

That makes it a good extension point, not dead code.

### F. Driver seam: all required information already exists before lowering

- `compiler/meshc/src/main.rs`
  - all modules are parsed and type-checked first
  - only after that does the driver call `mesh_codegen::lower_to_mir_raw(...)` for each module

This is the most important architectural seam for S02.

The driver already has:

- every parsed module
- every `TypeckResult`
- the module graph
- compilation order

So the driver can, in principle, build a concrete usage map keyed by exporting module + function name **before** lowering begins.

That is likely the smallest honest fix surface.

## Natural Task Seams

### Seam 1 — regression surface first

**Files**

- `compiler/meshc/tests/e2e.rs`
- optionally new `.tmp/m032-s02/...` fixtures if execution wants durable external fixture directories

**What belongs here**

- keep or replace `e2e_m032_limit_xmod_identity`
- add success-path coverage for repaired imported unconstrained identity
- add local single-file unconstrained identity runtime coverage
- keep `e2e_cross_module_polymorphic` and `e2e_cross_module_service` as adjacency controls

**Why independent**

- the failing/proof surface is cleanly separable from the implementation
- it gives executor agents a hard stop against symptom-only fixes

### Seam 2 — driver + lowering implementation

**Files**

- `compiler/meshc/src/main.rs`
- `compiler/mesh-codegen/src/lib.rs`
- `compiler/mesh-codegen/src/mir/lower.rs`
- possibly `compiler/mesh-codegen/src/mir/types.rs` if fallback handling needs tightening

**What belongs here**

- aggregate concrete function-usage types after typecheck, before lowering
- thread them into `lower_to_mir_raw(...)` / `Lowerer`
- recover unresolved return types alongside parameter types
- make the imported-exported path and the local single-file path converge on the same repair

**Why this is the riskiest seam**

- this is where slice success or failure will be decided
- done badly, it can regress already-green module-system behavior

### Seam 3 — mesher truth cleanup

**Files**

- `mesher/storage/writer.mpl`
- maybe `mesher/services/writer.mpl` only if execution finds a real product-neutral dogfood move

**What belongs here**

- remove or rewrite the stale `main.mpl` / service-export implication at `mesher/storage/writer.mpl:3-5`
- leave the raw-SQL JSONB rationale intact
- do not mix in the S04 `from_json` wording cleanup beyond what S02 truly repairs

**Why this stays separate**

- the mesher change is low-risk and mostly truth-surface work
- it should not be entangled with the compiler iteration loop

### Seam 4 — verification script drift

**Files**

- `scripts/verify-m032-s01.sh`

**What belongs here**

- flip the `xmod_identity` step from expected failure to expected success once the fix lands
- keep the retained real-failure families (`nested_and`, route closures, timer/service cast) intact

**Why it matters**

- if this script is left unchanged, S02 can succeed locally while the durable replay surface still reports the old blocker as current truth

## Mesher Reality Check

The best current mesher evidence is more constrained than the roadmap wording might imply.

What is true today:

- `mesher/services/writer.mpl` already imports `Storage.Writer.insert_event`
- `mesher/main.mpl` imports `Services.Writer.StorageWriter`
- there is **not** an obvious real feature still stranded in `main.mpl` solely because of the `xmod_identity` bug

What that means for planning:

- the honest guaranteed mesher change is the mixed-truth comment cleanup in `mesher/storage/writer.mpl`
- if the planner wants stronger dogfood than a comment rewrite, it needs to find a **natural** extraction, not add a synthetic `identity` helper just to tick the requirement box
- I did not find that natural extraction during this investigation

## Risks and Constraints

- **Do not patch around LLVM verification.**
  - Skipping verification or coercing unresolved types to pointers would hide the exact failure that the local controls show as runtime corruption.
- **Do not widen the slice into “real generic monomorphization everywhere.”**
  - The architecture already has a narrower usage-recovery seam.
- **Do not treat `mesher/storage/writer.mpl` as a big code-move task by default.**
  - The product path already works; the comment is the live stale surface.
- **Do not forget S01 proof drift.**
  - `scripts/verify-m032-s01.sh:140-141` still encodes `xmod_identity` as an expected failure.

## Verification Plan

Minimum authoritative S02 verification should include:

```bash
cargo test -p meshc --test e2e e2e_cross_module_polymorphic -- --nocapture
cargo test -p meshc --test e2e e2e_cross_module_service -- --nocapture
cargo test -p meshc --test e2e m032_ -- --nocapture
bash scripts/verify-m032-s01.sh
cargo run -q -p meshc -- fmt --check mesher
cargo run -q -p meshc -- build mesher
```

And the repaired behavior should be proven by new success-path coverage for:

- imported unconstrained identity
- local unconstrained identity

Not just by removing the old expected-failure assertion.

## Planner Notes

- Treat `compiler/mesh-codegen/src/mir/lower.rs` as the center of gravity.
- Treat `compiler/meshc/src/main.rs` as the likely enabler for importer-side usage recovery.
- Treat `compiler/mesh-typeck` as mostly control/context, not the first edit target.
- Treat `mesher/storage/writer.mpl` as mixed truth:
  - stale service/main wording belongs to S02
  - stale `from_json` wording belongs with the S04 comment surgery family
  - raw SQL rationale remains real

## Resume Notes

- First execution task should be the regression-surface change in `compiler/meshc/tests/e2e.rs`; lock the repaired behavior before touching compiler code.
- The first implementation read should start in `compiler/mesh-codegen/src/mir/lower.rs` around `lower_fn_def(...)`, `build_fn_value_usage_types(...)`, and `resolve_param_from_usage(...)`, then widen only as needed into `compiler/meshc/src/main.rs` and `compiler/mesh-codegen/src/lib.rs`.
- After the compiler fix lands, update `scripts/verify-m032-s01.sh` so `xmod_identity` is no longer treated as an expected failure, then rerun the mesher build/fmt gates.
