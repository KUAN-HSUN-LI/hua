use crate::{backend::base::DrawingBase, color::RGBAColor};

pub fn draw_rect<DB: DrawingBase>(
    canvas: &mut DB,
    upper_left: (u32, u32),
    bottom_right: (u32, u32),
    fill: Option<RGBAColor>,
) {
    let (w, _) = canvas.get_size();
    let (x0, y0) = upper_left;
    let (x1, y1) = bottom_right;
    let buf = canvas.borrow_mut_buf();

    if let Some(color) = fill {
        for y in y0..y1 {
            let start = (y * w + x0) as usize;
            let end = (y * w + x1) as usize;
            buf[start..end].iter_mut().for_each(|c| *c = color);
        }
    } else {
        // draw outline
        todo!()
    }
}
