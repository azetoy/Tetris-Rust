use piston_window::{Button, Key};
use piston_window::{Context, DrawState, Graphics, Rectangle};
use piston_window::{Event, PistonWindow, PressEvent, RenderEvent, WindowSettings};
use rand::Rng;
use std::collections::HashMap;

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

    fn press(&mut self, args: &Button) 
    {
        let temp = self.0.clone();
            self.0 = temp   
                .into_iter()
                .map(|(mut x, y)| {
                    let mut vec = Vec::new();
                    let mut acc = 0;
                    vec.push(x);
                    let len = vec.len();
                    println!("vec = {:?}, len = {:?} x = {:?}",vec,len,x);
                    if let &Button::Keyboard(key) = args 
                    {
                        match key {
                            Key::Up => 
                            for i in vec 
                            {
                                println!(" i = {:?}",i);
                                if i >= (0,0) 
                                {

                                    acc += 1;
                                    println!("acc vaut {:?}",acc);

                                } 
                                if acc == 4
                                {
                                    println!("ww");
                                    x.1 -= 1;
                                    acc = 0;
                                }
                                else
                                {
                                    println!("board hit up i vaut {:?}",i);
                                    println!("acc vaut {:?}",acc);
                                }
                            },
                            Key::Down => if (x.1) >= 15 {println!("board hit Down");} else{x.1 += 1},
                            Key::Left => x.0 -= 1,
                            Key::Right => if (x.0) >= 7 {println!("board hit Right");} else{x.0 += 1},
                            _ => {
                                println!("{:?}", x.0);
                            }
                        };
                    }
                    (x, y)
                })
                .collect();
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
                //Plateau de jeu
                Rectangle::new([0.2, 0.2, 0.2, 1.0]).draw(
                    cube_bordure,
                    &DrawState::default(),
                    c.transform,
                    g,
                );
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
}
struct Game {
    pieces: Piece,
    dim: Dimmension,
    piece_p: Vec<Piece>,
}
impl Game {
    fn new(dim: Dimmension) -> Self {
        Self {
            dim,
            pieces: Default::default(),
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
            self.pieces.render(&self.dim, &c, g);
        });
    }
}
fn main() {
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
    partie.pieces = partie.pieces.merged(&partie.piece_p[rng.gen_range(0, 6)]);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            partie.render(&mut window, &e);
        }
        if let Some(b) = e.press_args() {
            Piece::press(&mut partie.pieces, &b);
        }
    }
}
