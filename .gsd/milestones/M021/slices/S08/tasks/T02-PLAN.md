# T02: 112-mesher-rewrite-search-dashboard-and-alerts 02

**Slice:** S08 — **Milestone:** M021

## Description

Rewrite alert system queries from Repo.query_raw/execute_raw to ORM APIs where expressible, and document ORM boundaries for complex alert queries.

Purpose: Eliminate Repo.query_raw from the alert system domain, completing REWR-05.
Output: ~7 alert queries rewritten to ORM; ~3 queries documented with ORM boundary rationale.

## Must-Haves

- [ ] "Simple alert queries (list_alert_rules, toggle_alert_rule, check_new_issue, get_event_alert_rules, should_fire_by_cooldown, get_threshold_rules, list_alerts) use ORM Query/Repo APIs instead of Repo.query_raw/execute_raw"
- [ ] "Complex alert queries (create_alert_rule, evaluate_threshold_rule, fire_alert) retain raw SQL with ORM boundary documentation"
- [ ] "All rewritten functions preserve identical signatures and behavior"

## Files

- `mesher/storage/queries.mpl`
