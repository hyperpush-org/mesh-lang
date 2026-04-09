# GSD Overrides

User-issued overrides that supersede plan document content.

---
## Override: 2026-03-31T20:32:50.863Z

**Change:** Im still noticing code like this in the tiny cluster code

pub fn normalize_failover_delay_ms(delay :: Int) -> Int do
  if delay <= 0 do
    0
  else if delay > 5000 do
    5000
  else
    delay
  end
end

pub fn configured_failover_delay_ms() -> Int do
  normalize_failover_delay_ms(Env.get_int("TINY_CLUSTER_WORK_DELAY_MS", 0))
end

fn maybe_delay_work_execution() do
  let delay = configured_failover_delay_ms()
  if delay > 0 do
    Timer.sleep(delay)
    0
  else
    0
  end
end

This should NOT be required to be done by the user. It should also be part of the language
**Scope:** resolved
**Applied-at:** M046/S03/T03

---

## Override: 2026-04-01T16:34:24.487Z

**Change:** Why am i STLL seeing this syntax in @tiny-cluster/ and @cluster-proof/ esmpels??

I SHould NOT be seeing this
@cluster pub fn execute_declared_work(_request_key :: String, _attempt_id :: String) -> Int do
  1 + 1
end

it SHOULD be liek this instead
@cluster 
pub fn add() -> Int do
  1 + 1
end
**Scope:** resolved
**Applied-at:** M047/S05/T06

---
