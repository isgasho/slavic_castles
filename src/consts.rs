use quicksilver::graphics::Color;

pub const BASE_RESOURCE_AMOUNT: i32 = 5;
pub const BASE_RESOURCE_PRODUCTION: i32 = 1;
pub const BASE_TOWER_HP: i32 = 20;
pub const BASE_WALLS_HP: i32 = 15;
pub const MAX_TOWER_HP: i32 = 100;
pub const MAX_WALLS_HP: i32 = 50;
pub const MAX_PRODUCTION: i32 = 10;
pub const CARDS_IN_DECK: i32 = 5;

pub const DELAY_BETWEEN_MOVES: f64 = 1.2;
pub const AVATAR_SHAKE_DURATION: f64 = 0.6;
pub const RESOURCE_SHAKE_DURATION: f64 = 0.6;
pub const AVATAR_SHAKE_STRENGTH: (f32, f32) = (4.0, 4.0);
pub const RESOURCE_SHAKE_STRENGTH: (f32, f32) = (4.0, 7.0);
// texts
pub const HELP_TEXT: &str = "Help\n Get 100 life or destroy opponent to win\n Left mouse button- use card\n Right mouse button- discard card\n R- restart game\n H- show this info";
// UI
pub const FONT_COLOR: Color = Color {
    r: 29.0 / 255.0,
    g: 53.0 / 255.0,
    b: 87.0 / 255.0,
    a: 1.0,
};
pub const FONT_WHITE_COLOR: Color = Color {
    r: 0.95,
    g: 0.95,
    b: 0.95,
    a: 1.0,
};
pub const BTN_HOVERED_COLOR: Color = Color {
    r: 0.95,
    g: 1.0,
    b: 0.95,
    a: 1.0,
};
pub const BTN_DISABLED_COLOR: Color = Color {
    r: 0.6,
    g: 0.6,
    b: 0.6,
    a: 0.9,
};
pub const FONT_GREY_COLOR: Color = Color {
    r: 0.65,
    g: 0.65,
    b: 0.65,
    a: 1.0,
};
pub const ACTIVE_FONT_COLOR: (f32, f32, f32, f32) =
    (230.0 / 255.0, 57.0 / 255.0, 70.0 / 255.0, 1.0);
pub const BG_COLOR: Color = Color {
    r: 145.0 / 255.0,
    g: 101.0 / 255.0,
    b: 78.0 / 255.0,
    a: 255.0,
};
pub const GREY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};

pub const TOOLS_COLOR: Color = Color {
    r: 140.0 / 255.0,
    g: 193.0 / 255.0,
    b: 1.0,
    a: 1.0,
};
pub const MAGIC_COLOR: Color = Color {
    r: 118.0 / 255.0,
    g: 206.0 / 255.0,
    b: 113.0 / 255.0,
    a: 1.0,
};
pub const SOLDIERS_COLOR: Color = Color {
    r: 206.0 / 255.0,
    g: 55.0 / 255.0,
    b: 75.0 / 255.0,
    a: 1.0,
};

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

pub const CARD_SIZE_X: f32 = 234.0;
pub const CARD_SIZE_Y: f32 = 320.0;
pub const BTN_SIZE_X: f32 = 256.0;
pub const BTN_SIZE_Y: f32 = 64.0;
pub const DECK_JSON: &'static [u8] = include_bytes!("../static/deck.json");
pub const BOARD_BG_IMG: &'static [u8] = include_bytes!("../static_not_included/ingame_bg.png");
pub const START_SCREEN_BG_IMG: &'static [u8] =
    include_bytes!("../static_not_included/start_screen_bg.png");
pub const LOGO_IMG: &'static [u8] = include_bytes!("../static_not_included/logo.png");
pub const BASE_BTN_IMG: &'static [u8] = include_bytes!("../static_not_included/base_btn.png");
