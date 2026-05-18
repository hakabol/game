use tcod::colors::*;
use tcod::console::*;
use tcod::input::{Key, KeyCode::*};

use std::cmp;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 47;

const Y_SUB: i32 = SCREEN_HEIGHT - MAP_HEIGHT;
const X_SUB: i32 = SCREEN_WIDTH - MAP_WIDTH;


const COLOR_DARK_WALL: Color = Color {r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150};

const LIMIT_FPS: i32 = 20;

#[derive(Clone, Copy, Debug)]

struct Tile{
    blocked: bool,
    block_sight: bool,
}

impl Tile{
    pub fn empty() -> Self{
        Tile{
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self{
        Tile{
            blocked: true,
            block_sight: true,
        }
    }
}

type Map = Vec<Vec<Tile>>;

struct Game{
    map: Map,
}

#[derive(Debug)]

#[derive(PartialEq, Clone)]
struct Character{
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Character{
    fn new(x: i32, y: i32, char: char, color: Color) -> Self{
        Character {x, y, char, color}
    }

    fn move_by(&mut self, dx: i32, dy: i32, game: &Game, objects: &Vec<Character>){
        let mut check = true;
        for object in objects{
            if object != self{
                if self.x + dx == object.x && self.y + dy == object.y{
                    check = false;
                }
            }
        }
        if !game.map[(self.x + dx - X_SUB) as usize][(self.y + dy - Y_SUB) as usize].blocked && check{
            self.x += dx;
            self.y += dy;
        }
    }
    
    fn draw(&self, con: &mut dyn Console){
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

struct Tcod{
    root: Root,
    con: Offscreen,
}

struct Rect{
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect{
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self{
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("idk game i guess")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod {root, con};

    tcod::system::set_fps(LIMIT_FPS);

    let enemy = Character::new(SCREEN_WIDTH/2 - 5, SCREEN_HEIGHT/2 - 5, '#', WHITE);
    let player = Character::new(25, 23, '@', WHITE);

    let mut objects = vec![player, enemy];

    let mut map = make_map();
    
    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);

    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_h_tunnel(30, 50, 23, &mut map);
    
    let game = Game { map: map };

    while !tcod.root.window_closed(){
        tcod.con.clear();

        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        let clone_obj = objects.clone();
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player, &game, &clone_obj);
        if exit{
            break;
        }

    }
}

fn handle_keys(tcod: &mut Tcod, object: &mut Character, game: &Game, objects: &Vec<Character>) -> bool{
    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {code: Up, ..} =>               object.move_by( 0, -1, game, objects),
        Key {code: Down, ..} =>             object.move_by( 0,  1, game, objects),
        Key {code: Left, ..} =>             object.move_by(-1,  0, game, objects),
        Key {code: Right, ..} =>            object.move_by( 1,  0, game, objects),
        Key { printable: 'w', .. } =>       object.move_by( 0, -1, game, objects),
        Key { printable: 's', .. } =>       object.move_by( 0,  1, game, objects),
        Key { printable: 'a', .. } =>       object.move_by(-1,  0, game, objects),
        Key { printable: 'd', .. } =>       object.move_by( 1,  0, game, objects),

        Key {code: Enter, alt: true, ..} => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        _ => {}
    }

    false
}

fn make_map() -> Map{
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &Vec<Character>){
    for y in Y_SUB..(SCREEN_HEIGHT){
        for x in X_SUB..(SCREEN_WIDTH){
            let wall = game.map[(x - X_SUB) as usize][(y - Y_SUB) as usize].block_sight;

            if wall{
                tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            }
            else{
                tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }
    for object in objects{
        object.draw(&mut tcod.con);
    }

    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (X_SUB, Y_SUB),
        1.0,
        1.0,
    );

}

fn create_room(room: Rect, map: &mut Map){
    for x in (room.x1 + 1)..room.x2{
        for y in (room.y1 + 1)..room.y2{
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map){
    for x in cmp::min(x1, x2)..=cmp::max(x1, x2){
        map[x as usize][y as usize] = Tile::empty();
    }
}
