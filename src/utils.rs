use opengl_graphics::GlGraphics;
use graphics::*;

pub fn draw_frame(x: f64, y: f64, width: f64, height: f64, bg_color: [f32; 4], fg_color: [f32; 4], border_width: f64, c: Context, gl: &mut GlGraphics) {
    rectangle(
        bg_color,
        [
            0.0,
            0.0,
            width + border_width * 2.0,
            height + border_width * 2.0,
        ],
        c.transform.trans(x - border_width, y - border_width),
        gl,
    );
    rectangle(
        fg_color,
        [
            0.0,
            0.0,
            width,
            height,
        ],
        c.transform.trans(x, y),
        gl,
    );
}

pub fn draw_tetromino(x: f64, y: f64, shape: u16, color: [f32; 4], c: Context, gl: &mut GlGraphics) {
    for y_l in 0..4 {
        for x_l in 0..4 {
            // Some bitwise fuckery to check if current xy has block or not
            if shape & (0x8000 >> (y_l * 4 + x_l)) != 0 {
                // rectangle(
                //     color,
                //     [
                //         x_l as f64 * 20.0,
                //         y_l as f64 * 20.0,
                //         20.0,
                //         20.0,
                //     ],
                //     c.transform.trans(x, y),
                //     gl,
                // );
                draw_block(x_l as f64 * 20.0 + x, y_l as f64 * 20.0 + y, color, c, gl);
            }
        }
    }
}

pub fn draw_block(x: f64, y: f64, color: [f32; 4], c: Context, gl: &mut GlGraphics) {
    polygon(
        brightness(color, 1.2),
        &[
            [0.0, 0.0],
            [20.0, 0.0],
            [16.0, 4.0],
            [4.0, 4.0],
            [4.0, 16.0],
            [0.0, 20.0]
        ],
        c.transform.trans(x, y),
        gl
    );
    polygon(
        brightness(color, 0.8),
        &[
            [20.0, 20.0],
            [0.0, 20.0],
            [4.0, 16.0],
            [16.0, 16.0],
            [16.0, 4.0],
            [20.0, 0.0]
        ],
        c.transform.trans(x, y),
        gl
    );
    rectangle(
        color,
        [4.0, 4.0, 12.0, 12.0],
        c.transform.trans(x, y),
        gl
    );
}

fn brightness(color: [f32; 4], brightness: f32) -> [f32; 4] {
    [color[0] * brightness, color[1] * brightness, color[2] * brightness, color[3]]
}
