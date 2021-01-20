// inspirer des source suivante 

//https://github.com/da-x/tetris-demo
//Licence MIT

use piston_window::{Button, Key};
use piston_window::{Context, DrawState, Graphics, Rectangle};
use piston_window::{Event, PistonWindow, PressEvent, RenderEvent, WindowSettings};
use rand::Rng;
use std::collections::HashMap;
use piston_window::UpdateEvent;
use std::time::Instant;

#[derive(Copy, Clone)]
enum Couleur {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Orange,
    Violet,
}
#[derive(Default)]
struct Dimmension {
    taille_cube: usize,
    taille_x: usize,
    taille_y: usize,
}
impl Dimmension {
    fn resolution(&self) -> [u32; 2] {
        [
            (self.taille_x * self.taille_cube) as u32,
            (self.taille_y * self.taille_cube) as u32,
        ]
    }
}
#[derive(Default, Clone)]
struct Piece(HashMap<(i32, i32), Couleur>);
//Hash [Key(i32,i32) -> valeur,Couleur]

impl Piece {
    fn new(v: &[(i32, i32)], couleur: Couleur) -> Self {
        Piece(v.iter().cloned().map(|(x, y)| ((x, y), couleur)).collect())
    }
    fn merged(&self, other: &Piece) -> Self {
        let mut hashmap = HashMap::new();
        hashmap.extend(other.0.iter());
        hashmap.extend(self.0.iter());
        Self(hashmap)
    }

    fn chech_pos(&mut self,x : i32,y:i32,piece_total : &Piece) -> bool
    {
        let mut r:bool = true;
        let mut res2:Vec<bool>;
        res2 = [].to_vec();
        let mut res:Vec<bool>;
        let temp = self.0.clone();
        
        res = temp
            .into_iter()
            .map(|(x_pos,_y_pos)|
            {
                println!("pos x = {:?} pos y = {:?}",x_pos.0,x_pos.1);
                if x_pos.0 + x < 0 || x_pos.1 + y >= 17 || x_pos.0 + x >= 8
                {
                    false
                }
                else
                {
                    let temp_piece = piece_total.0.clone();
                    res2 = temp_piece
                    .into_iter()
                    .map(|(x_pos2,_y_pos2)|
                    {
                        if x_pos.0 + x == x_pos2.0 && x_pos.1 + y == x_pos2.1
                        {
                            println!("¯_(ツ)_/¯,( ͡° ͜ʖ ͡°)");
                            false   
                        }
                        else
                        {
                            /*println!("
                            ░░░█▀▀▀░█▀▀▀░░█▀▀░▀▀█░░█░░
                            ░░░█░▀█░█░▀█░░█▀▀░▄▀░░░▀░░
                            ░░░▀▀▀▀░▀▀▀▀░░▀▀▀░▀▀▀░░▀░░");*/
                            println!("( 。・_・。)人(。・_・。 ),╭∩╮（︶︿︶）╭∩╮");
                            true
                        }
                    }).collect();
                    true
                }


            }).collect();
            for i in res
            {
                if i == false
                    {
                       r = false;
                    }
                
            }
            for i in res2
            {
                if i== false
                {
                    r = false;
                }
            }
            r

    }


    fn dab(&mut self,x:i32,y:i32)
    {
        println!("im here x = {:?} y = {:?}",x,y);
        let temp = self.0.clone();
        
        self.0 = temp
            .into_iter()
            .map(|(mut x_pos,_y_pos)|
            {
                x_pos.0 += x;
                x_pos.1 += y;
                (x_pos,_y_pos)
            }).collect();      
    }

    fn press(&mut self, args: &Button,piece : &Piece) 
    { 
         
        if let &Button::Keyboard(key) = args 
        {
            match key {
                Key::Down => if self.chech_pos(0,1,&piece){self.dab(0,1);},
                Key::Left => if self.chech_pos(-1,0,&piece){self.dab(-1,0);},
                Key::Right => if self.chech_pos(1,0,&piece){self.dab(1,0);},
                _ => {},
            };
        }
    }

    fn render<G>(&self, dim: &Dimmension, c: &Context, g: &mut G)
    where
        G: Graphics,
    {
        for x in 0..dim.taille_x {
            for y in 0..dim.taille_y {
                let taille_cube = dim.taille_cube as f64;
                let taille_bordure = taille_cube / 20.0;
                let cube_bordure = [
                    taille_cube * (x as f64),
                    taille_cube * (y as f64),
                    taille_cube,
                    taille_cube,
                ];
                let cube = [
                    cube_bordure[0] + taille_bordure,
                    cube_bordure[1] + taille_bordure,
                    taille_cube - taille_bordure * 2.0,
                    taille_cube - taille_bordure * 2.0,
                ];
                if let Some(couleur) = self.0.get(&(x as i32, y as i32)) {
                    let code = match couleur {
                        Couleur::Red => [0.8, 0.0, 0.0, 1.0],
                        Couleur::Green => [0.0, 0.8, 0.0, 1.0],
                        Couleur::Blue => [0.0, 0.0, 0.8, 1.0],
                        Couleur::Yellow => [0.8, 0.8, 0.0, 1.0],
                        Couleur::Orange => [0.8, 0.3, 0.0, 1.0],
                        Couleur::Violet => [0.8, 0.0, 0.8, 1.0],
                        Couleur::Cyan => [0.0, 0.8, 0.8, 1.0],
                    };
                    Rectangle::new(code).draw(cube_bordure, &DrawState::default(), c.transform, g);
                    let code = [code[0] * 1.2, code[1] * 1.2, code[2] * 1.2, 1.0];
                    Rectangle::new(code).draw(cube, &DrawState::default(), c.transform, g);
                }
            }
        }
    }

    fn render_board<G>(&self, dim: &Dimmension, c: &Context, g: &mut G)
        where
        G: Graphics,
    {
        for x in 0..dim.taille_x {
            for y in 0..dim.taille_y {
                let taille_cube = dim.taille_cube as f64;
                let taille_bordure = taille_cube / 20.0;
                let cube_bordure = [
                    taille_cube * (x as f64),
                    taille_cube * (y as f64),
                    taille_cube,
                    taille_cube,
                ];
                let cube = [
                    cube_bordure[0] + taille_bordure,
                    cube_bordure[1] + taille_bordure,
                    taille_cube - taille_bordure * 2.0,
                    taille_cube - taille_bordure * 2.0,
                ];
                //Plateau de jeu
                Rectangle::new([0.2, 0.2, 0.2, 1.0]).draw(
                    cube_bordure,
                    &DrawState::default(),
                    c.transform,
                    g,
                );

            }
        }
    }
}
struct Game {
    pieces: Piece,
    dim: Dimmension,
    current_piece: Piece,
    piece_p: Vec<Piece>,
}
impl Game {
    fn new(dim: Dimmension) -> Self {
        Self {
            dim,
            pieces: Default::default(),
            current_piece: Default::default(),
            piece_p: vec![
                Piece::new(&[(0, 0), (0, 1), (1, 0), (1, 1)], Couleur::Yellow),
                Piece::new(&[(0, 0), (1, 0), (2, 0), (1, 1)], Couleur::Violet),
                Piece::new(&[(0, 0), (1, 0), (2, 0), (3, 0)], Couleur::Cyan),
                Piece::new(&[(0, 0), (0, 1), (1, 1), (2, 1)], Couleur::Blue),
                Piece::new(&[(2, 0), (0, 1), (1, 1), (2, 1)], Couleur::Orange),
                Piece::new(&[(1, 0), (2, 0), (0, 1), (1, 1)], Couleur::Green),
                Piece::new(&[(0, 0), (1, 0), (1, 1), (2, 1)], Couleur::Red),
            ],
        }
    }
    fn render(&self, window: &mut PistonWindow, event: &Event) {
        window.draw_2d(event, |c, g, _| {
            self.pieces.render_board(&self.dim,&c,g);
            self.pieces.render(&self.dim, &c, g);
            self.current_piece.render(&self.dim, &c, g);
        });
    }
}
fn main() {
    let mut start = Instant::now();
    let dim = Dimmension {
        taille_cube: 40,
        taille_x: 8,
        taille_y: 20,
    };
    let bg = [0.0, 0.0, 0.0, 1.0];
    let mut rng = rand::thread_rng();

    let mut window: PistonWindow = WindowSettings::new("Tetris", dim.resolution())
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = window.events;

    let mut partie = Game::new(dim);
    partie.current_piece = partie.piece_p[rng.gen_range(0, 6)].clone();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            partie.render(&mut window, &e);

        }
        if let Some(b) = e.press_args() {
            Piece::press(&mut partie.current_piece, &b,&partie.pieces);
        }
        if let Some(b) = e.update_args()
        {
            let apres = start.elapsed();
            if apres.as_secs() > 1
            {

            if partie.current_piece.chech_pos(0,1,&partie.pieces){partie.current_piece.dab(0,1);}
            else{
                partie.pieces = partie.pieces.merged(&partie.current_piece);
                partie.current_piece =partie.piece_p[rng.gen_range(0, 6)].clone();

                };
            start = Instant::now();
            }

        }
       // piston_window::Event::from_dt(1000000000000000000000000000000000000000000000.0,&e);
    }
    
}
