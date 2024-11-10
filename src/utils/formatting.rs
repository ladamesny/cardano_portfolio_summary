use ratatui::{
    style::{Style, Color},
    text::Span,
};

/// Formats a number with comma separators and specified decimal places
pub fn format_number(value: f64, decimals: usize) -> String {
    let whole = value.trunc() as i64;
    let decimal = (value.fract() * 10f64.powi(decimals as i32)).abs().round();
    
    let whole_formatted = whole.to_string()
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();
        
    if decimals > 0 {
        format!("{}.{:0width$}", whole_formatted, decimal as i64, width = decimals)
    } else {
        whole_formatted
    }
}

/// Formats an ADA value with the ₳ symbol and specified decimal places
pub fn format_ada(value: f64, decimals: usize) -> String {
    format!("₳{}", format_number(value, decimals))
}

pub fn format_usd(value: f64, decimals: usize) -> String {
    format!("${}", format_number(value, decimals))
}

/// Formats a percentage change with colors and +/- symbol
pub fn format_change(change: f64) -> Span<'static> {
    let formatted = format!("{:+.2}%", change);
    let color = if change > 0.0 {
        Color::Green
    } else if change < 0.0 {
        Color::Red
    } else {
        Color::White
    };
    
    Span::styled(formatted, Style::default().fg(color))
} 