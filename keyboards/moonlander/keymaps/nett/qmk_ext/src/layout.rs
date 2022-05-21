#![allow(unused_imports)]
#![allow(unused)]
#![allow(non_upper_case_globals)]

const MATRIX_COLS: usize = qmk::bindgen::MATRIX_COLS as usize;
const MATRIX_ROWS: usize = qmk::bindgen::MATRIX_ROWS as usize;
pub type Keymaps =  [[[u16; MATRIX_COLS]; MATRIX_ROWS]; 3];

macro_rules! layout_moonlander {
    ($k00:expr, $k01:expr, $k02:expr, $k03:expr, $k04:expr, $k05:expr, $k06:expr,            $k60:expr, $k61:expr, $k62:expr, $k63:expr, $k64:expr, $k65:expr, $k66:expr,
    $k10:expr, $k11:expr, $k12:expr, $k13:expr, $k14:expr, $k15:expr, $k16:expr,             $k70:expr, $k71:expr, $k72:expr, $k73:expr, $k74:expr, $k75:expr, $k76:expr,
    $k20:expr, $k21:expr, $k22:expr, $k23:expr, $k24:expr, $k25:expr, $k26:expr,             $k80:expr, $k81:expr, $k82:expr, $k83:expr, $k84:expr, $k85:expr, $k86:expr,
    $k30:expr, $k31:expr, $k32:expr, $k33:expr, $k34:expr, $k35:expr,                                   $k91:expr, $k92:expr, $k93:expr, $k94:expr, $k95:expr, $k96:expr,
    $k40:expr, $k41:expr, $k42:expr, $k43:expr, $k44:expr,            $k53:expr,             $kb3:expr,            $ka2:expr, $ka3:expr, $ka4:expr, $ka5:expr, $ka6:expr,
                                                $k50:expr, $k51:expr, $k52:expr,             $kb4:expr, $kb5:expr, $kb6:expr
) => {{
        use alias_kc::*;
        use alias_fn::*;
        use Layer::*;
[
    [$k00,$k01,$k02,$k03,$k04,$k05,$k06],
    [$k10,$k11,$k12,$k13,$k14,$k15,$k16],
    [$k20,$k21,$k22,$k23,$k24,$k25,$k26],
    [$k30,$k31,$k32,$k33,$k34,$k35,noop],
    [$k40,$k41,$k42,$k43,$k44,noop,noop],
    [$k50,$k51,$k52,$k53,noop,noop,noop],

    [$k60,$k61,$k62,$k63,$k64,$k65,$k66],
    [$k70,$k71,$k72,$k73,$k74,$k75,$k76],
    [$k80,$k81,$k82,$k83,$k84,$k85,$k86],
    [noop,$k91,$k92,$k93,$k94,$k95,$k96],
    [noop,noop,$ka2,$ka3,$ka4,$ka5,$ka6],
    [noop,noop,noop,$kb3,$kb4,$kb5,$kb6]
]}}
}

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum Layer {
    Base = 0,
    Symb = 1,
    Mdia = 2,
}

pub const KEYMAPS: Keymaps = [
    // Base
    layout_moonlander![
        eql,    _1,     _2,     _3,     _4,     _5,     left,           rght,   _6,     _7,     _8,     _9,     _0,     mins,
        del,    q,      w,      e,      r,      t,      tg(Symb),       tg(Symb),y,     u,      i,      o,      p,      lt(Mdia,bsls),
        bspc,   a,      s,      d,      f,      g,      hypr,           meh,    h,      j,      k,      l,      scln,   lgui_t(quot),
        lsft,   lctl_t(z),x,    c,      v,      b,                              n,      m,      comm,   dot,    rctl_t(slsh),rsft,
        lt(Symb,grv),__,__,     left,   rght,           lalt_t(app),    rctl_t(esc),    up,     down,   lbrc,   rbrc,   mo(Symb),
                                        spc,    bspc,   lgui,           lalt,   tab,    ent
    ],

    // Symbols
    layout_moonlander![
        vsrn,   f1,     f2,     f3,     f4,     f5,     ______,         ______, f6,     f7,     f8,     f9,     f10,    f11,
        ______, exlm,   at,     lcbr,   rcbr,   pipe,   ______,         ______, up,     _7,     _8,     _9,     astr,   f12,
        ______, hash,   dlr,    lprn,   rprn,   grv,    ______,         ______, down,   _4,     _5,     _6,     plus,   ______,
        ______, perc,   circ,   lbrc,   rbrc,   tild,                           ampr,   _1,     _2,     _3,     bsls,   ______,
        eclr,   ______, ______, ______, ______,         rgb_vai,        rgb_tog,        ______, dot,    _0,     eql,    ______,
                                                rgb_hud,rgb_vad,rgb_hui,______, ______, ______
    ],

    // Media
    layout_moonlander![
        ______, ______, ______, ______, ______, ______, ______,         ______, ______, ______, ______, ______, ______, boot,
        ______, ______, ______, ______, ______, ______, ______,         ______, ______, ______, ______, ______, ______, ______,
        ______, ______, ______, ______, ______, ______, ______,         ______, ______, ______, ______, ______, ______, ______,
        ______, ______, ______, ______, ______, ______,                         ______, ______, ______, ______, ______, ______,
        ______, ______, ______, ______, ______,         ______,          ______,        volu,   vold,   mute,   ______, ______,
                                        ______, ______, ______,          ______, ______, ______
    ],
];

macro_rules! alias_keycodes {
    ($($name:ident $val:expr),*) => {
        pub mod alias_kc {
            use qmk::bindgen::{
                KC_TRANSPARENT,
                hid_keyboard_keypad_usage::*,
                internal_special_keycodes::*,
                quantum_keycodes::*
            };
            use crate::keymap::CustomKeycode;
        $(
            pub const $name: u16 = ($val) as u16;
        )*
        }
    }
}

alias_keycodes![
    noop    KC_NO,
    ______  KC_TRANSPARENT,
    __      KC_TRANSPARENT,

    vsrn    CustomKeycode::Version,

    a       KC_A,
    b       KC_B,
    c       KC_C,
    d       KC_D,
    e       KC_E,
    f       KC_F,
    g       KC_G,
    h       KC_H,
    i       KC_I,
    j       KC_J,
    k       KC_K,
    l       KC_L,
    m       KC_M,
    n       KC_N,
    o       KC_O,
    p       KC_P,
    q       KC_Q,
    r       KC_R,
    s       KC_S,
    t       KC_T,
    u       KC_U,
    v       KC_V,
    w       KC_W,
    x       KC_X,
    y       KC_Y,
    z       KC_Z,

    _1      KC_1,
    _2      KC_2,
    _3      KC_3,
    _4      KC_4,
    _5      KC_5,
    _6      KC_6,
    _7      KC_7,
    _8      KC_8,
    _9      KC_9,
    _0      KC_0,

    ent     KC_ENTER,
    esc     KC_ESCAPE,
    bspc    KC_BACKSPACE,
    tab     KC_TAB,
    spc     KC_SPACE,
    mins    KC_MINUS,
    eql     KC_EQUAL,
    lbrc    KC_LEFT_BRACKET,
    rbrc    KC_RIGHT_BRACKET,
    bsls    KC_BACKSLASH,
    nuhs    KC_NONUS_HASH,
    scln    KC_SEMICOLON,
    quot    KC_QUOTE,
    grv     KC_GRAVE,
    comm    KC_COMMA,
    dot     KC_DOT,
    slsh    KC_SLASH,
    caps    KC_CAPS_LOCK,

    tild    QK_LSFT | KC_GRAVE,
    exlm    QK_LSFT | KC_1,
    at      QK_LSFT | KC_2,
    hash    QK_LSFT | KC_3,
    dlr     QK_LSFT | KC_4,
    perc    QK_LSFT | KC_5,
    circ    QK_LSFT | KC_6,
    ampr    QK_LSFT | KC_7,
    astr    QK_LSFT | KC_8,
    lprn    QK_LSFT | KC_9,
    rprn    QK_LSFT | KC_0,
    plus    QK_LSFT | KC_EQUAL,
    lcbr    QK_LSFT | KC_LEFT_BRACKET,
    rcbr    QK_LSFT | KC_RIGHT_BRACKET,
    pipe    QK_LSFT | KC_BACKSLASH,

    f1      KC_F1,
    f2      KC_F2,
    f3      KC_F3,
    f4      KC_F4,
    f5      KC_F5,
    f6      KC_F6,
    f7      KC_F7,
    f8      KC_F8,
    f9      KC_F9,
    f10     KC_F10,
    f11     KC_F11,
    f12     KC_F12,
    f13     KC_F13,
    f14     KC_F14,
    f15     KC_F15,
    f16     KC_F16,
    f17     KC_F17,
    f18     KC_F18,
    f19     KC_F19,
    f20     KC_F20,
    f21     KC_F21,
    f22     KC_F22,
    f23     KC_F23,
    f24     KC_F24,

    pscr    KC_PRINT_SCREEN,
    scrl    KC_SCROLL_LOCK,
    paus    KC_PAUSE,
    ins     KC_INSERT,
    home    KC_HOME,
    pgup    KC_PAGE_UP,
    del     KC_DELETE,
    end     KC_END,
    pgdn    KC_PAGE_DOWN,
    rght    KC_RIGHT,
    left    KC_LEFT,
    down    KC_DOWN,
    up      KC_UP,
    num     KC_NUM_LOCK,

    psls    KC_KP_SLASH,
    past    KC_KP_ASTERISK,
    pmns    KC_KP_MINUS,
    ppls    KC_KP_PLUS,
    pent    KC_KP_ENTER,
    pdot    KC_KP_DOT,
    peql    KC_KP_EQUAL,

    nubs    KC_NONUS_BACKSLASH,
    app     KC_APPLICATION,
    powr    KC_KB_POWER,
    lctl    KC_LEFT_CTRL,
    lsft    KC_LEFT_SHIFT,
    lalt    KC_LEFT_ALT,
    lgui    KC_LEFT_GUI,
    rctl    KC_RIGHT_CTRL,
    rsft    KC_RIGHT_SHIFT,
    ralt    KC_RIGHT_ALT,
    rgui    KC_RIGHT_GUI,
    pwr     KC_SYSTEM_POWER,
    slep    KC_SYSTEM_SLEEP,
    wake    KC_SYSTEM_WAKE,
    mute    KC_AUDIO_MUTE,
    volu    KC_AUDIO_VOL_UP,
    vold    KC_AUDIO_VOL_DOWN,
    briu    KC_BRIGHTNESS_UP,
    brid    KC_BRIGHTNESS_DOWN,

    boot    QK_BOOTLOADER,
    eclr    QK_CLEAR_EEPROM,

    rgb_tog RGB_TOG,
    rgb_hui RGB_HUI,
    rgb_hud RGB_HUD,
    rgb_sai RGB_SAI,
    rgb_sad RGB_SAD,
    rgb_vai RGB_VAI,
    rgb_vad RGB_VAD,

    meh     lctl | lsft | lalt,
    hypr    lctl | lsft | lalt | lgui
];

mod alias_fn {
    use qmk::bindgen::{
        hid_keyboard_keypad_usage::*,
        internal_special_keycodes::*,
        quantum_keycodes::*
    };

    use super::Layer;

    macro_rules! gen_mod_t {
        ($($name:ident $mod:ident),*) => {
            $(
            pub const fn $name(kc: u16) -> u16 {
                mt(qmk::bindgen::mods_bit::$mod as u16, kc)
            }
            )*
        }
    }

    gen_mod_t![
        lctl_t  MOD_LCTL,
        rctl_t  MOD_RCTL,
        lsft_t  MOD_LSFT,
        rsft_t  MOD_RSFT,
        lalt_t  MOD_LALT,
        ralt_t  MOD_RALT,
        lgui_t  MOD_LGUI,
        rgui_t  MOD_RGUI
    ];

    /// Toggle to layer
    pub const fn tg(layer: Layer)  -> u16 {
        assert!(layer as u16 <= u8::MAX as u16);
        QK_TOGGLE_LAYER as u16 | layer as u16
    }

    /// L-ayer T-ap
    pub const fn lt(layer: Layer, kc: u16) -> u16 {
        assert!(layer as u16 <= 16);
        assert!(kc <= u8::MAX as u16);
        QK_LAYER_TAP as u16 | ((layer as u16) << 8) | kc
    }

    /// M-od T-ap
    pub const fn mt(m: u16, kc: u16) -> u16 {
        assert!(m <= 0x1F);
        assert!(kc <= u8::MAX as u16);
        QK_MOD_TAP as u16 | (m << 8) | kc
    }

    /// Momentarily switch layer
    pub const fn mo(layer: Layer) -> u16 {
        assert!(layer as u16 <= u8::MAX as u16);
        QK_MOMENTARY as u16| layer as u16
    }
}