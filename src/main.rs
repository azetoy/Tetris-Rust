extern crate piston_window;

use piston_window::*;

pub struct Game {
    //gl: GlGraphics,
    rotation: f64,
    x: f64,
    y: f64,
    up: bool,down: bool,left: bool,right: bool
}


impl Game {
    fn new() -> Game 
    {
        Game{rotation: 0.0,x : 0.0, y : 0.0, up: false, down: false,left:false,right:false}
    }


    fn update(&mut self, upd: UpdateArgs) 
    {
        self.rotation = 0.0 * upd.dt;
        match self.upd
        {
            self.up => self.x += (10.0) * upd.dt,
            self.down => self.x += (-10.0)* upd.dt,
            self.left => self.y += (-10.0)* upd.dt,
            self.right => self.y += (10.0)* upd.dt,  
        } 


    }

    fn draw(&mut self,x: PistonWindow) 
    {
       x.draw_2d(|c, g|
       {
           clear([0.0, 0.0, 0.0, 1.0],g);
           let center = c.transform.trans((400) as f64, (800) as f64);
           let square = rectangle::square(0.0,0.0,30.0);
           let red = [1.0, 0.0, 0.0, 1.0];
           rectangle(red,square,center.trans(self.x,self.y).trans(-10.0, -10.0),);

       });
    }
    fn on_input(&mut self, inp: Input)
    {
        match inp
        {
            Input::Press(but) =>
            {
                match but
                {
                    Button::Keyboard(Key::Up) => {self.up = true;}
                    Button::Keyboard(Key::Down) => {self.down = true;}
                    Button::Keyboard(key::Left) => {self.left = true;}
                    Button::Keyboard(Key::Right) => {self.right = true;}
                    _ => {}
                }
            }
            Input::Release(but) =>
            {
                match but 
                {
                    Button::Keyboard(Key::Up) => {self.up = false;}
                    Button::Keyboard(Key::Down) => {self.down = false;}
                    Button::Keyboard(key::Left) => {self.left = false;}
                    Button::Keyboard(Key::Right) => {self.right = false;}
                    _ => {}

                }
            }
            _ => {}
        }
    }
}
fn main() 
{
    let window: PistonWindow = WindowSettings::new(
        "The Binding of Tetris",
        [400,800]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new();
    for e in window
    {
        match e.event
        {
            Some(Event::Update(upd)) => {game.update(upd);}
            Some(Event::Render(ren)) => {game.draw(ren, e);}
            Some(Event::Input(inp)) => {game.on_input(inp);}
            _ => {}
        }
    }
}
