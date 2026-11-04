//! The declared palette: the role words the format names, and the colour each one is drawn in.
//!
//! `planet-package` knows roles, not colours — but the **declared set** is shared ground: the
//! importer quantises a source colour to a role word, and the client resolves that word back to a
//! colour. Keeping one table here is what stops the two answers from drifting apart, which would
//! show up as a cell drawn in a colour the package never named.
//!
//! Alpha is part of an entry because `transparent` is a declared paper role with no paper at all.
//! It is never the answer to a quantisation: a source colour that resolved to `transparent` would
//! be drawn as nothing, and a cell that draws nothing is not a cell anyone can see.

/// One declared role and the colour it is drawn in.
pub const DECLARED: &[(&str, (u8, u8, u8, u8))] = &[
    ("surface", (20, 22, 28, 255)),
    ("chrome", (27, 30, 38, 255)),
    ("background", (11, 12, 16, 255)),
    ("divider", (42, 47, 58, 255)),
    ("transparent", (0, 0, 0, 0)),
    ("text", (230, 232, 240, 255)),
    ("dim", (154, 160, 173, 255)),
    ("disabled", (90, 96, 109, 255)),
    ("accent", (108, 182, 255, 255)),
    ("red", (255, 107, 107, 255)),
    ("green", (100, 214, 138, 255)),
    ("orange", (255, 171, 94, 255)),
];

/// The colour a role is drawn in, or `None` when this palette does not carry it.
pub fn colour_of(role: &str) -> Option<(u8, u8, u8, u8)> {
    DECLARED
        .iter()
        .find(|(name, _)| *name == role)
        .map(|(_, colour)| *colour)
}

/// The declared role nearest to `rgb`, by squared distance in the colour's own space.
///
/// Only opaque roles are candidates: `transparent` is not a colour a source fill can be quantised
/// to, and the paper role `surface` stays a candidate because it is what the stage already is.
/// The declared **paper** role nearest to `rgb`.
///
/// A `fill` row draws paper, so a cell's colour has to name one of the paper roles; the ink roles
/// are not candidates however close they are, because a fill row that named an ink would be refused
/// `cellframe-vocabulary`. `transparent` is excluded for the same reason it is excluded above: a
/// cell that draws nothing is not a cell.
pub fn nearest_paper(rgb: (u8, u8, u8)) -> &'static str {
    let mut best: Option<(u64, &'static str)> = None;
    for (name, (r, g, b, a)) in DECLARED {
        if *a == 0 || !crate::cellframe::PAPERS.contains(name) {
            continue;
        }
        let distance = squared(rgb, (*r, *g, *b));
        best = match best {
            None => Some((distance, *name)),
            Some((current, _)) if distance < current => Some((distance, *name)),
            Some(kept) => Some(kept),
        };
    }
    best.map(|(_, name)| name).unwrap_or("surface")
}

pub fn nearest(rgb: (u8, u8, u8)) -> &'static str {
    let mut best: Option<(u64, &'static str)> = None;
    for (name, (r, g, b, a)) in DECLARED {
        if *a == 0 {
            continue;
        }
        let distance = squared(rgb, (*r, *g, *b));
        best = match best {
            None => Some((distance, *name)),
            Some((current, _)) if distance < current => Some((distance, *name)),
            Some(kept) => Some(kept),
        };
    }
    best.map(|(_, name)| name).unwrap_or("surface")
}

fn squared(left: (u8, u8, u8), right: (u8, u8, u8)) -> u64 {
    let channel = |a: u8, b: u8| {
        let difference = i64::from(a) - i64::from(b);
        (difference * difference) as u64
    };
    channel(left.0, right.0) + channel(left.1, right.1) + channel(left.2, right.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_role_has_a_colour() {
        for (role, _) in DECLARED {
            assert!(colour_of(role).is_some(), "{role}");
        }
        assert!(colour_of("mauve").is_none());
    }

    #[test]
    fn a_role_quantises_to_itself() {
        for (role, (r, g, b, a)) in DECLARED {
            if *a == 0 {
                continue;
            }
            assert_eq!(nearest((*r, *g, *b)), *role);
        }
    }

    #[test]
    fn transparent_is_never_the_answer() {
        // Black is nearer `background` than anything else, but a transparent answer would be a cell
        // that draws nothing.
        assert_ne!(nearest((0, 0, 0)), "transparent");
        assert_eq!(nearest((0, 0, 0)), "background");
    }

    #[test]
    fn a_paper_quantisation_answers_a_paper_role() {
        for rgb in [(255, 0, 0), (0, 255, 0), (0, 0, 255), (128, 128, 128), (0, 0, 0)] {
            let role = nearest_paper(rgb);
            assert!(crate::cellframe::PAPERS.contains(&role), "{role}");
            assert_ne!(role, "transparent");
        }
    }

    #[test]
    fn an_unusual_colour_lands_on_something_visible() {
        let role = nearest((255, 0, 255));
        assert_ne!(role, "transparent");
        assert!(colour_of(role).is_some());
    }
}
