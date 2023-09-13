

use raylib::prelude::*;

fn main() {
    let width = 200;
    let height = 200; 
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("DAMNYOUTYPESOFASTAGEAN")
        .build();

    let mut images = vec![]; 
    for index in 1..=14 {
        let mut filename = String::from("images/frame"); 
        filename.push_str(index.to_string().as_str()); 
        filename.push_str(".png"); 
        let texture = rl
            .load_texture(&thread, filename.as_str()).
            expect(format!("Could not load image {}", filename).as_str());
        images.push(texture); 
    }


    let first = images[0].as_ref();
     
    let frame_width = first.width as f32; 
    let frame_height = first.height as f32; 
    let scale = 0.1; 
    let src_rect = ffi::Rectangle {x: 0.0, y: 0.0 , width: frame_width, height:  frame_height };
    let dest_rect = ffi::Rectangle {x: (width / 2) as f32 , y: (height / 2) as f32, width: frame_width / 5.0, height: frame_height / 5.0 };
        
    
    // let origin = ffi::Vector2 {x: (frame_width * scale) as f32,  y: (frame_height * scale) as f32}; 
    let origin = ffi::Vector2 {x: (frame_width * scale) as f32,  y: (frame_height * scale) as f32}; 
    

    rl.set_target_fps(60); 
    let mut index = 0;
    let mut display = String::from("["); 
    let mut text_x = width / 2;


    while !rl.window_should_close() {
        let mut message = String::from("");  
        if let Some(key) = rl.get_key_pressed() {
            use raylib::consts::KeyboardKey;  
            match key {
                KeyboardKey::KEY_A => {
                    index = 1; 
                    message.push_str("a"); 
                } 
                KeyboardKey::KEY_B => {
                    index = 2; 
                    message.push_str("b"); 
                } 
                KeyboardKey::KEY_C => {
                    index = 3; 
                    message.push_str("c"); 
                } 
                KeyboardKey::KEY_D => {
                    index = 4;  
                    message.push_str("d"); 
                }
                KeyboardKey::KEY_E => {
                    index = 5; 
                    message.push_str("e"); 
                }
                KeyboardKey::KEY_F => {
                    index = 6;  
                    message.push_str("f"); 
                } 
                KeyboardKey::KEY_G => {
                    index = 7;  
                    message.push_str("g"); 
                } 
                KeyboardKey::KEY_H => {
                    index = 8; 
                    message.push_str("h"); 
                } 
                KeyboardKey::KEY_I => {
                    index = 9;
                    message.push_str("i"); 
                } 
                KeyboardKey::KEY_J => {
                    index = 10; 
                    message.push_str("j"); 
                }
                KeyboardKey::KEY_K => {
                    index = 11; 
                    message.push_str("k"); 
                }
                KeyboardKey::KEY_L => {
                    index = 12; 
                    message.push_str("l"); 
                } 
                KeyboardKey::KEY_M => {
                    index = 13; 
                    message.push_str("m"); 
                } 
                KeyboardKey::KEY_N => {
                    index = 14; 
                    message.push_str("n"); 
                }
                KeyboardKey::KEY_O => {
                    index = 14; 
                    message.push_str("o"); 
                } 
                KeyboardKey::KEY_P => {
                    index = 2; 
                    message.push_str("p"); 
                }
                KeyboardKey::KEY_Q => {
                    index = 3; 
                    message.push_str("q"); 
                }
                KeyboardKey::KEY_R => {
                    index = 4;  
                    message.push_str("r"); 
                } 
                KeyboardKey::KEY_S => {
                    index = 5;
                    message.push_str("s"); 
                }
                KeyboardKey::KEY_T => {
                    index = 6;  
                    message.push_str("t"); 
                }
                KeyboardKey::KEY_U => {
                    index = 7; 
                    message.push_str("u"); 
                } 
                KeyboardKey::KEY_V => {
                    index = 8;
                    message.push_str("v"); 
                } 
                KeyboardKey::KEY_W => {
                    index = 9; 
                    message.push_str("w"); 
                }
                KeyboardKey::KEY_X => {
                    index = 10; 
                    message.push_str("x"); 
                }
                KeyboardKey::KEY_Y => {
                    index = 11; 
                    message.push_str("y"); 
                }
                KeyboardKey::KEY_Z => {
                    index = 12;  
                    message.push_str("z"); 
                } 
                KeyboardKey::KEY_SPACE => {
                    index = 13; 
                    message.push_str("[_]"); 
                }

               _ => {}
            } 
        }
        
        /* if message.size() > 10 {
            display.push_str(message); 
        } */

        if index > 0 {
            index = index - 1
        }
 
        if text_x < 0 {
            text_x = width / 2;
            display = "".to_string(); 
        }
        
        if !message.is_empty() {
            display.push_str(message.as_str()); 
            text_x -= 5; 
        }
       let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        d.draw_texture_pro(&images[index], src_rect, dest_rect, origin, 0.0, Color::WHITE); 
        d.draw_text(display.as_str(), text_x , height - 20 , 16, Color::WHITE); 
        //d.draw_texture(&images[0], 640/ 2, 480/ 2, Color::WHITE); 
    }
}


