//! The imported package carries the bitmaps' own RGBA texels, and the client paints them as pixel art.
//!
//! The comparison is byte-for-byte in both directions: the importer states a digest of the texels it
//! decoded from the payload, and the test compares it with the digest of the block it read back out
//! of the package's artifact table. No window is opened, so this is client draw-path evidence and not
//! a client witness (`AGENTS.md` §7).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

use planet_client::picture::Picture;
use planet_client::texel;
use planet_package::cellframe::CellFrame;
use planet_package::texels::Texels;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("the crate sits two levels under the repository root")
        .to_path_buf()
}

fn picture(width: u32, height: u32) -> Picture {
    Picture {
        width,
        height,
        stride: width * 4,
        pixels: vec![0u8; (width * height * 4) as usize],
    }
}

#[test]
fn the_package_carries_the_bitmaps_texels_and_the_client_paints_them() {
    println!("headless true window never opened — evidence about the package, not a client witness");
    let root = root();
    let input = root.join("inputs").join("swf1.swf");
    if !input.is_file() {
        eprintln!("absent: inputs/swf1.swf is not in this checkout, so nothing was proved");
        return;
    }
    let generated = root.join("generated");
    std::fs::create_dir_all(&generated).expect("generated/ is writable");
    let package = generated.join("swf1-texels.pkg");
    let ledger = generated.join("mount-ledger-texels.tsv");
    let output = Command::new(env!("CARGO_BIN_EXE_swf-import"))
        .arg(&input)
        .arg(&package)
        .arg(&ledger)
        .arg("swf1")
        .output()
        .expect("the importer binary runs");
    assert!(output.status.success(), "the importer refused the real package");
    let printed = String::from_utf8_lossy(&output.stdout);

    // What the importer decoded out of the payloads, keyed by character.
    let mut source_digests: BTreeMap<u16, String> = BTreeMap::new();
    let mut refused: Vec<String> = Vec::new();
    for line in printed.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields[0] == "bitmap_texel" {
            let digest_at = fields
                .iter()
                .position(|field| *field == "digest")
                .unwrap_or_else(|| panic!("a digest is stated on {line}"));
            source_digests.insert(
                fields[1].parse().expect("a character id"),
                fields[digest_at + 1].to_string(),
            );
        }
        if fields.len() >= 4 && fields[0] == "bitmap_refused" {
            refused.push(fields[3].to_string());
        }
    }
    assert_eq!(
        source_digests.len(),
        16,
        "sixteen lossless payloads decode to texels; the importer printed {}",
        source_digests.len()
    );
    assert_eq!(
        refused.len(),
        2,
        "the two JPEG-tag bitmaps stay counted and refused, not faked: {refused:?}"
    );
    assert!(
        refused.iter().all(|code| code == "bitmap-tag-unimplemented"),
        "the JPEG refusal is named: {refused:?}"
    );

    // What the package carries, read back through the grammar and the texel codec.
    let document =
        std::fs::read_to_string(package.with_extension("frame")).expect("the frame document");
    let frame = CellFrame::parse(&document).expect("the grammar reads the frame it carried");
    let carried: Vec<Texels> = frame
        .artifacts
        .iter()
        .filter(|artifact| artifact.path.starts_with("bitmap/") && artifact.path.ends_with(".rgba"))
        .map(|artifact| {
            Texels::decode(&artifact.bytes).unwrap_or_else(|refusal| {
                panic!("artifact {} does not decode: {refusal}", artifact.path)
            })
        })
        .collect();
    println!(
        "bitmap_artifacts {} frame_artifacts {}",
        carried.len(),
        frame.artifacts.len()
    );
    assert_eq!(carried.len(), 16, "the package carries sixteen texel blocks");

    for block in &carried {
        assert_eq!(
            block.pixels.len(),
            block.width as usize * block.height as usize,
            "bitmap {} carries its full rectangle",
            block.id
        );
        assert!(
            block.distinct_colours() > 8,
            "bitmap {} is a picture rather than a flat block ({} colours)",
            block.id,
            block.distinct_colours()
        );
        assert_eq!(
            Texels::decode(&block.encode()).expect("re-encoding the carried block"),
            *block,
            "bitmap {} survives an encode/decode round trip unchanged",
            block.id
        );
        let source = source_digests
            .get(&block.id)
            .unwrap_or_else(|| panic!("the importer printed texels for bitmap {}", block.id));
        assert_eq!(
            &block.digest(),
            source,
            "bitmap {}: the source texels and the package texels are the same bytes",
            block.id
        );
    }

    // **The client's own paint path, at unit scale: every pixel.** A picture the size of the block is
    // painted from the *package's* texels, then compared against those same texels pixel by pixel.
    let first = &carried[0];
    let width = first.width as u32;
    let height = first.height as u32;
    let mut canvas = picture(width, height);
    let painted = texel::paint_texels(&mut canvas, first, 0, 0, width, height);
    let opaque = first.pixels.iter().filter(|pixel| pixel[3] != 0).count();
    assert_eq!(painted, opaque, "every opaque texel reached the picture");
    let mut mismatches = 0usize;
    for y in 0..height {
        for x in 0..width {
            let source = first.pixel(x, y).expect("a texel inside the block");
            if source[3] == 0 {
                continue;
            }
            if canvas.pixel(x, y) != Some(source) {
                mismatches += 1;
            }
        }
    }
    assert_eq!(
        mismatches, 0,
        "the rendered pixels equal the payload's pixels for bitmap {}",
        first.id
    );

    // **The same path at a placement's scale.** A sprite region is drawn into a stage-sized picture
    // and the region is sampled: a flat rectangle would answer one colour.
    let mut stage = picture(640, 480);
    let scaled = texel::paint_texels(&mut stage, first, 16, 32, 112, 208);
    assert!(scaled > 0, "the block painted its region");
    let mut colours: BTreeSet<[u8; 4]> = BTreeSet::new();
    for y in 32..32 + 208 {
        for x in 16..16 + 112 {
            if let Some(pixel) = stage.pixel(x, y) {
                if pixel[3] != 0 {
                    colours.insert(pixel);
                }
            }
        }
    }
    println!(
        "rendered_region {}x{} painted {} colours {}",
        112,
        208,
        scaled,
        colours.len()
    );
    assert!(
        colours.len() > 8,
        "the drawn region is pixel art ({} colours), not one mean colour",
        colours.len()
    );
    assert!(
        colours.len() <= first.distinct_colours(),
        "a render cannot invent colours the payload does not hold"
    );
}
