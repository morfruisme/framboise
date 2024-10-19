// disable console showing up on windows
#![windows_subsystem = "windows"]

use raylib::prelude::*;

fn main() {
    const W: i32 = 800;
    const H: i32 = 450;

    let color: math::Vector4 = Color::PURPLE.into();
    let line_width = 10.;
    
    let (mut rl, thread) = raylib::init()
        .title("")
        .size(W, H)
        .resizable()
        .build();
    //rl.set_target_fps(60);

    let mut frame_buffer = rl.load_render_texture(&thread, W as u32, H as u32).unwrap();
    let mut fs = rl.load_shader(&thread, None, Some("src/shader/frag.fs")).unwrap();

    // uniforms
    let dim_loc = fs.get_shader_location("dim");
    let color_loc = fs.get_shader_location("line_color");
    let lw_loc = fs.get_shader_location("line_width2");

    let img_loc = fs.get_shader_location("tex");
    let p_loc = fs.get_shader_location("p");
    let delta_loc = fs.get_shader_location("delta");

    fs.set_shader_value(dim_loc, Vector2 { x: W as f32, y: H as f32 });
    fs.set_shader_value(color_loc, color);
    fs.set_shader_value(lw_loc, line_width*line_width);
    
    while !rl.window_should_close() {
        let tex = frame_buffer.texture().clone();
        fs.set_shader_value_texture(img_loc, &tex);
        
        let p = rl.get_mouse_position();
        let b = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT); 
        fs.set_shader_value(p_loc, p);
        fs.set_shader_value(delta_loc, rl.get_mouse_delta());
        
        let mut d = rl.begin_drawing(&thread);
        d.draw_texture(&tex, 0, 0, Color::WHITE);
        {
            let mut d = d.begin_texture_mode(&thread, &mut frame_buffer);
            if b {
                let mut d = d.begin_shader_mode(&fs);
                d.draw_rectangle(0, 0, W, H, Color::WHITE);
            }
        }

    }
}