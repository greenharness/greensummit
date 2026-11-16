# git-attest becomes greensummit — full reviews

## Full reviews (reference)

Round 1 (opus-5-5, all approve-with-changes): po 35k/39 s, architect 41k/48 s, tui 40k/42 s, lang:python 42k/48 s, platform 37k/48 s, lang:rust 48k/58 s, backend 47k/79 s, unix 49k/125 s. 32 findings: 31 accepted, 1 rejected (tui#2, the old-ref trail line: out of the person's plain-sed scope).
Round 2 (resumed): 8 approve-with-changes; the sed recipe's HTML escapes, `#` delimiter and `:(exclude)` pathspecs, the order of step 6 with an `ATTEST_SIGNING_KEY` placeholder, and the 24 count were applied. The gate_stamp LEGACY conflict (lang:rust keep, backend and unix drop) went to the person, who chose drop.
Codex (gpt, high): rework. (1) blocking: GitHub does not redirect Actions after a transfer (docs confirmed), so the person chose a new repo + archive; (2) blocking: greengate's release verifier; (3) high: compat tests keep old names; (4) high: greenbelay keeps provenance names. All applied.
Round 4 (after Codex): backend, architect, lang:rust, po, unix and tui found a high on the `push --mirror` source and on origin being repointed only late; fixed with explicit refspecs from a `$TMPDIR` mirror clone and set-url in Phase 1. Final bind: all approve.
Carried into implementation: the relais/greengate release tag must carry a greensummit note (merge pushed through greengate); greenbelay's reader installed with greengate; the signer `--version` text bump; rollback wording (keep "archived actions still resolve"); a rollback trigger (<80% of swept repos skipping 48 h after the sweep); a CHANGELOG/README line saying old-name notes are ignored; measure the push guard in the gap between the greengate and greenbelay installs; the Phase 5 README commit lands as a PR on `fredericrous/attest`; `gh run list` on the new repo is empty after the refspec push.

