

use raylib::prelude::*;
use async_std::{
    channel::{self},
    task,
};
use winit::{
    event::{
        ButtonId, DeviceEvent, ElementState, Event, KeyboardInput, MouseScrollDelta, WindowEvent, VirtualKeyCode },
    event_loop::{ControlFlow, EventLoop},
    window::Window,
 };

pub(crate) enum HandledEvent {
    Keyboard(KeyboardInput),
    // Maybe used in the future 
    /*MouseButton {
        button: ButtonId,
        state: ElementState,
    },
    MouseScroll(MouseScrollDelta),*/ 
}


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

    let event_loop = EventLoop::new(); 
    let window = Window::new(&event_loop)
        .expect("Could not create x11 window"); 
    window.set_visible(false); 
    
    let (tx, _) = channel::unbounded(); 
    while !rl.window_should_close() {
        let mut message = String::from("");  
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            let tx = tx.clone();
            match event {
                Event::DeviceEvent { event: DeviceEvent::Key(key), .. } => {
                    let current_key = key.clone();
                    task::spawn(async move { tx.send(HandledEvent::Keyboard(key)).await }); 
                    if current_key.state == ElementState::Pressed {
                        return (); 
                    }
                    let code  = current_key.virtual_keycode.unwrap_or(VirtualKeyCode::Space); 

                    // println!("found code: {:?} -> {:?}", code, key); 
                    let key = format!("{:?}", code);
                    
                    match key.as_str() {
                        "A" => {
                            index = 14; 
                            message.push_str("a"); 
                        } 
                        "B" => {
                            index = 2; 
                            message.push_str("b"); 
                        } 
                        "C" => {
                            index = 3; 
                            message.push_str("c"); 
                        } 
                        "D" => {
                            index = 4;  
                            message.push_str("d"); 
                        }
                        "E" => {
                            index = 5; 
                            message.push_str("e"); 
                        }
                        "F" => {
                            index = 6;  
                            message.push_str("f"); 
                        } 
                        "G" => {
                            index = 7;  
                            message.push_str("g"); 
                        } 
                        "H" => {
                            index = 8; 
                            message.push_str("h"); 
                        } 
                        "I" => {
                            index = 9;
                            message.push_str("i"); 
                        } 
                        "J" => {
                            index = 10; 
                            message.push_str("j"); 
                        }
                        "K" => {
                            index = 11; 
                            message.push_str("k"); 
                        }
                        "L" => {
                            index = 12; 
                            message.push_str("l"); 
                        } 
                        "M" => {
                            index = 13; 
                            message.push_str("m"); 
                        } 
                        "N" => {
                            index = 14; 
                            message.push_str("n"); 
                        }
                        "O" => {
                            index = 14; 
                            message.push_str("o"); 
                        } 
                        "P" => {
                            index = 2; 
                            message.push_str("p"); 
                        }
                        "Q" => {
                            index = 3; 
                            message.push_str("q"); 
                        }
                        "R" => {
                            index = 4;  
                            message.push_str("r"); 
                        } 
                        "S" => {
                            index = 5;
                            message.push_str("s"); 
                        }
                        "T" => {
                            index = 6;  
                            message.push_str("t"); 
                        }
                        "U" => {
                            index = 7; 
                            message.push_str("u"); 
                        }
                        "V" => {
                            index = 8;
                            message.push_str("v"); 
                        } 
                        "W" => {
                            index = 9; 
                            message.push_str("w"); 
                        }
                        "X" => {
                            index = 10; 
                            message.push_str("x"); 
                        }
                        "Y" => {
                            index = 11; 
                            message.push_str("y"); 
                        }
                        "Z" => {
                            index = 12;  
                            message.push_str("z"); 
                        } 
                       "Return" => {
                            index = 13; 
                            message.push_str("[e]"); 
                        }
                       "Space" => {
                            index = 14; 
                            message.push_str("[_]"); 
                       }
                       "Escape" => {
                            index = 1; 
                            message.push_str("<esc>"); 
                       }
                       "Left" => {
                            index = 2;
                            message.push_str("<left>");
                       }
                       "Right" => {
                            index = 3; 
                            message.push_str("<right>"); 
                       }
                       "Up" => {
                            index = 4;
                            message.push_str("<Up>"); 
                       }
                       "Down" => {
                            index = 5;
                            message.push_str("<down>"); 
                       }
                       "Semicolon" => {
                            index = 6;
                            message.push_str(";");
                       }
                       "Period" => {
                           index = 7; 
                           message.push_str("."); 
                       }
                       "Apostrophe" => {
                            index = 8; 
                            message.push_str("'"); 
                       }
                       "Tab" => {
                            index = 9; 
                            message.push_str("[TAB]"); 
                       }
                       "LBracket" => {
                            index = 10; 
                            message.push_str("["); 
                       }
                       "RBracket" => {
                            index = 11;
                            message.push_str("]");
                       }
                       "Back" => {
                            index = 12; 
                            message.push_str("[BACK]"); 
                       }
                       "Key1" => {
                            index = 13; 
                            message.push_str("1"); 
                       }
                       "key2" => {
                            index = 14; 
                            message.push_str("2"); 
                       }
                       "Key3" => {
                            index = 1;
                            message.push_str("3"); 
                       }
                       "Key4" => {
                            index = 2; 
                            message.push_str("4"); 
                       }
                       "Key5" => {
                            index = 3; 
                            message.push_str("5"); 
                       }
                       "Key6" => {
                            index = 4; 
                            message.push_str("6"); 
                       }
                       "Key7" => {
                            index = 5; 
                            message.push_str("7"); 
                       }
                       "key8" => {
                            index = 6;
                            message.push_str("8"); 
                       }
                       "Key9" => {
                            index = 7;
                            message.push_str("9"); 
                       }
                        _ => {
                            println!("unhandled key: {}", key);  
                        }
                    }

                }
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    *control_flow = ControlFlow::Exit; 
                    println!("Exit pressed"); 
                }
                _ => {
                    message = String::from(""); 
                }
            }

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
           
           });    
    }
}


