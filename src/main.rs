extern crate  piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs,
                   UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::Transformed;
use graphics::rectangle;




 

struct Game
{
    gl: GlGraphics,
    pos_x: f64,
    pos_y: f64,

}

impl Game
{
    fn render(&mut self,arg: &RenderArgs)
    {
        use graphics;
        
        let BACKGROUND: [f32; 4] = [0.349019608 , 0.349019608 , 0.290196078 , 1.0];
        let COO: [f32;4] = [0.364705882, 0.717647059, 0.870588235, 0.8];

        let square_c = rectangle::square(100.0, 100.0, 30.0);
        let pos_x = self.pos_x as f64;
        let pos_y = self.pos_y as f64;

        println!("pos x = {}",pos_x);
        println!("pos y = {}",pos_y);

    

        self.gl.draw(arg.viewport(), |c, gl| {
            graphics::clear(BACKGROUND, gl);
            rectangle(COO, square_c,c.transform.trans(pos_x,pos_y),gl); // deplace le carre de -200 vers la gauche  
        });
    }

    fn press(&mut self, args: &Button)
    {
        if let &Button::Keyboard(key) = args
        {
            if self.pos_x == -100.0 || self.pos_x == 270.0
            {
                println!("board x hit");

            }
            if self.pos_y == -100.0 || self.pos_y == 670.0
            {
                println!("board y hit");
            } 
            else
            {

                match key 
                {
                    Key::Up => {self.pos_y -= 10.0}
                    Key::Down => {self.pos_y += 10.0}
                    Key::Left => {self.pos_x -= 10.0}
                    Key::Right => {self.pos_x += 10.0}
                    _ => {println!("other1");}
                }
            }
        }
    }

    fn release(&mut self, args: &Button)
    {
        if let &Button::Keyboard(key) = args 
        {
            match key
            {
                Key::Up => {println!("Up release");}
                Key::Down => {println!("Down release");}
                Key::Left => {println!("Left release");}
                Key::Right => {println!("Right release");}
                _ => {println!("other release");}
            }
        }
    }
}

fn main()
{
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Tetris Game",[400,800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut game = Game{
        gl: GlGraphics::new(opengl),
        pos_x: 200.0,
        pos_y: 0.0,
        };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) 
    {
            if let Some(r) = e.render_args() 
            {
                game.render(&r);
            }
            if let Some(b) = e.press_args()
            {
                game.press(&b);
            }
            if let Some(b) = e.release_args()
            {
                game.release(&b);
            }
    
    }
}
 