use macroquad::prelude::*;

enum Tool {
    Pen,
    Eraser,
}

#[macroquad::main("Blackboard")]
async fn main() {
    let mut strokes: Vec<(Vec<(f32, f32)>, Color, f32)> = vec![];
    let mut current_stroke: Vec<(f32, f32)> = vec![];
    let mut tool = Tool::Pen;
    let mut brush_size: f32 = 4.0;
    let board_color = BLACK;

    loop {
        clear_background(board_color);

        // input
        if is_key_pressed(KeyCode::P) {
            tool = Tool::Pen;
        }
        if is_key_pressed(KeyCode::E) {
            tool = Tool::Eraser;
        }
        if is_key_pressed(KeyCode::C) {
            strokes.clear();
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Equal) {
            brush_size = (brush_size + 2.0).min(50.0);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Minus) {
            brush_size = (brush_size - 2.0).max(1.0);
        }

        // drawing
        if is_mouse_button_down(MouseButton::Left) {
            current_stroke.push(mouse_position());
        } else if !current_stroke.is_empty() {
            let color = match tool {
                Tool::Pen => WHITE,
                Tool::Eraser => board_color,
            };
            strokes.push((std::mem::take(&mut current_stroke), color, brush_size));
        }

        // render strokes
        for (stroke, color, size) in &strokes {
            draw_stroke(stroke, *color, *size);
        }

        // render current stroke
        let current_color = match tool {
            Tool::Pen => WHITE,
            Tool::Eraser => board_color,
        };
        draw_stroke(&current_stroke, current_color, brush_size);

        // cursor preview
        let (mx, my) = mouse_position();
        draw_circle_lines(mx, my, brush_size / 2.0, 1.0, GRAY);

        // HUD
        let tool_name = match tool {
            Tool::Pen => "Pen",
            Tool::Eraser => "Eraser",
        };
        draw_text(&format!("[P]en  [E]raser  [C]lear  Size: {} (+/-)", brush_size), 10.0, 25.0, 20.0, BEIGE);
        draw_text(&format!("Tool: {}", tool_name), 10.0, 50.0, 20.0, BEIGE);

        next_frame().await
    }
}

fn draw_stroke(points: &[(f32, f32)], color: Color, size: f32) {
    if points.len() == 1 {
        draw_circle(points[0].0, points[0].1, size / 2.0, color);
    }
    for w in points.windows(2) {
        draw_line(w[0].0, w[0].1, w[1].0, w[1].1, size, color);
        draw_circle(w[1].0, w[1].1, size / 2.0, color);
    }
}
