use std::sync::LazyLock;

use askama::filters::HtmlSafeOutput;
use bbscope::BBCode;

static BBCODE: LazyLock<BBCode> =
    LazyLock::new(|| BBCode::default().expect("create bbcode instance"));

#[askama::filter_fn]
pub fn bbcode(
    input: &dyn std::fmt::Display,
    _: &dyn askama::Values,
) -> askama::Result<HtmlSafeOutput<String>> {
    let bbcode = &*BBCODE;

    let html = bbcode.parse(&input.to_string());

    Ok(HtmlSafeOutput(html))
}
