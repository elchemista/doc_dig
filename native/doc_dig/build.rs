//! Build script for `doc_dig`.
//! 1. Ensure **all** the shared libraries built by the `extractous` crate
//!    (especially `libtika_native.so`) are present under `priv/native/` so they
//!    ship with the compiled NIF and are discoverable at run‑time.
//! 2. Make the script idempotent across Mix environments (`dev`, `test`, etc.).
//!    If the libs are **already** in `priv/native/`, we do *not* fail – we just
//!    emit the correct RPATH and exit early. This prevents the failure you saw
//!    when `mix test` recompiles in a fresh profile without rebuilding
//!    `extractous`.
//! 3. Inject `-Wl,-rpath,$ORIGIN` on Linux, so the dynamic‑linker looks in the
//!    NIF’s own directory (`priv/native`) first.
//!
//! Implementation is pure `std`; no external dependencies.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // ── Determine important paths ────────────────────────────────────────────
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let priv_native = crate_root
        .parent() // native
        .and_then(|p| p.parent()) // project root
        .expect("cannot determine project root")
        .join("priv/native");

    // Ensure priv/native exists so `mix release` can bundle it even if empty.
    fs::create_dir_all(&priv_native).unwrap();

    // ── Fast‑path: libs already present in priv/native ───────────────────────
    let already_copied = priv_native.join("libtika_native.so");
    if already_copied.is_file() {
        println!("cargo:rerun-if-changed={}", already_copied.display());
        inject_rpath();
        return; // nothing else to do
    }

    // ── Slow‑path: we need to locate extractous output and copy libs ─────────
    let tika_path =
        find_libtika(&out_dir).expect("Could not locate libtika_native.so built by extractous");
    let libs_dir = tika_path.parent().expect("tika path has no parent");

    let patterns = if cfg!(target_os = "macos") {
        vec!["dylib"]
    } else {
        vec!["so"]
    };

    for entry in fs::read_dir(libs_dir).unwrap() {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if patterns.iter().any(|p| p == &ext) {
                let dest = priv_native.join(path.file_name().unwrap());
                fs::copy(&path, &dest)
                    .unwrap_or_else(|e| panic!("copy {} failed: {}", path.display(), e));
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }

    inject_rpath();
}

fn inject_rpath() {
    if cfg!(target_os = "linux") {
        // $ORIGIN resolves to the directory containing the final libdoc_dig.so
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }
}

fn find_libtika(out_dir: &Path) -> Option<PathBuf> {
    // OUT_DIR (vendored or cached)
    let vendored = out_dir.join("libtika_native.so");
    if vendored.is_file() {
        return Some(vendored);
    }

    // Same‑profile sibling `extractous-*` directories
    if let Some(p) = find_in_extractous_tree(out_dir) {
        return Some(p);
    }

    // Opposite profile (release ↔ debug)
    if let Some(p) = find_in_opposite_profile(out_dir) {
        return Some(p);
    }

    // Search everything under the nearest `build/`
    if let Some(build_root) = ascend_until(out_dir, |seg| seg == "build") {
        if let Some(p) = search_recursively(build_root) {
            return Some(p);
        }
    }
    None
}

fn find_in_opposite_profile(out_dir: &Path) -> Option<PathBuf> {
    let profile_dir = out_dir.parent()?.parent()?.parent()?; // {profile}
    let opp_profile = match profile_dir.file_name()?.to_str()? {
        "release" => "debug",
        "debug" => "release",
        _ => return None,
    };
    let sibling = profile_dir.parent()?.join(opp_profile).join("build");
    if sibling.is_dir() {
        return search_recursively(&sibling);
    }
    None
}

fn ascend_until<'a>(start: &'a Path, pred: impl Fn(&str) -> bool) -> Option<&'a Path> {
    let mut current = start;
    loop {
        if let Some(name) = current.file_name().and_then(|n| n.to_str()) {
            if pred(name) {
                return Some(current);
            }
        }
        current = current.parent()?;
    }
}

use std::collections::VecDeque;
fn search_recursively(root: &Path) -> Option<PathBuf> {
    let mut q = VecDeque::from([root.to_path_buf()]);
    while let Some(dir) = q.pop_front() {
        for entry in fs::read_dir(&dir).ok()? {
            let p = entry.ok()?.path();
            if p.is_dir() {
                q.push_back(p);
            } else if p
                .file_name()
                .map(|n| n == "libtika_native.so")
                .unwrap_or(false)
            {
                return Some(p);
            }
        }
    }
    None
}

fn find_in_extractous_tree(out_dir: &Path) -> Option<PathBuf> {
    // out_dir → …/target/{triple}/{profile}/build/doc_dig-*/out
    let build_root = out_dir.parent()?.parent()?; // …/build/
    let mut q: VecDeque<PathBuf> = build_root
        .read_dir()
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .map_or(false, |n| n.to_string_lossy().starts_with("extractous-"))
        })
        .collect();

    while let Some(dir) = q.pop_front() {
        for entry in fs::read_dir(&dir).ok()? {
            let p = entry.ok()?.path();
            if p.is_dir() {
                q.push_back(p.clone());
            } else if p
                .file_name()
                .map(|n| n == "libtika_native.so")
                .unwrap_or(false)
            {
                return Some(p);
            }
        }
    }
    None
}
