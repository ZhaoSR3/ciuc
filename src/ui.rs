use image::RgbaImage;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::canvas::{Canvas, Context},
};

use crate::{
    image::{get_pixel, resize_image},
    models::{CellPixel, MaintainOrFill, Model},
};

// 上半块字符的Unicode
const UPPER_HALF_BLOCK: &str = "\u{2580}";

// 下半块字符的Unicode
const LOWER_HALF_BLOCK: &str = "\u{2584}";

fn make_cell(image: &RgbaImage, x: u32, y: u32) -> CellPixel {
    let top_pixel = get_pixel(image, x, y);
    let bottom_pixel = if y + 1 < image.height() {
        Some(get_pixel(image, x, y + 1))
    } else {
        None
    };

    let mut cell_pixel = CellPixel::new();

    // 阀值二值化
    if top_pixel.0[3] >= 128 {
        // 不透明
        let [r, g, b, _] = top_pixel.0;
        cell_pixel.top = Some(Color::Rgb(r, g, b));
    }

    match bottom_pixel {
        Some(pixel) => {
            if pixel.0[3] >= 128 {
                let [r, g, b, _] = pixel.0;
                cell_pixel.bottom = Some(Color::Rgb(r, g, b));
            }
        }
        None => {}
    }

    cell_pixel
}

fn draw_cell(ctx: &mut Context, cell_pixel: CellPixel, x: u32, y: u32) {
    match (cell_pixel.top, cell_pixel.bottom) {
        (Some(top), Some(bottom)) => {
            ctx.print(
                x as f64,
                y as f64,
                Span::styled(
                    UPPER_HALF_BLOCK.to_string(),
                    Style::default().fg(top).bg(bottom),
                ),
            );
        }

        (Some(top), None) => {
            ctx.print(
                x as f64,
                y as f64,
                Span::styled(UPPER_HALF_BLOCK.to_string(), Style::default().fg(top)),
            );
        }

        (None, Some(bottom)) => {
            ctx.print(
                x as f64,
                y as f64,
                Span::styled(LOWER_HALF_BLOCK.to_string(), Style::default().fg(bottom)),
            );
        }

        (None, None) => {}
    }
}

fn fit_size(
    original_width: u32,
    original_height: u32,
    area_width: u16,
    area_height: u16,
) -> (u32, u32) {
    let target_width = u32::from(area_width);
    let target_height = u32::from(area_height) * 2;

    let scale = (target_width as f64 / original_width as f64)
        .min(target_height as f64 / original_height as f64);

    let new_width = ((original_width as f64 * scale).floor() as u32).max(1);
    let new_height = ((original_height as f64 * scale).floor() as u32).max(1);

    (new_width, new_height)
}

fn render(model: &mut Model, frame: &mut Frame) {
    let image = match &model.sprite_frame {
        Some(sf) => &sf.image,
        None => match &mut model.sprite_frames {
            Some(sfs) => &sfs.update().image,
            None => {
                panic!("Not a single sprite frame.")
            }
        },
    };

    let frame_area = frame.area();
    let image_width = image.width();
    let image_height = image.height();
    let frame_area_width = frame_area.width;
    let frame_area_height = frame_area.height;

    if image_width == 0 || image_height == 0 || frame_area_width == 0 || frame_area_height == 0 {
        panic!("The dimensions of the image or display area are incorrect.");
    }

    let (width, height) = match model.maintain_or_fill {
        MaintainOrFill::Maintain => fit_size(
            image_width,
            image_height,
            frame_area_width,
            frame_area_height,
        ),
        MaintainOrFill::Fill => (
            u32::from(frame_area_width),
            u32::from(frame_area_height) * 2,
        ),
    };

    let reimage = resize_image(image, width, height);

    let reimage_width = reimage.width();
    let reimage_height = reimage.height();
    let reheight = reimage_height.div_ceil(2);

    let canvas_width = reimage_width as u16;
    let canvas_height = reheight as u16;
    let rect_x = frame_area.x + (frame_area_width - canvas_width) / 2;
    let rect_y = frame_area.y + (frame_area_height - canvas_height) / 2;
    let canvas_area = Rect::new(rect_x, rect_y, canvas_width, canvas_height);

    let canvas = Canvas::default()
        .x_bounds([0.0, reimage_width.saturating_sub(1).max(1) as f64])
        // y需要向上取整
        .y_bounds([0.0, reheight.saturating_sub(1).max(1) as f64])
        .paint(|ctx| {
            for y in (0..reimage_height).step_by(2) {
                for x in 0..reimage_width {
                    let cell_pixel = make_cell(&reimage, x, y);
                    // 翻转y轴
                    let canvas_y = reheight - 1 - y.div_ceil(2);
                    draw_cell(ctx, cell_pixel, x, canvas_y);
                }
            }
        });

    frame.render_widget(canvas, canvas_area);
}

pub fn view(model: &mut Model, frame: &mut Frame) {
    render(model, frame);
}
