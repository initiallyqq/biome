use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::CssFileSource;

const SCSS_VARIABLE_DECLARATION: &str = "$color: red;";
const SCSS_VARIABLE_VALUE: &str = ".selector { color: $color; }";

fn diagnostic_text(parse: &biome_css_parser::CssParse) -> String {
    format!("{:?}", parse.diagnostics())
}

fn expect_scss_diagnostics(source: &str, expected_message: &str, options: CssParserOptions) {
    let parse = parse_css(source, CssFileSource::css(), options);
    let diagnostics = diagnostic_text(&parse);

    assert!(
        diagnostics.contains(expected_message),
        "expected an SCSS-specific diagnostic in CSS parser-reporting builds, got: {diagnostics}",
    );
}

#[test]
fn css_files_do_not_report_scss_exclusive_syntax_without_parser_option() {
    for source in [SCSS_VARIABLE_DECLARATION, SCSS_VARIABLE_VALUE] {
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let diagnostics = diagnostic_text(&parse);

        assert!(
            !diagnostics.contains("SCSS"),
            "expected no SCSS-specific diagnostics without the parser option, got: {diagnostics}"
        );
        assert!(
            !parse.diagnostics().is_empty(),
            "expected parsing to keep reporting invalid CSS syntax"
        );
    }
}

#[test]
fn css_files_report_scss_exclusive_syntax_when_enabled_by_parser_options() {
    expect_scss_diagnostics(
        SCSS_VARIABLE_DECLARATION,
        "SCSS variable declarations",
        CssParserOptions::default().report_scss_exclusive_syntax(),
    );

    expect_scss_diagnostics(
        SCSS_VARIABLE_VALUE,
        "SCSS variables",
        CssParserOptions::default().report_scss_exclusive_syntax(),
    );
}

#[test]
fn scss_files_parse_scss_exclusive_syntax_without_reporting_it_as_css_error() {
    let parse = parse_css(
        SCSS_VARIABLE_DECLARATION,
        CssFileSource::scss(),
        CssParserOptions::default(),
    );
    let diagnostics = diagnostic_text(&parse);

    assert!(
        !diagnostics.contains("SCSS only feature"),
        "expected SCSS parsing to accept SCSS syntax, got: {diagnostics}"
    );
}
