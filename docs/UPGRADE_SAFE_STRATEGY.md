# Upgrade-Safe Strategy

## Principle
Treat `octk` as an external, optional layer. Do **not** patch OpenClaw core internals.

## Why this is upgrade-safe
- OpenClaw updates can happen independently
- `octk` has its own release cycle (SemVer)
- Integration uses wrapper/config-level behavior only
- Fail-open behavior keeps OpenClaw functional if toolkit is unavailable

## Hard requirements
1. Optional by default (`auto` mode)
2. Fail-open fallback to raw output
3. No core monkey-patching
4. Versioned report schema
5. CI compatibility checks against current OpenClaw CLI behavior

## Operational recommendation
- Rollout in canary first
- Track `saved_percent` + quality incidents
- Auto-disable for `no_gain` cases

## Compatibility matrix policy
Maintain `docs/COMPATIBILITY.md` and update each release.
