---
status: done
branch: feat/greensummit
repos: [attest, amont, amont-agent, relais, grh, greenharness-skills, homelab, decisions, activity-vision, agent-console, application-landscape, authelia-oidc-operator, aval, cluster-vision, customer-vision, ddns-updater-operator, duro-app, duro-design-system, duro-lexical-multi, duro-operator, governance-audit, grid, gtm-agent, homelab-preview-operator, kb-vision, social-planner, sre-agent, stealth-fetch, ticket-vision, trade-agents, vault-transit-unseal-operator, website-builder]
adrs: [ADR-0071, ADR-0037, ADR-0069]
plan-id: p-01M7JZ490DTGP50C29SRAYEMYB
---
# git-attest becomes greensummit

## Review panel

👉 **Decide:** none — approve if git-attest becomes greensummit in a new greenharness repo, by sed, with a hard cut across 32 repos.
📍 8 core repos + 24 swept · 6 phases · 3 releases · 2 steps for you · next: Phase 0 ADR. Panel: backend, architect, po, lang:rust, lang:python, platform, tui, unix (opus-5-5), codex (gpt).
**Changed by review:** new repo + archive (Actions don't follow transfers); signer image bump unconditional; relais and greengate release verifiers handled by hand, with dry runs.
📄 Full reviews: [2026-11-16-git-attest-becomes-greensummit.reviews.md](2026-11-16-git-attest-becomes-greensummit.reviews.md)
**Verdicts:** round 1: 8 approve-with-changes; Codex rework (4 applied); final bind 8 approve; 1 rejected (trail line); 9 carried; words 1743 → 2557 (×1.47, mostly fixes of wrong statements); remove 0, merge 0.

## Picture

```
          today: git-attest 1.10.0     after: greensummit 2.0.0
          ------------------------     ------------------------
 repo     fredericrous/attest          greenharness/greensummit
 crate    git-attest / git_attest      greensummit / greensummit
 binary   git-attest                   greensummit, no copy
 Action   fredericrous/attest@v1       greenharness/greensummit@v2
 sign     fredericrous/attest/sign     greenharness/greensummit/sign
 notes    greengate-attest(-inputs)    greensummit(-inputs)
          + amont-attest read          one set, no LEGACY read
 ssh ns   greengate-attest             greensummit
 header   greengate-attest-v2          greensummit-v2
 local    attest-local/...             greensummit-local/...
 spec     .github/attest-inputs        .github/greensummit-inputs
 env      ATTEST_*, GIT_ATTEST         GREENSUMMIT_*
 grh      part attest                  part greensummit (alias attest)
 clone    ~/Developer/Perso/attest     ~/Developer/Perso/greensummit
```

The rename is text, by sed, with one hard cut of the wire names: no alias,
no dual-read window. Every failure on the verify side is fail-open (the
Action and `verify.sh` print a warning and the suite runs), so the worst case
of any ordering is one CI run per repository that does not skip. Resolving an
Action is not fail-open, and GitHub does not redirect `uses:` after a
transfer or rename ("any workflow that uses that action will fail with
`repository not found`", GitHub docs). So greensummit is a NEW repository
`greenharness/greensummit` holding attest's full history, tags and notes
(`git push --mirror`), and `fredericrous/attest` is archived only after the
sweep: until then its `@v1` and `/sign@<sha>` keep resolving (the person,
2026-11-16, GitHub's own advice). One exception
is not fail-open: relais `release.yaml` runs `ATTEST_CHECK=1` and refuses a
release without a signature on all three platforms, so relais's `sign@` pin,
`release.yaml` verifier and `allowed_signers` change in one PR, and no relais
release is tagged before main has re-signed under `greensummit`.

## Context

Programme `decisions/docs/programmes/greensummit-rename` (charted
2026-11-16, decisions#214). The person asked: "rename git-attest to
greensummit and move to greenharness org. use sed we don't have users", and
chose at charting: greensummit (replacing greenskipass), 2.0.0 with Action
`@v2`, the crates.io `git-attest` crate left untouched, and the wire names
hard-cut with no window (asked twice, the second time naming ADR-0071's
ordered signed-notes switch). `greensummit` is free on crates.io and in the
`greenharness` org (checked 2026-11-16).

Found (2026-11-16, `git grep` of tracked files, history excluded): 31
repositories. attest 49 files; amont (greengate) 71 (lib users in
`crates/greengate-runtime/src/attest.rs`, `gate_stamp.rs`,
`crates/greengate-fleet/src/attest.rs`, `cluster/detached/{sign,collect}.rs`,
CI templates `templates/ci/{github,forgejo}/*.yaml`, tests); amont-agent
(greenbelay) 12 (`notes_names.rs`, `shipped.rs`, `push_preflight.rs`, corpus);
relais 16 (`allowed_signers`, `ci.yaml` Action + `sign@<sha>`,
`release.yaml` downloads git-attest 1.8.0, `ci-release-platforms.sh`
`GIT_ATTEST`); grh 3 (catalog part, README, tests); homelab 14 (6 Forgejo
workflows, `.forgejo/allowed_signers`, Dockerfile comments); 24 more repos:
the Action line in their workflows and an `allowed_signers` file.
Names live centrally in `attest/src/format.rs` (`Names`, `LEGACY`, `GREEN`,
`WRITE`, `LOCAL_PREFIX`) and `attest/sign/sign.sh`.

Concurrent: the greenbelay rename (ADR-0071) has open branches in
greenbelay, relais, amont (`feat/greenbelay-readers` touches `attest.rs`),
greenharness-skills and grh, merging in the next hours; it asked that nobody
`brew upgrade greenpiste` or run `grh install` before its machine switch.
This plan touches those five repositories only after their greenbelay PR
merged, on a fresh branch from `origin/main`.

## Goal

After this plan nothing live names git-attest or its old wire names: the
repository is `greenharness/greensummit`, crate, library, binary, Action, grh
part, env vars and notes all say greensummit, the fleet's CI verifies
greensummit notes and skips covered suites again, and git-attest is left
only in history.

## Non-goals

- Aliases: no `git-attest` binary copy, no fallback read of
  `greengate-attest` or `amont-attest` notes, no `@v1` moved forward.
- Re-signing or migrating old notes; the old notes refs are left on remotes
  (unreachable names, harmless; ADR records it).
- The crates.io `git-attest` crate: untouched (the person's choice).
- `greengate-provenance` / `amont-provenance` namespaces: greengate's, not
  this tool's.
- Role words: `attest-signer` (homelab service, namespace, Vault paths,
  `GREENGATE_SIGNER_NAMESPACE`), `greengate.attest`/`amont.attest` config
  keys, `attestations/` directories, greengate's `attest` subcommand, the
  `ATTEST_SIGNING_KEY` secret name, the verb "attest" in prose.
- History: `CHANGELOG.md` entries, `docs/plans/`, `docs/adr/` bodies,
  `docs/programmes/`, vendored `.adr/packs/*.pack` (refreshed by `aval add`,
  never sed).
- The other renames (relais, aval, adr-lint) and the greenbelay rename.

## Code review
- attest: backend, lang:rust, platform, unix, architect (tui left out: only the name in output changes; lang:python left out: no Python file)
- amont: backend, lang:rust, platform (unix, tui left out: no CLI change but a dependency; architect left out: attest's seat covers the format contract; lang:python: none touched)
- amont-agent: backend, lang:rust (platform, unix, tui, architect left out: one names table, tests and a CI line)
- relais: backend, platform, lang:python (lang:rust, unix, tui, architect left out: CI files, one script and its Python test; Rust only in comments)
- grh: backend, lang:rust (platform, unix, tui, architect left out: a catalog entry, README and tests)
- homelab: platform (backend and languages left out: workflow lines, allowed_signers, Dockerfile comments)
- decisions: none — markdown only
- greenharness-skills: none — markdown only
- the 24 workflow-only repos: none — one `uses:` line and one `allowed_signers` line each, checked by their own CI run

## Behaviour

- **The sed**, per repository, over tracked files minus history (the
  Non-goals list), each expression asserted to have matched where expected
  (count before, count after), run as
  `git grep -lz -E 'OLD' -- . ':(exclude)CHANGELOG.md' ':(exclude)docs/plans' ':(exclude)docs/adr' ':(exclude)docs/programmes' ':(exclude).adr/packs' ':(exclude)Cargo.lock' ':(exclude)tests/compat' | xargs -0 -r sed -i -E 's#OLD#NEW#g'`
  (NUL-separated; `#` delimiter, since rule 1 holds a `|`; OLD and NEW are each rule's pattern and replacement); `git ls-files -s | grep '^120000'` first,
  since GNU `sed -i` replaces a symlink with a file:
  1. `fredericrous/attest` → `greenharness/greensummit`, then
     `greenharness/greensummit(/sign)?@v1([^.0-9]|$)` → `greenharness/greensummit\1@v2\2`
     (README and `sign/action.yml` name `sign@v1`; greengate-fleet's `sign@v1.7.0` test pin stays)
     (`/sign@<sha>` pins are rewritten by hand to the v2.0.0 commit);
  2. `git-attest` → `greensummit`, `git_attest` → `greensummit`;
  3. `greengate-attest` → `greensummit` (covers header `-v2`, both refs,
     namespace, both fingerprint domains, `-inputs`);
  4. in `allowed_signers`: the `namespaces="…"` list loses `amont-attest`
     and `greengate-attest` and gains `greensummit` (provenance entries kept);
     per repo, the line count is unchanged and the
     `cluster-signer@x86_64-linux` line survives with `greensummit`;
  5. `attest-local/` → `greensummit-local/`; `.github/attest-inputs` and
     `.forgejo/attest-inputs` move by `git mv` to `greensummit-inputs`, and
     every reader of that path follows;
  6. attest repo only: `ATTEST_GIT_ATTEST` → `GREENSUMMIT_BIN` first (Makefile),
     then `ATTEST_SIGNING_KEY` swapped to a placeholder, `ATTEST_` →
     `GREENSUMMIT_` on the rest, the placeholder restored (sed has no
     negative match); relais: `GIT_ATTEST` → `GREENSUMMIT_BIN`;
     `GIT_ATTEST` joins both census regexes;
  7. attest repo only, the names it prints: `title=attest::` → `title=greensummit::`,
     the `attest: ` message prefix → `greensummit: ` (action.yml, sign/action.yml,
     fetch.sh, verify.sh, sign/sign.sh, src/main.rs, tests), the prefix strip in
     `action.yml` (`sed 's/^attest: //'`) in the same commit, and `name: attest`
     → `name: greensummit` in both action.yml files; checked by
     `git grep -nE "attest: |title=attest"` = 0 outside history.
  Then `git grep -nE 'git-attest|git_attest|fredericrous/attest|greengate-attest|amont-attest|attest-local/|attest-inputs'`
  outside history lists only the allowlist named in each PR (expected: 0,
  except greengate's provenance `amont-provenance` and its 3a-3 migration
  code if it still names `amont-attest` as a source to delete).
- **What the sed cannot do, by hand:**
  - `attest/src/format.rs`: `LEGACY` removed; the one set (renamed
    `SUMMIT`, `GREEN` gone) has `producer: "greengate"` unchanged; the read
    order is that one set; `verify.rs`/`notes.rs` drop their LEGACY paths and
    the `legacy_notes`/`legacy_inputs_notes` Action outputs; `SPEC.md`
    "Names" rewritten to one set; historical compat tests (`tests/compat/`, the Makefile targets that build
    archived tags as `git-attest`) keep their old names, since they model
    released code; only their expected verdict against 2.0.0 becomes "absent";
  - `attest/Cargo.toml`: `name = "greensummit"`, `[lib] name =
    "greensummit"`, `[[bin]] name = "greensummit"`, `version = "2.0.0"`,
    repository URL; release asset names follow (`greensummit-<ver>-<target>`);
  - greengate: `greensummit = "2"` in both crates, `[GREEN, LEGACY]`
    iterations over attest notes (attest.rs, detached/collect.rs) become the one set, and `gate_stamp.rs` drops `LEGACY` too (the person, 2026-11-16: old local stamps stop counting, gates rerun once), its read becoming the one set, so no `amont-attest` read remains in greengate, tests that built LEGACY blocks assert the
    block is now ignored; CHANGELOG Unreleased;
  - greenbelay `notes_names.rs`: the attestation names become one set,
    `refs/notes/greensummit`, header `greensummit-v2`; its provenance names
    (`greengate-provenance`, `amont-provenance`) and their readers stay; `shipped.rs` and push-preflight cases follow;
  - relais `release.yaml`: download `greensummit` 2.0.0 from
    `greenharness/greensummit` instead of git-attest 1.8.0: tag `v2.0.0`,
    asset `greensummit-2.0.0-x86_64-unknown-linux-musl`, the sha256 of that
    published asset, `GREENSUMMIT_BIN=$RUNNER_TEMP/$name/greensummit`;
  - grh `catalog.toml`: part `attest` becomes `greensummit`,
    `aliases = ["attest"]`, description keeps the role; README and tests;
  - homelab: the signer image embeds greengate (`services/attest-signer/Dockerfile` `GREENGATE_VERSION`/`GREENGATE_SHA256`): bump both to the Phase 2 release and roll the signer; check: a cluster-signed block with header `greensummit-v2` is filed by the laptop's collect.
- **Order** (fail-open makes any order safe; this one minimises unskipped CI):
  ADR → greensummit 2.0.0 (new repo, release, `v2`) → greengate
  release + `brew upgrade greengate` here (writer switches), with greenbelay's reader release installed in the same step once the greenbelay switch is done (`shipped.rs` reads only the attest ref it knows) → readers
  (greenbelay, relais, grh, skills, homelab) → fleet sweep (Action `@v2` +
  `allowed_signers`) → local checkout rename + census. Between the greengate
  install and a repo's sweep PR, that repo's CI runs its suites (old Action
  cannot read greensummit notes): accepted.

## Phases

- [x] Phase 0 — ADR: `decisions/docs/adr/0073-git-attest-becomes-greensummit.md` (decisions#218),
  `decisions/docs/adr/HEADS.md`, `decisions/.adr.yaml`: replaces ADR-0071's
  `harness.bundle-names` (greensummit names the part; the signed-notes
  namespaces are greensummit's), supersedes ADR-0037's `ci.attestation-implementation` (greensummit is the one implementation; the amont-* wire names are no longer frozen and are no longer read), and amends `harness.renames` (a tool with no
  outside users may hard-cut its signed-notes names with no window at the
  person's choice; old notes stop verifying and the suite runs). It also amends ADR-0064 `harness.tool-boundaries` (greensummit implements the format) and ADR-0069 (the cluster-signer line changes namespaces under the hard cut). Programme
  map tickets t02–t04 resolved: `decisions/docs/programmes/greensummit-rename/*.md`.
- [x] Phase 1a — greensummit 2.0.0 code (this branch; repo created, refs pushed, origin repointed, CI key rotated): `gh repo create greenharness/greensummit --public`,
  `git clone --mirror https://github.com/fredericrous/attest.git "$TMPDIR/attest-mirror.git"`, then
  `git -C "$TMPDIR/attest-mirror.git" push git@github.com:greenharness/greensummit.git 'refs/heads/*:refs/heads/*' 'refs/tags/*:refs/tags/*' 'refs/notes/*:refs/notes/*'`
  (never `--mirror` from the working checkout, which would publish local branches, `refs/remotes/*` and the unpublished `refs/notes/attest-local/*`; never `--mirror` from the clone, whose `refs/pull/*` GitHub rejects as hidden refs),
  then `git -C ~/Developer/Perso/attest remote set-url origin git@github.com:greenharness/greensummit.git` so this phase's PR, tag and release land on greensummit,
  its two secrets set there (Your steps), then sed + hand edits in `attest/src/format.rs`, `attest/src/verify.rs`,
  `attest/src/notes.rs`, `attest/src/main.rs`, `attest/src/cli_sign.rs`,
  `attest/src/tests.rs`, `attest/tests/names.rs`, `attest/src/fingerprint.rs`, `attest/src/produce.rs`, `attest/sign/sign.sh`, `attest/sign/action.yml`,
  `attest/action.yml`, `attest/fetch.sh`, `attest/verify.sh`,
  `attest/SPEC.md`, `attest/README.md`, `attest/Makefile`,
  `attest/Cargo.toml`, `attest/Cargo.lock`, `attest/.github/workflows/*.yaml`,
  `attest/.github/allowed_signers`, `attest/.github/greensummit-inputs`,
  `attest/tests/*.sh`, `attest/CHANGELOG.md`; release v2.0.0 by its
  `RELEASE` steps (tag-release skill), then `move-major.yaml -f tag=v2.0.0`.
- [x] Phase 1b — release v2.0.0 from `greenharness/greensummit` (needs `CARGO_REGISTRY_TOKEN` there), prepare-release bumps `RELEASE`, then `move-major.yaml -f tag=v2.0.0`.
- [x] Phase 2 — greengate writes greensummit (after `feat/greenbelay-readers`
  merged): `amont/crates/greengate-runtime/Cargo.toml`,
  `amont/crates/greengate-fleet/Cargo.toml`, `amont/Cargo.lock` (excluded from the sed, regenerated by `cargo update -p greensummit`),
  `amont/crates/greengate-runtime/src/attest.rs`,
  `amont/crates/greengate-runtime/src/gate_stamp.rs`,
  `amont/crates/greengate-runtime/src/attest_inputs.rs`,
  `amont/crates/greengate-fleet/src/attest.rs`,
  `amont/crates/greengate-runtime/src/cluster/detached/sign.rs`,
  `amont/.github/workflows/release.yaml` and `amont/scripts/ci-release-gates.sh`
  (greengate's own release verifier downloads git-attest 1.7.0: same hand
  edit as relais, v2.0.0 asset and sha256, its release gate proven on a dry
  run before the greengate tag),
  `amont/crates/greengate-runtime/src/cluster/detached/collect.rs`,
  `amont/crates/greengate/tests/it/*.rs`, `amont/templates/ci/github/*.yaml`,
  `amont/templates/ci/forgejo/*.yaml`, `amont/docs/ci.md`,
  `amont/docs/configuration.md`, `amont/.github/workflows/ci.yaml`,
  `amont/.github/allowed_signers`, `amont/CHANGELOG.md`; release greengate
  (tag-release), `brew upgrade greengate` on this Mac.
- [x] Phase 3 — readers, each after its greenbelay PR merged:
  `amont-agent/crates/greenbelay-guard/src/notes_names.rs` (path after the
  greenbelay rename), `amont-agent/crates/greenbelay-process/src/shipped.rs`,
  `amont-agent/crates/greenbelay-guard/src/rules/push_preflight.rs`,
  `amont-agent/.github/workflows/ci.yaml`, `amont-agent/.github/allowed_signers`;
  `relais/.github/workflows/ci.yaml`, `relais/.github/workflows/release.yaml`,
  `relais/.github/allowed_signers`, `relais/scripts/ci-release-platforms.sh`,
  `relais/scripts/test_ci_platform.py`; `grh/catalog.toml`, `grh/README.md`,
  `grh/tests/cli.rs`; greenharness-skills `parts/**/*.md`;
  `homelab/.forgejo/workflows/*.yaml`, `homelab/.forgejo/allowed_signers`,
  `homelab/services/forgejo-runner/Dockerfile`,
  `homelab/services/attest-signer/Dockerfile`. Releases only where a
  shipped binary changed (greenbelay, grh), following their own release
  skills.
- [x] Phase 4 — fleet sweep, one PR per repository, merged on green, in
  the repos listed in `repos:` after decisions (24 names; build the list from a `git grep` of `fredericrous/attest` and state the exact count in the first PR), in batches of 4-6: the Action line and
  `allowed_signers` (namespaces and the trailing key comment), the workflow comment naming `refs/notes/amont-attest`, e.g. `sre-agent/.forgejo/workflows/ci.yml`,
  `sre-agent/.forgejo/allowed_signers`,
  `duro-operator/.github/workflows/pr.yml`,
  `duro-operator/.github/allowed_signers`; any `attest-inputs` moved;
  `AGENTS.md` regenerated where it names the tool (`greengate agents-md`);
  `.adr/packs/decisions.pack` refreshed by `aval add`.
- [x] Phase 5 — this machine and census: `gh repo archive fredericrous/attest` after a README commit pointing to greensummit, pushed to `git@github.com:fredericrous/attest.git` by explicit URL (origin already points to greensummit since Phase 1); `git -C ~/Developer/Perso/attest remote -v` shows greensummit,
  `git -C ~/Developer/Perso/attest worktree list` shows the main checkout only
  (else `git -C ~/Developer/Perso/greensummit worktree repair` after the move),
  `mv ~/Developer/Perso/attest ~/Developer/Perso/greensummit` from another
  cwd (no session's cwd inside it), `cargo uninstall git-attest` if present;
  the census command (Verification 6) at 0; programme map t05–t07 closed:
  `decisions/docs/programmes/greensummit-rename/map.md`.

### 🧑 Your steps
- `gh secret set ATTEST_SIGNING_KEY --repo greenharness/greensummit` — secrets cannot be copied between repositories and the key's value is yours (skipped if the agent finds the key file named in attest's RELEASE notes)
- `gh secret set CARGO_REGISTRY_TOKEN --repo greenharness/greensummit` — only if the v2.0.0 release job's `cargo publish` is refused for a new crate name (the token may be scoped to `git-attest`); a crates.io token is yours to mint

## Verification

1. attest: `cargo test --locked`, `make check` (compat, help, platform
   tables) green in the worktree; `cargo run -- --version` prints
   `greensummit 2.0.0`; a note signed by the built binary lands under
   `refs/notes/greensummit` and `covered` reports its gates; a block with
   header `greengate-attest-v2` reads as absent, held by `tests/names.rs::a_note_under_an_old_name_is_absent`.
   Observed 2026-11-17: `cargo test --locked` green; `make check` 156 passed, 0 failed; `cargo run -- --version` → `greensummit 2.0.0`; census grep outside history lists only old-name fixtures (src/tests.rs, tests/names.rs, tests/lib.sh, tests/sign.sh, tests/vectors, tests/conformance.sh, tests/legacy.sh), the Makefile archived-tag targets and docs saying old notes are ignored.
2. New repo: `gh secret list --repo greenharness/greensummit` lists
   `ATTEST_SIGNING_KEY` and `CARGO_REGISTRY_TOKEN`.
   Release: crates.io `greensummit` 2.0.0 exists; release asset
   `greensummit-2.0.0-x86_64-unknown-linux-musl.tar.gz` downloads;
   `git ls-remote https://github.com/greenharness/greensummit refs/tags/v2`
   equals the v2.0.0 commit; `fredericrous/attest` still serves `@v1`
   (an unswept repo's CI resolves it) until Phase 5 archives it.
3. greengate: its suite green; after `brew upgrade greengate`, a push from a
   scratch branch in attest's worktree leaves `refs/notes/greensummit` on
   origin (`git ls-remote origin refs/notes/greensummit`), and no new
   `greengate-attest` note.
4. End to end: that PR's CI (Action `@v2`) prints `covered=` with the
   pre-push gates and skips them; one swept fleet repo (sre-agent, Forgejo)
   and one GitHub repo (relais) skip their covered suites on the first PR
   after their sweep; a relais release dry run on the first main commit
   after Phase 3 has its `platforms` job find the signature on all three
   platforms.
5. Each sweep PR's own CI run green (a run that did not skip counts as
   green); then, for every swept repo, the first push after merge prints a non-empty `covered=` (list per repo in the PR log; any empty one blocks Phase 5).
   Observed 2026-11-17: every swept repo plus relais has a non-empty `covered=` on its origin/main tree; the per-repo list and how each was observed are in the Decision log. No empty one is left, so nothing blocks Phase 5.
6. Census: over every repo and worktree under `~/Developer/Perso`,
   `~/.claude` and `git config --global --list`,
   `git grep -nE 'git-attest|git_attest|fredericrous/attest|greengate-attest|amont-attest|attest-local/|attest-inputs'`
   minus history = 0 (allowlist named); `command -v git-attest` fails.
   Observed 2026-11-17: 0 outside the allowlist on every repo's origin/main after homelab#1225 and agentic-platform#23 (`~/.local/state/gs-census/census.sh origin/main`, filtered by `allowlist.txt` there); `git config --global` 0; `~/.claude` config 0; `command -v git-attest` fails and `cargo install --list` has no git-attest. Worktrees: 23 carry old names, all HOLD (unpushed, open PR or uncommitted work). Details in the Decision log and decisions t07.

## Rollback

- After Phase 1, before Phase 2's install: nothing to revert; `fredericrous/attest`
  is untouched and serves `@v1` (1.10.0), so old workflows keep working.
- After Phase 2's install, before Phase 4: reinstall the previous greengate (and greenbelay) release and the previous signer image tag; `@v1` reads its notes again.
- After any Phase 4 sweep PR merged: roll forward only (a fix release of greengate or greensummit); reverting a sweep PR restores `@v1` and the old namespace for that repo, which works only until Phase 5 archives `fredericrous/attest` (an archived repo's actions still resolve).

## Decision log
- 2026-11-17 — Phase 5 done. attest#50 (README pointer, CI 14/14 green, merged) pushed to `fredericrous/attest` through a temporary `legacy` remote, since the push review cannot read a bare URL; the remote was removed before the move. `fredericrous/attest` archived; `git ls-remote https://github.com/fredericrous/attest refs/tags/v1` still answers d0deec2. `~/Developer/Perso/attest` moved to `greensummit` with one worktree, origin greensummit. The census found two live hits outside the allowlist: agentic-platform, the 25th repo, missed by Phase 4 because its local checkout was behind origin (agentic-platform#23, CI green, merged), and the runner Dockerfile's dead amont block (homelab#1225: the block deleted, plus the three hadolint fixes from b31b214d4 with no rule suppressed; the 54 apt pins re-checked against today's trixie indexes, all still newest). Allowlist added to the 11-17 entry below: greengate's kept names (its own decision log), greenbelay's "not read" docs and tests, homelab's `attest-signer` role word, decisions' `.adr.yaml` key description and `docs/principles/ci-attested.md` prose, amont's `adr/` folder (history). Also checked: four Forgejo repos with no local clone (shallow clones, 0 hits), GitHub code search (one allowlisted fixture). The `gate_stamp::READ` and verify.rs one-element read follow-ups are no longer waiting on the census.
- 2026-11-17 — Verification 5 observed; Phase 4 ticked. `greensummit: verified under greensummit` is printed only for a covering block, so it proves a non-empty `covered=` (the value goes to `$GITHUB_OUTPUT`, not the log). Three kinds of evidence:
  - **Post-merge main run's CI log** (7 GitHub repos): cluster-vision 31c86cac, authelia-oidc-operator 36e532b8, ddns-updater-operator 9ffa712f, homelab-preview-operator fb0d59b5, vault-transit-unseal-operator 001c395c, duro-app 8a7fc31a, duro-operator 4b649a64.
  - **Computed** with the released greensummit 2.0.0 (x86_64-apple-darwin, SHA256SUMS match) in a detached worktree of origin/main, using each workflow's `platform`/`anywhere` inputs (13 Forgejo repos). Calibrated first on 3 known CI outcomes (cluster-vision covered, greentopo empty, greenrack notes absent): 3 of 3 matched. Forgejo logs were unreadable from this Mac: kubectl and Python get "no route to host" while `/usr/bin/nc` connects (macOS Local Network privacy). Covered: sre-agent 241a017f, agent-console 18593e3d, customer-vision 6529195f, governance-audit 6322de5d, gtm-agent 3de76408, application-landscape 58cbeb67, grid 8fe07fc3, stealth-fetch bb6d7c94, website-builder eb8ab80c, duro-lexical-multi 6d43a7c9, kb-vision 3f31a2b2, social-planner 7aca6ae5, trade-agents 7c9c6edf.
  - **Computed after a probe land**: three repos whose real post-merge main run had an empty `covered=`. Each was fixed by a `greengate land` of main's commit to a scratch `chore/greensummit-probe` branch, deleted afterwards. activity-vision 9e7e9b3a had never had an attestation note under any name (`greengate.attest` was first set in the sweep). ticket-vision 395680f6: the sweep branch was cut one bot commit (`chart 0.8.2 [skip ci]`) behind main, so the merge tree was never signed. greenrack e71dffb3: the sweep land owed no gate (see the corrected entry below).
  - **Platform-pinned, verified by computation**: greentopo 47a62be3 pins the Linux runner's platform for `cargo test`, and this Mac signs x86_64-macos. Its CI `covered=` is empty, as it was on every 1.x run (1.10.0 printed `attest: verified under …`, and no pre-rename run carried it). With `--platform any` its note verifies under greensummit and covers six gates, so the rename broke nothing. relais b9642b64: after its main run 44158920750, CI-signed blocks cover `pre-push-cargo-test` on x86_64-linux and aarch64-macos. Its PR runs test only on aarch64-macos and cannot match the laptop's x86_64-macos note, the same as before the rename.
  - Follow-up, owner greengate: why greenrack's sweep land (receipt `1794926103331353000-64619`) pushed without an `attested` line. A missed signature only costs a CI skip, until the next land.
- 2026-11-17 — Phase 4 merged, not yet ticked: 24 of 24 repos merged green (the `git grep -l fredericrous/attest` list matched `repos:` exactly). Forgejo: sre-agent#90, activity-vision#20, agent-console#16, customer-vision#41, governance-audit#13, gtm-agent#47, application-landscape#345, grid#42, stealth-fetch#31, ticket-vision#32, website-builder#233, duro-lexical-multi#37, kb-vision#63, social-planner#80, trade-agents#422. GitHub: greentopo#63 (aval), cluster-vision#64, authelia-oidc-operator#23, ddns-updater-operator#23, homelab-preview-operator#46, vault-transit-unseal-operator#18, duro-app#166, duro-operator#26, greenrack#188 (duro-design-system). Each tree had an implementation-review approve; relais#491 renamed relais's own key comment. Open before the tick: Verification 5's non-empty `covered=` on each swept repo's first laptop push after merge, listed per repo; any empty one blocks Phase 5.
- 2026-11-17 — the sweep added a third rule beside the Action line and the namespaces: the trailing `allowed_signers` key comment `amont-attest` → `greensummit`, and the workflow comment `refs/notes/amont-attest` → `refs/notes/greensummit` (prefix `# greensummit:`). Both are census hits and neither takes part in verification.
- 2026-11-17 — census allowlist (Verification 6). Excluded paths (history or generated from history): vendored packs `.adr/packs/*.pack`, generated `HEADS.md` (both quote accepted decision text), `docs/plans`, `docs/programmes`, `docs/adr` (closed plans, tickets, ADR bodies), CHANGELOGs. The one hit outside them is allowlisted: relais `crates/relais/src/verify/mod.rs:3571` `PRE_FIELD_RECEIPT_5`, a recorded receipt fixture. Observed over the 25 repos' origin/main only; `~/.claude`, `git config --global` and every worktree are still open for Phase 5.
- 2026-11-17 — duro-design-system swept narrow (Action + both `allowed_signers` lines, cluster-signer included): `feat/greenrack` already regenerates its pack, HEADS.md and CLAUDE.md. Its land filed no note. Corrected later the same day: the registry scope does not explain it, since a probe land of main e71dffb3 attested the four universal gates. The sweep land's receipt reads `gates.owed []`, `prestage stamped`, and its log has no `attested` line. A later land with the same receipt fields (this repo, a47a4cc) did attest, so the cause was not found. `pre-push-registry` is still scoped to registry files (`greengate rehearse --cluster`: "nothing to rehearse"). The cluster check waits for the first dds push that touches the registry.
- 2026-11-17 — Verification 4, sre-agent half: resolution and verification observed, skip NOT observed. The PR's CI resolved `https://github.com/greenharness/greensummit@v2` on Forgejo and printed `verified under greensummit`; pytest still ran because sre-agent's `pre-push-pytest` is inert there (needs pytest.ini or conftest.py) — pre-existing, not this rename. relais#491's CI skipped its attested suite (`sign attestations` skipping, tests pass).
- 2026-11-17 — found in the sweep, outside it: duro-app's `prepare` runs `amont init` (devDependency amont 1.47.1), so each land's snapshot install rewrites the live checkout's hooks with the amont shim (restored by `greengate init`); activity-vision had no `greengate.attest` (set to true, the other 23 had it).
- 2026-11-17 — Phase 3 done: greenbelay 6.2.0 (greenbelay#281/#282); homelab#1224 (signer on greengate 4.11.0); relais#489 (Action `@v2`, `sign@9a54c1d`, verifier greensummit 2.0.0, `GREENSUMMIT_BIN`, every `allowed_signers` line incl. cluster-signer on `greensummit`); grh#50 released as grh 0.10.0 (part `greensummit`, alias `attest`, pins greengate 4.11.0 + greenbelay 6.2.0; installed on this Mac); greenharness-skills needed no change (no old name outside history; "attest" there is a role word).
- 2026-11-17 — Verification 3 observed: a probe push of attest faee452 moved origin `refs/notes/greensummit` fb96e763 → 5107b682 (header `greensummit-v2`, x86_64-macos) and left `greengate-attest` at f368825a.
- 2026-11-17 — Verification 4, relais half observed: main d80be294's push run signed all three platforms under `refs/notes/greensummit`; the release dry run (run 44075937033) printed `verified under greensummit` three times and passed every platform, no warning. Still open: a non-empty `covered=` on the first relais PR and on sre-agent after its sweep.
- 2026-11-17 — the cluster check (a cluster-signed `greensummit-v2` block filed by collect) moves to duro-design-system after its Phase 4 sweep: relais has no cluster lane (the gate-runner image lacks Rust and adr-lint), so only duro's `registry` gate reaches the in-cluster signer.
- 2026-11-17 — v2.0.0 released (greensummit#2, tag on 9a54c1d): crates.io `greensummit` 2.0.0, 7 assets, the x86_64 macOS asset prints `greensummit 2.0.0`. `move-major` failed on a first-ever major (the plural refs route prefix-matched `v2.0.0`), so `v2` was created by hand after the workflow's own checks, and the workflow was fixed (greensummit#3).
- 2026-11-17 — Phase 1 code came from relais run run-6606bbe73f2de-cb75 (salvaged: it changes Cargo.toml/Makefile, the profile's own inputs), verified by hand; one test expectation fixed.
- 2026-11-17 — follow-up, owner greensummit 2.0.x: collapse verify.rs's one-element set loops (`states`, `sets`, `found` breaks) into a direct read; deliberate in this rename PR (no behaviour change).
- 2026-11-16 — the ADR is 0073, not 0072: the topo-spotter rename took 0072 the same hour. It also replaces `harness.standalone-tools`. `.adr.yaml` needed no change (no new key); `aval.pack` was regenerated (decisions#218).
- 2026-11-16 — the CI signing key was rotated (`ci@greensummit.invalid`, a fresh key set only as the new repo's secret): the old `ci@attest` private key is not on this machine, only attest trusted it, and the hard cut leaves its old notes unread anyway.
- 2026-11-16 — only `main`, the tags and the notes were pushed to the new repo; the 43 merged branches stayed behind.
- 2026-11-16 — wire names hard-cut with no window, the person's choice at
  charting, against ADR-0071's ordered switch; recorded by ADR-0072.
- 2026-11-16 — greensummit's single name set keeps `producer: "greengate"`:
  the producer line names the gate runner, not this tool.
- 2026-11-16 — new repository + archive instead of a transfer: GitHub does
  not redirect Actions after a transfer (the person, after Codex's review);
  ADR-0072 records it against `harness.renames`' "moves by GitHub transfer".
- 2026-11-16 — `attest-signer` and other role words keep their names
  (ADR-0071: the role word becomes the part's description).

## Implementation review
- implementation-review (opus): approve-with-changes → delta approve-with-changes; 4 fixed, 2 plan-only (Verification 1 cites the real test, Phase 1 split); 86k+93k tok.
- backend: approve-with-changes → approve; trail line deliberate (panel/person scope), collapse stated in PR, ceiling fixed, tests converted not cut; 83k+87k.
- lang:rust: approve-with-changes → approve-with-changes; comments/tmp refs fixed; one-set loops deliberate (follow-up named); stdout→stderr line goes in the 2.0.0 CHANGELOG entry at release; 65k+71k.
- platform: approve-with-changes → approve-with-changes; RELEASE bumped by prepare-release (1b); rollback in the plan only, deliberate: no outside consumers; 69k+72k.
- unix approve-with-changes → approve (60k+68k); architect approve-with-changes → approve (53k+60k).
- CI fix (Windows CRLF fixture, `.gitattributes`): delta implementation-review approve, lang:rust and platform approve-with-changes, nothing new; platform's rollback-in-a-shipped-file kept deliberate (owner's acceptance in the PR body).

<!-- panel: repos=attest,amont,amont-agent,relais,grh,greenharness-skills,homelab,decisions adds=contract reviewers=backend,architect,po,lang:rust,lang:python,platform,tui,unix,codex body-sha=8688f3a08ef1 -->
