use planet_package::cellframe::{
    check_extent, CellFrame, HEADER, MAX_CELLS, MAX_COLS, MAX_FRAME_BYTES, MAX_ROWS,
};

fn base() -> String {
    format!(
        "{HEADER}\n\
         frame\tMAP-0001\n\
         \twindow\tmap-plan\n\
         \tcols\t56\n\
         \trows\t22\n\
         \torder\t10\n\
         \trequires\tnone\n\
         \tfeature\tall\n\
         fill\t0\t0\t56\tsurface\n\
         run\t0\t0\ttext\tmap-plan\n"
    )
}

fn map_document() -> String {
    format!(
        "{}\
         \tfield\televation\tterrain::elevation_m\n\
         \tfield\tbiome\tecologiy::biome_at\n\
         \tshaper\tprofile32\t32\n\
         \tprojection\torthographic\n\
         \tcentre\t1024\t-512\n\
         \tcell\t32\n\
         \textent\t56\t22\n\
         \tpitch\t-45\n",
        base()
    )
}

fn parse(document: &str) -> Result<CellFrame, planet_package::Refusals> {
    CellFrame::parse(document)
}

fn code(document: &str) -> &'static str {
    parse(document).expect_err("the document should be refused").code()
}

#[test]
fn a_valid_frame_parses_into_typed_rows() {
    let frame = parse(&base()).expect("a well-formed frame");
    assert_eq!(frame.id, "MAP-0001");
    assert_eq!(frame.window, "map-plan");
    assert_eq!((frame.cols, frame.rows, frame.order), (56, 22, 10));
    assert!(frame.requires.is_empty());
    assert_eq!(frame.feature.as_deref(), Some("all"));
    assert_eq!(frame.fills.len(), 1);
    assert_eq!(frame.fills[0].paper, "surface");
    assert_eq!(frame.fills[0].cols, 56);
    assert_eq!(frame.runs[0].ink, "text");
    assert_eq!(frame.runs[0].text, "map-plan");
    assert!(frame.map.is_none());
    assert!(frame.vocabulary.accepts_paper("chrome"));
}

#[test]
fn a_map_declaration_is_one_typed_record() {
    let frame = parse(&map_document()).expect("a map frame");
    let map = frame.map.expect("the sampling rows are one declaration");
    assert_eq!(map.fields.len(), 2);
    assert_eq!(map.source_of("elevation"), Some("terrain::elevation_m"));
    assert_eq!(map.shaper, "profile32");
    assert_eq!(map.shaper_cells, 32);
    assert_eq!(map.projection, "orthographic");
    assert_eq!(map.centre_xz, [1024.0, -512.0]);
    assert_eq!(map.cell_m, 32.0);
    assert_eq!(map.extent, [56, 22]);
    assert_eq!(map.pitch_deg, -45.0);
}

#[test]
fn a_foreign_header_is_refused_by_name() {
    let document = base().replace(HEADER, "shploingus.cellframe/0");
    assert_eq!(code(&document), "cellframe-schema");
    let refusal = parse(&document).unwrap_err();
    assert!(refusal.message().contains("shploingus.cellframe/1"), "{}", refusal.message());
}

#[test]
fn an_unknown_key_is_refused_by_name() {
    let document = base().replace("\torder\t10\n", "\torder\t10\n\tspan\t9\n");
    assert_eq!(code(&document), "cellframe-key");
    assert!(parse(&document).unwrap_err().message().contains("span"));
}

#[test]
fn a_duplicate_key_is_refused_by_name() {
    let document = base().replace("\torder\t10\n", "\torder\t10\n\tcols\t56\n");
    assert_eq!(code(&document), "cellframe-duplicate");
}

#[test]
fn every_missing_key_is_named_in_one_pass() {
    let document = base()
        .replace("\tcols\t56\n", "")
        .replace("\torder\t10\n", "");
    let refusal = parse(&document).unwrap_err();
    assert_eq!(refusal.len(), 2, "{}", refusal.message());
    assert!(refusal.message().contains("cols"), "{}", refusal.message());
    assert!(refusal.message().contains("order"), "{}", refusal.message());
    assert!(refusal.problems().iter().all(|problem| problem.code() == "cellframe-missing"));
}

#[test]
fn a_row_past_the_grid_is_refused_with_the_row() {
    let document = base().replace("fill\t0\t0\t56\tsurface\n", "fill\t50\t0\t56\tsurface\n");
    assert_eq!(code(&document), "cellframe-grid");
    assert!(parse(&document).unwrap_err().message().contains("fill"));
}

#[test]
fn a_frame_that_draws_nothing_is_refused() {
    let document = base()
        .replace("fill\t0\t0\t56\tsurface\n", "")
        .replace("run\t0\t0\ttext\tmap-plan\n", "");
    assert_eq!(code(&document), "cellframe-empty");
}

#[test]
fn an_undeclared_role_is_refused_rather_than_substituted() {
    let document = base().replace("\tsurface", "\thazard");
    assert_eq!(code(&document), "cellframe-vocabulary");
}

#[test]
fn a_field_source_is_a_law_token_and_never_an_expression() {
    let document = map_document().replace("terrain::elevation_m", "elevation + 1");
    assert_eq!(code(&document), "cellframe-binding");
}

#[test]
fn a_field_name_the_renderer_does_not_sample_is_refused() {
    let document = map_document().replace("\tfield\televation", "\tfield\ttemperature");
    assert_eq!(code(&document), "cellframe-binding");
}

#[test]
fn one_field_has_one_source() {
    let document = map_document().replace("elevation + 1", "terrain::elevation_m");
    let document = format!(
        "{}",
        document.replace(
            "\tfield\tbiome\tecologiy::biome_at\n",
            "\tfield\tbiome\tecologiy::biome_at\n\tfield\televation\tterrain::height_m\n"
        )
    );
    assert_eq!(code(&document), "cellframe-duplicate");
}

#[test]
fn a_partial_map_is_refused_rather_than_drawn_as_glyphs() {
    let document = map_document().replace("\tpitch\t-45\n", "");
    let refusal = parse(&document).unwrap_err();
    assert!(refusal.message().contains("pitch"), "{}", refusal.message());
}

#[test]
fn the_formats_bounds_are_refusals_not_truncations() {
    assert_eq!(
        code(&base().replace("\tcols\t56\n", &format!("\tcols\t{}\n", MAX_COLS + 1))),
        "cellframe-bound"
    );
    assert_eq!(
        code(&base().replace("\trows\t22\n", &format!("\trows\t{}\n", MAX_ROWS + 1))),
        "cellframe-bound"
    );
    let wide = base()
        .replace("\tcols\t56\n", "\tcols\t200\n")
        .replace("\trows\t22\n", "\trows\t200\n")
        .replace("fill\t0\t0\t56\tsurface\n", "fill\t0\t0\t200\tsurface\n");
    let refusal = parse(&wide).unwrap_err();
    assert!(
        refusal.message().contains(&MAX_CELLS.to_string()),
        "{}",
        refusal.message()
    );
}

#[test]
fn a_declaration_over_the_byte_ceiling_is_refused_before_it_is_read() {
    let mut document = base();
    while document.len() <= MAX_FRAME_BYTES {
        document.push_str("# padding so the ceiling is the first thing to fail\n");
    }
    assert_eq!(code(&document), "cellframe-bound");
}

#[test]
fn the_streamed_vocabulary_section_declares_the_words_a_package_may_use() {
    let document = format!(
        "[vocabulary]\n\
         paper\tsurface,chrome,background,divider,transparent,hazard\n\
         ink\ttext,dim,disabled,accent,divider,red,green,orange\n\
         \n\
         {}",
        base().replace("\tsurface", "\thazard")
    );
    let frame = parse(&document).expect("the section declares hazard");
    assert!(frame.vocabulary.accepts_paper("hazard"));
    assert_eq!(frame.fills[0].paper, "hazard");
}

#[test]
fn without_the_section_the_declared_set_is_the_formats_own() {
    let document = format!(
        "[vocabulary]\n\
         ink\taccent\n\
         \n\
         {}",
        base()
    );
    // The section replaces the set, so the format's `text` ink is no longer declared.
    assert_eq!(code(&document), "cellframe-vocabulary");
}

#[test]
fn an_unterminated_vocabulary_section_is_refused() {
    let document = format!("[vocabulary]\npaper\tsurface\n{}", base());
    assert_eq!(code(&document), "cellframe-row");
}

#[test]
fn a_frame_fits_its_window_or_it_is_refused() {
    let frame = parse(&base()).unwrap();
    assert_eq!(check_extent(&frame, "map-plan", [56, 22]), Ok(()));
    match check_extent(&frame, "map-plan", [42, 18]) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "cellframe-extent");
            assert!(refusal.message().contains("map-plan"), "{}", refusal.message());
        }
        Ok(()) => panic!("a frame wider than its window must be refused"),
    }
    assert_eq!(
        check_extent(&frame, "taskbar", [56, 22])
            .unwrap_err()
            .code(),
        "cellframe-extent"
    );
}
