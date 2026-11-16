//! The one note-name set, `greensummit`, and the older names that are no
//! longer read, proven with real `ssh-keygen` keys and a real bare origin
//! through the verifier's entry point (SPEC.md, "Names").

use std::path::{Path, PathBuf};
use std::process::Command;

use greensummit::format::{self, Names, SUMMIT, WRITE};
use greensummit::verify::{evaluate, Verdict};

fn run(dir: &Path, program: &str, args: &[&str]) -> String {
    let out = Command::new(program)
        .current_dir(dir)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{program} {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

struct Fixture {
    dir: PathBuf,
    origin: Option<PathBuf>,
    tree: String,
}

impl Fixture {
    /// A repository whose signers file lists `namespaces`, with no origin: its
    /// local refs are judged as they are.
    fn local(name: &str, namespaces: &str) -> Fixture {
        let dir =
            std::env::temp_dir().join(format!("greensummit-names-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir); // a leftover of an earlier run; absent is the normal case
        std::fs::create_dir_all(&dir).unwrap();
        run(
            &dir,
            "ssh-keygen",
            &["-q", "-t", "ed25519", "-N", "", "-f", "key"],
        );
        run(
            &dir,
            "ssh-keygen",
            &["-q", "-t", "ed25519", "-N", "", "-f", "stranger"],
        );
        let public = std::fs::read_to_string(dir.join("key.pub")).unwrap();
        std::fs::write(
            dir.join("allowed_signers"),
            format!("ci@example.org namespaces=\"{namespaces}\" {public}"),
        )
        .unwrap();
        run(&dir, "git", &["init", "-q", "--template="]);
        std::fs::write(dir.join("file.txt"), "content\n").unwrap();
        run(&dir, "git", &["add", "file.txt"]);
        run(
            &dir,
            "git",
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "-qm",
                "init",
            ],
        );
        let tree = run(&dir, "git", &["rev-parse", "HEAD^{tree}"]);
        Fixture {
            dir,
            origin: None,
            tree,
        }
    }

    /// The same, with an empty bare repository as its origin.
    fn with_origin(name: &str, namespaces: &str) -> Fixture {
        let mut fixture = Fixture::local(name, namespaces);
        let origin = fixture.dir.with_extension("origin.git");
        let _ = std::fs::remove_dir_all(&origin); // a leftover of an earlier run
        let path = origin.to_str().unwrap();
        run(&fixture.dir, "git", &["init", "-q", "--bare", path]);
        run(&fixture.dir, "git", &["remote", "add", "origin", path]);
        fixture.origin = Some(origin);
        fixture
    }

    fn payload(&self, names: &Names, gates: &str) -> String {
        format!(
            "{}\ntree {}\ngates {gates}\nplatform x86_64-linux\n{} 1.0\n",
            names.format, self.tree, names.producer
        )
    }

    /// `payload` signed under `ns` with `key`, as a note block.
    fn block(&self, ns: &str, key: &str, payload: &str) -> String {
        std::fs::write(self.dir.join("payload"), payload).unwrap();
        let _ = std::fs::remove_file(self.dir.join("payload.sig")); // absent on the first call
        run(
            &self.dir,
            "ssh-keygen",
            &["-Y", "sign", "-n", ns, "-f", key, "payload"],
        );
        let sig = std::fs::read_to_string(self.dir.join("payload.sig")).unwrap();
        format!("{}\n\n{}", payload.trim_end(), sig.trim_end())
    }

    /// The note on `object` in `notes_ref`, replaced.
    fn note(&self, notes_ref: &str, object: &str, block: &str) {
        run(
            &self.dir,
            "git",
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "notes",
                "--ref",
                notes_ref,
                "add",
                "-f",
                "-m",
                block,
                object,
            ],
        );
    }

    /// A well-formed note of `names`' set on the tree, signed under `ns`.
    fn attest(&self, names: &Names, ns: &str, gates: &str) {
        let block = self.block(ns, "key", &self.payload(names, gates));
        self.note(names.notes_ref, &self.tree, &block);
    }

    /// Publish `refs` to origin and drop the local copies, so the verifier has
    /// to fetch them: origin's content is the only thing it can see.
    fn publish_only(&self, refs: &[&str]) {
        for r in refs {
            let spec = format!("refs/notes/{r}:refs/notes/{r}");
            run(&self.dir, "git", &["push", "-q", "origin", &spec]);
        }
        for r in refs {
            run(
                &self.dir,
                "git",
                &["update-ref", "-d", &format!("refs/notes/{r}")],
            );
        }
    }

    fn verdict(&self) -> Verdict {
        evaluate(
            &self.dir,
            &self.dir.join("allowed_signers"),
            None,
            Some("x86_64-linux"),
            &[],
            false,
        )
    }

    fn covered(&self) -> Vec<String> {
        self.verdict().gates
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir); // temp dir cleanup; a leftover is harmless
        if let Some(origin) = &self.origin {
            let _ = std::fs::remove_dir_all(origin); // as above
        }
    }
}

fn gates(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|s| s.to_string()).collect()
}

/// The names 1.10.x wrote under, which a 2.0.0 reader no longer knows.
const GREENGATE_ATTEST: Names = Names {
    format: "greengate-attest-v2",
    notes_ref: "greengate-attest",
    inputs_ref: "greengate-attest-inputs",
    namespace: "greengate-attest",
    input_domain: "greengate-attest-input",
    canon_domain: "greengate-attest-canon",
    producer: "greengate",
};

/// The names 1.0 to 1.9 wrote under.
const AMONT_ATTEST: Names = Names {
    format: "amont-attest-v2",
    notes_ref: "amont-attest",
    inputs_ref: "amont-attest-inputs",
    namespace: "amont-attest",
    input_domain: "amont-attest-input",
    canon_domain: "amont-attest-canon",
    producer: "amont",
};

/// A signers line that lists every namespace ever used, so that a refusal
/// below is the reader's doing and not the signers file's.
const EVERY_NAMESPACE: &str = "amont-attest,greengate-attest,greensummit";

/// A block captured from a 1.10.x producer, checked in.
const OLD_BLOCK: &str = include_str!("fixtures/greengate-attest-v2.note");

#[test]
fn the_constants_are_the_summit_names() {
    assert_eq!(format::FORMAT, "greensummit-v2");
    assert_eq!(format::NOTES_REF, "greensummit");
    assert_eq!(format::INPUTS_REF, "greensummit-inputs");
    assert_eq!(format::NAMESPACE, "greensummit");
    assert_eq!(WRITE, SUMMIT);
}

/// Everything written derives from `WRITE`: the set-less keys and the
/// payload's producer word included.
#[test]
fn every_written_name_derives_from_write() {
    use greensummit::fingerprint::{canon_key, canon_key_in, key, key_in};
    let fixture = Fixture::local("write", EVERY_NAMESPACE);
    let fp = "a".repeat(40);
    assert_eq!(
        key(&fixture.dir, "ci-fmt", &fp),
        key_in(&fixture.dir, &WRITE, "ci-fmt", &fp)
    );
    assert_eq!(
        canon_key(&fixture.dir, &fp),
        canon_key_in(&fixture.dir, &WRITE, &fp)
    );
    let payload = format::payload("t", "g", "p", &[], None, "x");
    assert!(payload.starts_with(&format!("{}\n", WRITE.format)));
    assert!(payload.ends_with(&format!("\n{} x\n", WRITE.producer)));
}

#[test]
fn the_set_spells_every_name() {
    assert_eq!(SUMMIT.format, "greensummit-v2");
    assert_eq!(SUMMIT.notes_ref, "greensummit");
    assert_eq!(SUMMIT.inputs_ref, "greensummit-inputs");
    assert_eq!(SUMMIT.namespace, "greensummit");
    assert_eq!(SUMMIT.input_domain, "greensummit-input");
    assert_eq!(SUMMIT.canon_domain, "greensummit-canon");
    assert_eq!(SUMMIT.producer, "greengate");
}

#[test]
fn lookups_are_keyed_under_the_summit_domains() {
    use greensummit::fingerprint::{
        canon_key_preimage, canon_key_preimage_in, key_preimage, key_preimage_in,
    };
    assert_eq!(
        key_preimage_in(&SUMMIT, "ci-fmt", "abc"),
        "greensummit-input ci-fmt abc\n"
    );
    assert_eq!(
        canon_key_preimage_in(&SUMMIT, "abc"),
        "greensummit-canon abc\n"
    );
    assert_eq!(
        key_preimage("ci-fmt", "abc"),
        key_preimage_in(&SUMMIT, "ci-fmt", "abc")
    );
    assert_eq!(
        canon_key_preimage("abc"),
        canon_key_preimage_in(&SUMMIT, "abc")
    );
}

#[test]
fn a_greensummit_note_is_verified_and_named() {
    let fixture = Fixture::local("summit", EVERY_NAMESPACE);
    fixture.attest(&SUMMIT, "greensummit", "g-summit");
    let verdict = fixture.verdict();
    assert_eq!(verdict.gates, gates(&["g-summit"]));
    assert_eq!(verdict.verified_under, ["greensummit"]);
}

/// A valid signature on a block that covers nothing (another tree) is not
/// counted: the line says a skip was earned under that name.
#[test]
fn a_valid_block_that_covers_nothing_is_not_named() {
    let fixture = Fixture::local("nocover", EVERY_NAMESPACE);
    let payload = fixture
        .payload(&SUMMIT, "g")
        .replace(&fixture.tree, &"0".repeat(40));
    let block = fixture.block("greensummit", "key", &payload);
    fixture.note(SUMMIT.notes_ref, &fixture.tree, &block);
    let verdict = fixture.verdict();
    assert!(verdict.gates.is_empty());
    assert!(verdict.verified_under.is_empty());
}

/// A note `notes list` finds but `notes show` cannot read is still a note:
/// the trail says why it covers nothing.
#[test]
fn an_unreadable_note_is_named_in_the_trail() {
    let fixture = Fixture::local("unreadable", EVERY_NAMESPACE);
    fixture.attest(&SUMMIT, "greensummit", "g-summit");
    let listing = run(
        &fixture.dir,
        "git",
        &["notes", "--ref", SUMMIT.notes_ref, "list", &fixture.tree],
    );
    let blob = listing.split_whitespace().next().unwrap().to_string();
    let object = fixture
        .dir
        .join(".git/objects")
        .join(&blob[..2])
        .join(&blob[2..]);
    std::fs::remove_file(&object).unwrap();
    let verdict = fixture.verdict();
    assert!(verdict.gates.is_empty(), "{:?}", verdict.gates);
    assert!(
        verdict
            .trail
            .iter()
            .any(|l| l.starts_with("cannot read the note on ")),
        "{:?}",
        verdict.trail
    );
}

/// A note filed under a name this version does not know is absent, whatever
/// its signature and whichever ref it sits in.
#[test]
fn a_note_under_an_old_name_is_absent() {
    for old in [&GREENGATE_ATTEST, &AMONT_ATTEST] {
        let fixture = Fixture::local("oldname", EVERY_NAMESPACE);
        fixture.attest(old, old.namespace, "g-old");
        assert!(fixture.covered().is_empty(), "{}", old.notes_ref);
    }
}

#[test]
fn a_checked_in_greengate_attest_v2_block_reads_as_absent() {
    assert!(OLD_BLOCK.starts_with("greengate-attest-v2\n"));
    for notes_ref in [GREENGATE_ATTEST.notes_ref, SUMMIT.notes_ref] {
        let fixture = Fixture::local("fixture", EVERY_NAMESPACE);
        fixture.note(notes_ref, &fixture.tree, OLD_BLOCK.trim_end());
        let verdict = fixture.verdict();
        assert!(verdict.gates.is_empty(), "{notes_ref}: {:?}", verdict.gates);
        assert!(verdict.verified_under.is_empty(), "{notes_ref}");
    }
}

#[test]
fn an_old_header_block_is_refused_under_the_new_namespace() {
    for old in [&GREENGATE_ATTEST, &AMONT_ATTEST] {
        let fixture = Fixture::local("oldhdr-newns", EVERY_NAMESPACE);
        let block = fixture.block("greensummit", "key", &fixture.payload(old, "g"));
        fixture.note(SUMMIT.notes_ref, &fixture.tree, &block);
        assert!(fixture.covered().is_empty(), "{}", old.format);
    }
}

#[test]
fn a_new_header_block_is_refused_under_an_old_namespace() {
    for old in [&GREENGATE_ATTEST, &AMONT_ATTEST] {
        let fixture = Fixture::local("newhdr-oldns", EVERY_NAMESPACE);
        let block = fixture.block(old.namespace, "key", &fixture.payload(&SUMMIT, "g"));
        fixture.note(SUMMIT.notes_ref, &fixture.tree, &block);
        assert!(fixture.covered().is_empty(), "{}", old.namespace);
    }
}

#[test]
fn a_signers_line_pinned_to_an_old_namespace_accepts_nothing() {
    let fixture = Fixture::local("pin-old", "amont-attest,greengate-attest");
    fixture.attest(&SUMMIT, "greensummit", "g");
    assert!(fixture.covered().is_empty());
}

#[test]
fn origin_holding_only_old_refs_covers_nothing_and_does_not_stop_the_run() {
    let fixture = Fixture::with_origin("onlyold", EVERY_NAMESPACE);
    fixture.attest(&GREENGATE_ATTEST, "greengate-attest", "g-old");
    fixture.publish_only(&["greengate-attest"]);
    let verdict = fixture.verdict();
    assert!(verdict.gates.is_empty());
    let (notes_mirror, inputs_mirror) = verdict.notes.unwrap();
    assert_eq!(
        (notes_mirror.as_str(), inputs_mirror.as_str()),
        ("absent", "absent")
    );
}

#[test]
fn a_local_mirror_origin_does_not_have_is_not_judged() {
    let fixture = Fixture::with_origin("stale", EVERY_NAMESPACE);
    fixture.attest(&SUMMIT, "greensummit", "g-summit");
    // Nothing published: origin has neither ref, so the local mirror is deleted.
    assert!(fixture.covered().is_empty());
}

/// The note refs of `git_dir` (a work tree or a bare repository), sorted.
fn note_refs(dir: &Path, git_dir: &Path) -> Vec<String> {
    let git_dir = git_dir.to_str().unwrap();
    let out = run(
        dir,
        "git",
        &[
            "--git-dir",
            git_dir,
            "for-each-ref",
            "--format=%(refname)",
            "refs/notes/",
        ],
    );
    let mut refs: Vec<String> = out.lines().map(str::to_string).collect();
    refs.sort();
    refs
}

/// `greensummit sign` run in the fixture with its key on stdin, under a HOME
/// and a git config of its own. Returns its stdout.
fn sign_with_cli(fixture: &Fixture, extra: &[&str]) -> String {
    use std::io::Write;
    let mut args = vec![
        "sign",
        "--gates",
        "g1",
        "--platform",
        "x86_64-linux",
        "--allow-dirty",
        "--github-output",
    ];
    args.extend_from_slice(extra);
    let mut child = Command::new(env!("CARGO_BIN_EXE_greensummit"))
        .current_dir(&fixture.dir)
        .args(&args)
        .env("HOME", &fixture.dir)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "t")
        .env("GIT_AUTHOR_EMAIL", "t@t")
        .env("GIT_COMMITTER_NAME", "t")
        .env("GIT_COMMITTER_EMAIL", "t@t")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let key = std::fs::read(fixture.dir.join("key")).unwrap();
    child.stdin.take().unwrap().write_all(&key).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "greensummit sign: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A spec declaring `g1`'s inputs, committed, so a signed block carries an
/// `input` line and is filed in the inputs ref too.
fn commit_spec(fixture: &Fixture) {
    std::fs::create_dir_all(fixture.dir.join(".github")).unwrap();
    std::fs::write(
        fixture.dir.join(".github/greensummit-inputs"),
        "g1 file.txt\n",
    )
    .unwrap();
    run(&fixture.dir, "git", &["add", ".github/greensummit-inputs"]);
    run(
        &fixture.dir,
        "git",
        &[
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "commit",
            "-qm",
            "spec",
        ],
    );
}

/// `greensummit sign` writes the one set and nothing else: the header, the
/// producer word, both refs on origin, the inputs key under the written domain,
/// a signature that verifies under `-n greensummit` alone.
#[test]
fn greensummit_sign_writes_only_the_summit_names() {
    let mut fixture = Fixture::with_origin("sign-green", "greensummit");
    commit_spec(&fixture);
    fixture.tree = run(&fixture.dir, "git", &["rev-parse", "HEAD^{tree}"]);
    let out = sign_with_cli(&fixture, &[]);
    assert!(out.contains("status=pushed\n"), "{out}");

    let origin = fixture.origin.clone().unwrap();
    assert_eq!(
        note_refs(&fixture.dir, &origin),
        ["refs/notes/greensummit", "refs/notes/greensummit-inputs"]
    );
    let origin_dir = origin.to_str().unwrap();
    let body = run(
        &fixture.dir,
        "git",
        &[
            "--git-dir",
            origin_dir,
            "notes",
            "--ref",
            "greensummit",
            "show",
            &fixture.tree,
        ],
    );
    let (payload, _) = format::split_note(&body).unwrap();
    assert!(payload.starts_with("greensummit-v2\n"), "{payload}");
    assert!(payload.ends_with("\ngreengate attest-sign\n"), "{payload}");

    // The inputs note sits under the key of the fingerprint it claims.
    let fp = format::input_fp(&payload, "g1").expect("an input line");
    let k = greensummit::fingerprint::key_in(&fixture.dir, &SUMMIT, "g1", fp).unwrap();
    run(
        &fixture.dir,
        "git",
        &[
            "--git-dir",
            origin_dir,
            "notes",
            "--ref",
            "greensummit-inputs",
            "show",
            &k,
        ],
    );

    // The signers file lists greensummit alone, so only a
    // signature verifies, and the verifier says which set it went through.
    let verdict = fixture.verdict();
    assert_eq!(verdict.gates, gates(&["g1"]));
    assert_eq!(verdict.verified_under, ["greensummit"]);
}

/// `--no-push` keeps the block under the local ref.
#[test]
fn greensummit_sign_no_push_writes_the_local_refs() {
    let fixture = Fixture::with_origin("sign-local", "greensummit");
    let out = sign_with_cli(&fixture, &["--no-push"]);
    assert!(out.contains("status=local\n"), "{out}");
    let refs = note_refs(&fixture.dir, &fixture.dir.join(".git"));
    assert_eq!(
        refs,
        [
            "refs/notes/greensummit-local/greensummit",
            "refs/notes/greensummit-local/greensummit-inputs"
        ]
    );
    let body = run(
        &fixture.dir,
        "git",
        &[
            "notes",
            "--ref",
            "greensummit-local/greensummit",
            "show",
            &fixture.tree,
        ],
    );
    assert!(body.starts_with("greensummit-v2\n"), "{body}");
    // The block's canonical fingerprint is filed under its key in the
    // local inputs ref, and under no 1.x key there.
    let (payload, _) = format::split_note(&body).unwrap();
    let canon = format::canon_claim(&payload).expect("a canon line");
    let summit_key = greensummit::fingerprint::canon_key_in(&fixture.dir, &SUMMIT, canon).unwrap();
    let old_key =
        greensummit::fingerprint::canon_key_in(&fixture.dir, &AMONT_ATTEST, canon).unwrap();
    let inputs = |key: &str| {
        run(
            &fixture.dir,
            "git",
            &[
                "notes",
                "--ref",
                "greensummit-local/greensummit-inputs",
                "show",
                key,
            ],
        )
    };
    assert!(inputs(&summit_key).contains("BEGIN SSH SIGNATURE"));
    let old_present = Command::new("git")
        .current_dir(&fixture.dir)
        .args([
            "notes",
            "--ref",
            "greensummit-local/greensummit-inputs",
            "show",
            &old_key,
        ])
        .output()
        .unwrap()
        .status
        .success();
    assert!(!old_present, "an old-name key in the inputs ref");
}

/// `notes::sync_mirrors` brings the written set's mirrors in line with origin
/// and leaves older names alone: an origin with neither deletes the mirror
/// and keeps an old-name ref.
#[test]
fn sync_mirrors_touches_only_the_summit_refs() {
    let fixture = Fixture::with_origin("sync", EVERY_NAMESPACE);
    fixture.attest(&SUMMIT, "greensummit", "g-green");
    fixture.attest(&AMONT_ATTEST, "amont-attest", "g-old");
    let mut loud = Vec::new();
    let (notes, inputs) = greensummit::notes::sync_mirrors(&fixture.dir, &mut loud);
    assert_eq!(notes, greensummit::notes::Mirror::Deleted);
    assert_eq!(inputs, greensummit::notes::Mirror::Deleted);
    assert_eq!(
        note_refs(&fixture.dir, &fixture.dir.join(".git")),
        ["refs/notes/amont-attest"]
    );
}
