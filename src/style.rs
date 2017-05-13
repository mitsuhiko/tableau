use console::Style;

/// The table rendering style.
pub struct TableStyle {
    pub default_header_style: Style,
    pub default_text_style: Style,
    pub top: Option<char>,
    pub top_mid: Option<char>,
    pub top_left: Option<char>,
    pub top_right: Option<char>,
    pub bottom: Option<char>,
    pub bottom_mid: Option<char>,
    pub bottom_left: Option<char>,
    pub bottom_right: Option<char>,
    pub mid: Option<char>,
    pub mid_left: Option<char>,
    pub mid_mid: Option<char>,
    pub mid_right: Option<char>,
    pub left: Option<char>,
    pub middle: Option<char>,
    pub right: Option<char>,
    pub head: Option<char>,
    pub head_mid: Option<char>,
    pub head_left: Option<char>,
    pub head_right: Option<char>,
    pub truncate_str: Option<String>,
    pub padding_left: u8,
    pub padding_right: u8,
    _more: (),
}

impl Default for TableStyle {
    fn default() -> TableStyle {
        TableStyle {
            default_header_style: Style::new().bold(),
            default_text_style: Style::new(),
            top: Some('-'),
            top_mid: Some('+'),
            top_left: Some('+'),
            top_right: Some('+'),
            bottom: Some('-'),
            bottom_mid: Some('+'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
            left: Some('|'),
            mid_left: None,
            mid: None,
            mid_mid: None,
            right: Some('|'),
            mid_right: None,
            middle: Some('|'),
            head: Some('-'),
            head_mid: Some('+'),
            head_left: Some('+'),
            head_right: Some('+'),
            truncate_str: Some("...".into()),
            padding_left: 1,
            padding_right: 1,
            _more: (),
        }
    }
}

impl TableStyle {
    fn full_unicode() -> TableStyle {
        TableStyle {
            top: Some('─'),
            top_mid: Some('┬'),
            top_left: Some('┌'),
            top_right: Some('┐'),
            bottom: Some('─'),
            bottom_mid: Some('┴'),
            bottom_left: Some('└'),
            bottom_right: Some('┘'),
            left: Some('│'),
            mid_left: Some('├'),
            mid: Some('─'),
            mid_mid: Some('┼'),
            right: Some('│'),
            mid_right: Some('┤'),
            middle: Some('│'),
            head: Some('─'),
            head_mid: Some('┬'),
            head_left: Some('┌'),
            head_right: Some('┐'),
            truncate_str: Some("…".into()),
            ..Default::default()
        }
    }

    fn full_ascii() -> TableStyle {
        TableStyle {
            top: Some('-'),
            top_mid: Some('+'),
            top_left: Some('+'),
            top_right: Some('+'),
            bottom: Some('-'),
            bottom_mid: Some('+'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
            left: Some('|'),
            mid_left: Some('+'),
            mid: Some('-'),
            mid_mid: Some('+'),
            right: Some('|'),
            mid_right: Some('+'),
            middle: Some('|'),
            head: Some('='),
            head_mid: Some('+'),
            head_left: Some('+'),
            head_right: Some('+'),
            ..Default::default()
        }
    }

    fn compact_ascii() -> TableStyle {
        TableStyle {
            top: Some('-'),
            top_mid: Some('+'),
            top_left: Some('+'),
            top_right: Some('+'),
            bottom: Some('-'),
            bottom_mid: Some('+'),
            bottom_left: Some('+'),
            bottom_right: Some('+'),
            left: Some('|'),
            mid_left: None,
            mid: None,
            mid_mid: None,
            right: Some('|'),
            mid_right: None,
            middle: Some('|'),
            head: Some('-'),
            head_mid: Some('+'),
            head_left: Some('+'),
            head_right: Some('+'),
            ..Default::default()
        }
    }
}
