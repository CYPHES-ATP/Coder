//! CYPHES identity mark rendering for ratatui.
//!
//! The original module name is retained so the runtime wiring stays stable for
//! Phase 1, but the visible art now matches the CYPHES person-silhouette mark.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// The pose / expression of the Rustle mascot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustlePose {
    Default,
    ArmsUp,
    LookLeft,
    LookRight,
    LookDown,
    /// Loading / error spinner — `frame` drives the animation.
    Loading { frame: u64 },
}

/// Mark style: bold CYPHES cyan foreground.
fn body_style() -> Style {
    Style::default()
        .fg(Color::Rgb(0, 246, 255))
        .add_modifier(Modifier::BOLD)
}

fn silhouette_style() -> Style {
    Style::default()
        .fg(Color::Rgb(245, 251, 250))
        .add_modifier(Modifier::BOLD)
}

fn signal_style() -> Style {
    Style::default()
        .fg(Color::Rgb(199, 255, 71))
        .add_modifier(Modifier::BOLD)
}

fn pulse_glyph(frame: u64) -> &'static str {
    const GLYPHS: [&str; 4] = ["●", "◐", "○", "◑"];
    GLYPHS[(frame / 5) as usize % GLYPHS.len()]
}

/// Returns 5 Lines representing the CYPHES profile mark.
pub fn rustle_lines(pose: &RustlePose) -> [Line<'static>; 5] {
    let signal = match pose {
        RustlePose::Loading { frame } => pulse_glyph(*frame),
        RustlePose::LookLeft => "◖",
        RustlePose::LookRight => "◗",
        RustlePose::LookDown => "●",
        RustlePose::ArmsUp => "◆",
        RustlePose::Default => "●",
    };

    let row1 = Line::from(vec![Span::styled("    ▄▄▄    ".to_string(), silhouette_style())]);
    let row2 = Line::from(vec![
        Span::styled("   █".to_string(), silhouette_style()),
        Span::styled(signal.to_string(), signal_style()),
        Span::styled("███   ".to_string(), silhouette_style()),
    ]);
    let row3 = Line::from(vec![Span::styled("    ▀▀▀    ".to_string(), silhouette_style())]);
    let row4 = Line::from(vec![Span::styled("  ▄█████▄  ".to_string(), body_style())]);
    let row5 = Line::from("");

    [row1, row2, row3, row4, row5]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_text(line: &Line<'_>) -> String {
        line.spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<Vec<_>>()
            .join("")
    }

    #[test]
    fn default_pose_uses_cyphes_profile_mark() {
        let lines = rustle_lines(&RustlePose::Default);
        assert_eq!(line_text(&lines[0]), "    ▄▄▄    ");
        assert_eq!(line_text(&lines[1]), "   █●███   ");
        assert_eq!(line_text(&lines[3]), "  ▄█████▄  ");
    }

    #[test]
    fn arms_up_pose_keeps_cyphes_profile_shape() {
        let lines = rustle_lines(&RustlePose::ArmsUp);
        assert_eq!(line_text(&lines[1]), "   █◆███   ");
    }

    #[test]
    fn look_right_pose_keeps_cyphes_profile_shape() {
        let lines = rustle_lines(&RustlePose::LookRight);
        assert_eq!(line_text(&lines[1]), "   █◗███   ");
    }
}
