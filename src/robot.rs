use speedy2d::{Graphics2D, Rect};
use strum_macros::IntoStaticStr;

use crate::spritesheet::Spritesheet;

struct Animation {
    current_frame: usize,
    length: usize,
}

impl Animation {
    const fn new(current_frame: usize, length: usize) -> Self {
        Self {
            current_frame,
            length,
        }
    }

    pub fn next(&mut self) {
        self.current_frame += 1;
        self.current_frame %= self.length;
    }
}

enum Drawable {
    Animation(Animation),
    Sprite,
}

impl Drawable {
    pub fn next(&mut self) {
        match self {
            Self::Animation(animation) => animation.next(),
            Self::Sprite => (),
        }
    }
}

pub struct Robot {
    spritesheet: Spritesheet,
    state: RobotState,
    last_state: RobotState,
    drawable: Option<Drawable>,
}

impl Robot {
    pub const fn new(spritesheet: Spritesheet) -> Self {
        Self {
            spritesheet,
            state: RobotState::Idle,
            last_state: RobotState::Idle,
            drawable: None,
        }
    }

    pub fn update(&mut self, current_frame: u64) {
        if current_frame % 4 == 0 {
            self.advance_animation();
        }
    }

    pub fn dbg_next_state(&mut self, nth: usize) {
        let some_states = [
            RobotState::Idle,
            RobotState::Walk,
            RobotState::Run,
            RobotState::Attack,
            RobotState::Switch,
            RobotState::Cheer,
            RobotState::Climb,
            RobotState::Hit,
        ];
        self.set_state(some_states[nth % some_states.len()]);
    }

    fn set_state(&mut self, state: RobotState) {
        self.state = state;
        self.set_drawable();
    }

    fn set_drawable(&mut self) {
        let state_name: &str = self.state.into();
        if let Some((_animation, animation_frames)) = ROBOT_ANIMATIONS
            .iter()
            .find(|(animation, _)| *animation == state_name)
        {
            self.drawable = Some(Drawable::Animation(Animation::new(0, *animation_frames)));
        } else if let Some((_sprite, [_x, _y])) = ROBOT_SPRITES
            .iter()
            .find(|(sprite, _)| *sprite == state_name)
        {
            self.drawable = Some(Drawable::Sprite);
        }
    }

    pub fn advance_animation(&mut self) {
        if let Some(drawable) = &mut self.drawable {
            drawable.next();
        }
    }

    pub fn draw(&self, dest: &Rect, graphics: &mut Graphics2D) {
        if let Some(drawable) = &self.drawable {
            let name = self.state.into();
            match drawable {
                Drawable::Animation(_animation) => {
                    self.draw_animation(dest, graphics, name);
                }
                Drawable::Sprite => {
                    if let Some((sprite, [x, y])) =
                        ROBOT_SPRITES.iter().find(|(sprite, _)| *sprite == name)
                    {
                        self.draw_sprite(dest, graphics, sprite, *x, *y);
                    }
                }
            }
        }
    }

    fn draw_animation(&self, dest: &Rect, graphics: &mut Graphics2D, animation_name: &str) {
        if let Some(drawable) = &self.drawable {
            match drawable {
                Drawable::Animation(animation) => {
                    let sprite_name = format!("{animation_name}{}", animation.current_frame);
                    if let Some((sprite, [x, y])) = ROBOT_SPRITES
                        .iter()
                        .find(|(sprite, _)| *sprite == sprite_name)
                    {
                        self.draw_sprite(dest, graphics, sprite, *x, *y);
                    }
                }
                Drawable::Sprite => unreachable!(),
            }
        }
    }

    fn draw_sprite(&self, dest: &Rect, graphics: &mut Graphics2D, _sprite: &str, x: u8, y: u8) {
        self.spritesheet
            .draw_sprite(dest, x.into(), y.into(), graphics);
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, IntoStaticStr)]
#[strum(serialize_all = "camelCase")]
enum RobotState {
    Idle,
    Jump,
    Fall,
    Duck,
    Hit,
    Climb,
    Cheer,
    Back,
    Slide,
    Interact,
    Switch,
    Kick,
    Side,
    Shove,
    ShoveBack,
    Talk,
    AttackKick,
    Hang,
    Hold,
    Show,
    BehindBack,
    Run,
    Attack,
    Think,
    Down,
    Drag,
    Hurt,
    Wide,
    Rope,
    Walk,
    FallDown,
}
const ROBOT_ANIMATIONS: [(&str, usize); 6] = [
    ("walk", 8),
    ("run", 3),
    ("attack", 3),
    ("switch", 2),
    ("cheer", 2),
    ("climb", 2),
];

const ROBOT_SPRITES: [(&str, [u8; 2]); 45] = [
    ("idle", [0, 0]),
    ("jump", [1, 0]),
    ("fall", [2, 0]),
    ("duck", [3, 0]),
    ("hit", [4, 0]),
    ("climb0", [5, 0]),
    ("climb1", [6, 0]),
    ("cheer0", [7, 0]),
    ("cheer1", [8, 0]),
    ("back", [0, 1]),
    ("slide", [1, 1]),
    ("interact", [2, 1]),
    ("switch0", [3, 1]),
    ("switch1", [4, 1]),
    ("kick", [5, 1]),
    ("side", [6, 1]),
    ("shove", [7, 1]),
    ("shoveBack", [8, 1]),
    ("talk", [0, 2]),
    ("attackKick", [1, 2]),
    ("hang", [2, 2]),
    ("hold", [3, 2]),
    ("show", [4, 2]),
    ("behindBack", [5, 2]),
    ("run0", [6, 2]),
    ("run1", [7, 2]),
    ("run2", [8, 2]),
    ("attack0", [0, 3]),
    ("attack1", [1, 3]),
    ("attack2", [2, 3]),
    ("think", [3, 3]),
    ("down", [4, 3]),
    ("drag", [5, 3]),
    ("hurt", [6, 3]),
    ("wide", [7, 3]),
    ("rope", [8, 3]),
    ("walk0", [0, 4]),
    ("walk1", [1, 4]),
    ("walk2", [2, 4]),
    ("walk3", [3, 4]),
    ("walk4", [4, 4]),
    ("walk5", [5, 4]),
    ("walk6", [6, 4]),
    ("walk7", [7, 4]),
    ("fallDown", [8, 4]),
];
