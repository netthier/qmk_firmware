#include QMK_KEYBOARD_H


#define LSFTA   LSFT_T(KC_A)
#define RSFTO   RSFT_T(KC_O)
#define LCTLR   LCTL_T(KC_R)
#define RCTLI   RCTL_T(KC_I)
#define LGUISPC LGUI_T(KC_SPC)
#define RGUIENT RGUI_T(KC_ENT)
#define RALTTAB RALT_T(KC_TAB)

#define LT1S    LT(1,KC_S)
#define LT1E    LT(1,KC_E)
#define LT2T    LT(2,KC_T)
#define LT2N    LT(2,KC_N)


const uint16_t PROGMEM combo_toggle_modless[] = {KC_B, KC_J, COMBO_END};
combo_t key_combos[] = {
    COMBO(combo_toggle_modless, TG(3))
};


// exempts RALTTAB from chordal hold
const char PROGMEM chordal_hold_layout[MATRIX_ROWS][MATRIX_COLS] = LAYOUT_split_3x5_2(
    'L', 'L', 'L', 'L', 'L',           'R', 'R', 'R', 'R', 'R',
    'L', 'L', 'L', 'L', 'L',           'R', 'R', 'R', 'R', 'R',
    'L', 'L', 'L', 'L', 'L',           'R', 'R', 'R', 'R', 'R',
                        'L', 'L', '*', 'R'
);


const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
    // base
    [0] = LAYOUT_split_3x5_2(
            KC_Q,       KC_W,       KC_F,       KC_P,       KC_B,                               KC_J,       KC_L,       KC_U,       KC_Y,       KC_SCLN,
            LSFTA,      LCTLR,      LT1S,       LT2T,       KC_G,                               KC_M,       LT2N,       LT1E,       RCTLI,      RSFTO,
            KC_Z,       KC_X,       KC_C,       KC_D,       KC_V,                               KC_K,       KC_H,       KC_COMM,    KC_DOT,     KC_SLSH,
                                                            LGUISPC,    KC_BSPC,    RALTTAB,    RGUIENT
    ),
    // navigation, numbers, math ops
    [1] = LAYOUT_split_3x5_2(
            KC_NO,      KC_PGDN,    KC_PGUP,    KC_ESC,     KC_NO,                              KC_PSLS,    KC_7,       KC_8,       KC_9,       KC_PMNS,
            KC_LEFT,    KC_DOWN,    KC_UP,      KC_RGHT,    KC_NO,                              KC_PAST,    KC_4,       KC_5,       KC_6,       KC_PPLS,
            KC_NO,      KC_NO,      KC_NO,      KC_PSCR,    KC_NO,                              KC_PDOT,    KC_1,       KC_2,       KC_3,       KC_PENT,
                                                            KC_NO,      KC_NO,      KC_NO,      KC_0
    ),
    // symbols
    [2] = LAYOUT_split_3x5_2(
            KC_EXLM,    KC_AT,      KC_HASH,    KC_DLR,     KC_PERC,                            KC_CIRC,    KC_AMPR,    KC_ASTR,    KC_LPRN,    KC_RPRN,
            KC_EQL,     KC_QUOT,    KC_LCBR,    KC_RCBR,    KC_BSLS,                            KC_PIPE,    KC_LPRN,    KC_RPRN,    KC_DQUO,    KC_UNDS,
            KC_NO,      KC_NO,      KC_LBRC,    KC_RBRC,    KC_GRV,                             KC_TILD,    KC_NO,      KC_NO,      KC_NO,      KC_NO,
                                                            KC_NO,      KC_NO,      KC_NO,      KC_NO
    ),
    // modless base (for e.g. gaming)
    [3] = LAYOUT_split_3x5_2(
            KC_Q,       KC_W,       KC_F,       KC_P,       KC_B,                               KC_J,       KC_L,       KC_U,       KC_Y,       KC_SCLN,
            KC_A,       KC_R,       KC_S,       KC_T,       KC_G,                               KC_M,       KC_N,       KC_E,       KC_I,       KC_O,
            KC_Z,       KC_X,       KC_C,       KC_D,       KC_V,                               KC_K,       KC_H,       KC_COMM,    KC_DOT,     KC_SLSH,
                                                            KC_SPC,     KC_BSPC,    KC_TAB,     KC_ENT
    )
};
