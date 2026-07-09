//! SVG backend for vista.

use foundry::Result;
use std::fs::File;
use std::io::Write;

/// Render a list of points as an SVG polyline.
pub fn render_polyline(points: &[(f64, f64)], width: u32, height: u32, path: &str) -> Result<()> {
    if points.is_empty() {
        return Err(foundry::Error::InvalidArgument("no points to render".into()));
    }
    let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
    let dx = (max_x - min_x).max(1e-12);
    let dy = (max_y - min_y).max(1e-12);
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width, height, width, height
    );
    svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);
    let mut d = String::new();
    for (i, p) in points.iter().enumerate() {
        let px = ((p.0 - min_x) / dx) * (width as f64 - 40.0) + 20.0;
        let py = height as f64 - 20.0 - ((p.1 - min_y) / dy) * (height as f64 - 40.0);
        if i == 0 {
            d.push_str(&format!("M {:.1},{:.1} ", px, py));
        } else {
            d.push_str(&format!("L {:.1},{:.1} ", px, py));
        }
    }
    svg.push_str(&format!(
        r#"<path d="{}" fill="none" stroke="steelblue" stroke-width="2"/>"#,
        d
    ));
    svg.push_str("</svg>");
    let mut file = File::create(path).map_err(|e| foundry::Error::Io(e.to_string()))?;
    file.write_all(svg.as_bytes())
        .map_err(|e| foundry::Error::Io(e.to_string()))?;
    Ok(())
}
