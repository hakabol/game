use tcod::colors::*;
use tcod::console::*;
use tcod::input::{Key, KeyCode::*};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

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

    fn move_by(&mut self, dx: i32, dy: i32){
        self.x += dx;
        self.y += dy;
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

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("idk game i guess")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod {root, con};

    tcod::system::set_fps(LIMIT_FPS);

    let enemy = Character::new(SCREEN_WIDTH/2 - 5, SCREEN_HEIGHT/2 - 5, '#', WHITE);
    let player = Character::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, '@', WHITE);

    let mut objects = vec![player, enemy];

    let map = make_map();
    let game = Game {map: map};

    while !tcod.root.window_closed(){
        tcod.con.clear();

        render_all(&mut tcod, &game, &objects);

        /*
        for character in &objects{
            character.draw(&mut tcod.con);
        }

        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
        */
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit{
            break;
        }

    }
}

fn handle_keys(tcod: &mut Tcod, object: &mut Character) -> bool{
    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {code: Up, ..} =>               object.move_by( 0, -1),
        Key {code: Down, ..} =>             object.move_by( 0,  1),
        Key {code: Left, ..} =>             object.move_by(-1,  0),
        Key {code: Right, ..} =>            object.move_by( 1,  0),
        Key { printable: 'w', .. } =>       object.move_by( 0, -1),
        Key { printable: 's', .. } =>       object.move_by( 0,  1),
        Key { printable: 'a', .. } =>       object.move_by(-1,  0),
        Key { printable: 'd', .. } =>       object.move_by( 1,  0),

        Key {code: Enter, alt: true, ..} => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        _ => {}
    }

    false
}

fn make_map() -> Map{
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &Vec<Character>){
    for y in 0..MAP_HEIGHT{
        for x in 0..MAP_WIDTH{
            let wall = game.map[x as usize][y as usize].block_sight;

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
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );

}
