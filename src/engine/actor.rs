use crate::engine::log::actor_pos_update;

pub enum EActorType {
    Player,
    Enemy,
    NPC,
}

pub enum EAIType {
    Pacific,
    Aggressive,
}

pub enum EBattlerState {
    Alive,
    Dead,
}

#[repr(u8)]
pub enum EMenuAction {
    Attack,
    Defend,
    Techs,
    Items,
    Wait,
    LookGround,
    LookFront,
}

#[repr(u8)]
pub enum EAction {
    Attacking,
    UsingSkill,
    UsingItem,
    Waiting,
    LookingGround,
}

#[repr(u8)]
pub enum EModifiers {
    STR,
    STA,
    WIS,
    SPD,
}

#[repr(u8)]
pub enum EAlignment {
    Free = 1,
    Vaccine = 2,
    Data = 3,
    Virus = 4,
}

impl EAlignment {
    pub fn get_advantage(&self, target: &EAlignment) -> f32 {
        match (self, target) {
            (EAlignment::Data, EAlignment::Vaccine) => 2.0,
            (EAlignment::Vaccine, EAlignment::Virus) => 2.0,
            (EAlignment::Virus, EAlignment::Data) => 2.0,

            (EAlignment::Vaccine, EAlignment::Data) => 0.5,
            (EAlignment::Virus, EAlignment::Vaccine) => 0.5,
            (EAlignment::Data, EAlignment::Virus) => 0.5,
            _ => 1.0,
        }
    }
}

#[repr(u8)]
pub enum EAttribute {
    Fire = 1,
    Water = 2,
    Plant = 3,
    Eletric = 4,
    Earth = 5,
    Wind = 6,
    Light = 7,
    Dark = 8,
    Neutral = 9,
}

impl EAttribute {
    pub fn get_advantage(&self, target: &EAttribute) -> f32 {
        match (self, target) {
            (EAttribute::Fire, EAttribute::Plant) => 1.5,
            (EAttribute::Plant, EAttribute::Water) => 1.5,
            (EAttribute::Water, EAttribute::Fire) => 1.5,

            (EAttribute::Eletric, EAttribute::Wind) => 1.5,
            (EAttribute::Wind, EAttribute::Earth) => 1.5,
            (EAttribute::Earth, EAttribute::Eletric) => 1.5,

            (EAttribute::Light, EAttribute::Dark) => 1.5,
            (EAttribute::Dark, EAttribute::Light) => 1.5,

            (EAttribute::Plant, EAttribute::Fire) => 0.75,
            (EAttribute::Water, EAttribute::Plant) => 0.75,
            (EAttribute::Fire, EAttribute::Water) => 0.75,

            (EAttribute::Wind, EAttribute::Eletric) => 0.75,
            (EAttribute::Earth, EAttribute::Wind) => 0.75,
            (EAttribute::Eletric, EAttribute::Earth) => 0.75,
            _ => 1.0,
        }
    }
}

struct Aspects {
    alignment: EAlignment,
    attribute: EAttribute,
}

struct Status {
    max_hp: i32,
    max_mp: i32,
    str: i32,
    sta: i32,
    wis: i32,
    spd: i32,
}

struct Battler {
    hp: i32,
    mp: i32,
    state: EBattlerState,
    status: Status,
    modifiers: Vec<Option<EModifiers>>,
    aspects: Aspects,
}

impl Battler {
    fn get_atk(&self) -> i32 {
        self.status.str
    }

    fn get_def(&self) -> i32 {
        self.status.sta
    }

    fn get_spd(&self) -> i32 {
        self.status.spd
    }

    fn attack(&self, mut b: Battler) {
        let diff = self.get_atk() - b.get_def();
        let power_mod = diff / 500 * self.get_atk() + self.get_atk();
        let advantages = (
            self.aspects.alignment.get_advantage(&b.aspects.alignment),
            self.aspects.attribute.get_advantage(&b.aspects.attribute),
        );

        let damage = power_mod * (advantages.0 * advantages.1).round() as i32 / 20;

        b.hp -= damage;
    }

    fn update_check(mut self) {
        if self.hp < 1 {
            self.state = EBattlerState::Dead;
        }
    }
}

pub struct Actor {
    id: i16,
    role: EActorType,
    name: String,
    battler: Battler,
    position: (i16, i16),
    action_timer: i16,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            name: "Teste".to_owned(),
            id: 1,
            role: EActorType::NPC,
            battler: Battler {
                hp: 100,
                mp: 100,
                state: EBattlerState::Alive,
                status: Status {
                    max_hp: 100,
                    max_mp: 100,
                    str: 50,
                    sta: 50,
                    wis: 50,
                    spd: 50,
                },
                modifiers: Vec::new(),
                aspects: Aspects {
                    alignment: EAlignment::Free,
                    attribute: EAttribute::Neutral,
                },
            },
            position: (5, 5),
            action_timer: 0,
        }
    }
}

impl Actor {
    fn can_act(self) -> bool {
        self.action_timer > 500
    }

    fn update_action_timer(mut self) {
        self.action_timer += self.battler.get_spd() as i16;
    }

    pub fn new_player(name: String) -> Self {
        Self {
            name,
            role: EActorType::Player,
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_position(&self) -> (i16, i16) {
        self.position
    }

    pub fn move_to_direction(&mut self, mv: (i16, i16)) -> &mut Actor {
        self.position.0 += mv.0;
        self.position.1 += mv.1;

        actor_pos_update(&self.name, &self.position);
        self
    }

    pub fn take_action(self, action: EAction, a_target: Option<Actor>) {
        match action {
            EAction::Attacking => {
                if let Some(x) = a_target {
                    self.battler.attack(x.battler);
                }
            }
            EAction::Waiting => {
                todo!()
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::log::tail;

    use super::*;

    #[test]
    fn test_move() {
        let mut actor = Actor::default();

        let new = actor.move_to_direction((1, 0));

        assert_eq!(new.position, (6i16, 5i16));
        tail();
    }
}
