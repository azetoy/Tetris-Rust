use piston_window::{WindowSettings, PistonWindow, Event, RenderEvent, PressEvent};
use piston_window::{Rectangle, DrawState, Context, Graphics};
use piston_window::{Button, Key};

fn main() {
    let taille_cube = 2;
    let taille_x = 8;
    let taille_y= 20;
    let bg = [0.0,0.0,0.0,1.0];

    let mut window: PistonWindow = WindowSettings::new("Tetris",[taille_x*taille_cube,taille_y*taille_cube])
            .exit_on_esc(true).build().unwrap();
    let mut events = window.events;

    while let Some(e) = window.next(){
        if let Some(_)= e.render_args(){
            window.draw_2d(&e,|c,g,_|{
            });
        }
    } 
} 