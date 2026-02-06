use std::sync::{Arc, LazyLock};

use askama::filters::HtmlSafeOutput;
use bbscope::{BBCode, BBCodeTagConfig, EmitScope, MatchInfo, MatchType, ScopeInfo};

static BBCODE: LazyLock<BBCode> = LazyLock::new(|| {
    let mut matchers = vec![MatchInfo {
        id: "hr",
        regex: r#"^\[hr\]"#.try_into().expect("create hr regex"),
        match_type: MatchType::Simple(Arc::new(|_c| "<hr>".into())),
    }];

    add_span_style_matcher(&mut matchers, "color", "color");
    add_span_style_matcher(&mut matchers, "align", "text-align");
    add_span_style_matcher(&mut matchers, "size", "font-size");
    add_span_style_matcher(&mut matchers, "font", "font-family");

    BBCode::from_config(BBCodeTagConfig::extended(), Some(matchers))
        .expect("create bbcode instance")
});

#[askama::filter_fn]
pub fn bbcode(
    input: &dyn std::fmt::Display,
    _: &dyn askama::Values,
) -> askama::Result<HtmlSafeOutput<String>> {
    let bbcode = &*BBCODE;

    let html = bbcode.parse(&input.to_string());

    Ok(HtmlSafeOutput(html))
}

fn add_span_style_matcher(
    matchers: &mut Vec<MatchInfo>,
    bb_key: &'static str,
    css_key: &'static str,
) {
    let emitter: EmitScope = Arc::new(move |open_capture, body, _c| {
        if let Some(font) = open_capture.and_then(|x| x.name("attr")) {
            format!(r#"<span style="{css_key}:{}">{body}</span>"#, font.as_str(),)
        } else {
            body.to_string()
        }
    });

    BBCode::add_tagmatcher(matchers, bb_key, ScopeInfo::basic(emitter), None, None)
        .expect("add tagmatcher");
}
