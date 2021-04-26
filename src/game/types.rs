// Structs for different game objects
use std::fmt;
use std::collections::{HashMap, HashSet};


#[derive(Debug)]
pub struct GameState {
    // Which telescopes (or other observing device) the player has obtained
    pub telescopes: HashSet<Telescope>,

    // The maximum resolving power available to the player
    pub max_power: i32,

    // Which objects are potentially observable, given the player's resolving power
    pub observables: HashSet<AstroObject>,

    // Which objects have been observed, and at what level of detail
    pub observed: HashMap<AstroObject, usize>
}

impl GameState {
    pub fn init_with_scopes(scopes: Vec<Telescope>) -> Self {
        GameState {
            telescopes: scopes.clone().into_iter().collect(),
            max_power: scopes.iter().max_by_key(|scope| scope.max_power ).unwrap().max_power,
            observables: HashSet::new(),
            observed: HashMap::new()
        }
    }

    pub fn fill_observables(&mut self, objects: &[AstroObject]) {
        for obj in objects.iter() {
            if obj.power_needed <= self.max_power {
                // NB: the HashSet will ensure uniqueness of objects.
                self.observables.insert(obj.clone());
            }
        }

    }
}


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum AstroObjectCategory {
    Star,
    Galaxy,
    AlienShip,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AstroObject {
    pub key: String,
    pub name: String,
    pub category: AstroObjectCategory,
    pub power_needed: i32, // may be redundant since AstroDetail has it
    pub detail: Vec<AstroDetail>
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AstroDetail {
    pub level: usize,
    pub power_needed: i32,
    pub discovery_text: String
}

impl fmt::Display for AstroObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\", a {:?} with {} levels of observable detail", self.name, self.category, self.detail.len())
    }
}

#[macro_export]
macro_rules! astro {
    ( $cat:tt: $key:tt, $name:tt, [$( $power:expr => $desc:tt )*] ) => {
        {
            let mut obj = AstroObject {
                key: String::from($key),
                name: String::from($name),
                category: AstroObjectCategory::$cat,
                power_needed: 9999,
                detail: vec![]
            };
            $(
                obj.detail.push(AstroDetail {
                    level: obj.detail.len(),
                    power_needed: $power,
                    discovery_text: String::from($desc)
                });

                if $power < obj.power_needed {
                    obj.power_needed = $power
                }
            )*

            obj
        }
    }
}

/// Telescopes

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Telescope {
    pub key: String,
    pub name: String,
    pub description: String,
    pub max_power: i32,
}

macro_rules! telescope {
    ( $key:tt => $name:tt, $power:tt, $desc:tt ) => {
        Telescope {
            key: String::from($key),
            name: String::from($name),
            max_power: $power,
            description: String::from($desc)
        }
    }
}

pub type TelescopeIndex = HashMap<String, Telescope>;

pub trait Keyed {
    type KeyedItem;
    fn get_by_key(&self, key: &str) -> Option<Self::KeyedItem>;
}

impl Keyed for TelescopeIndex {
    type KeyedItem = Telescope;
    fn get_by_key(&self, key: &str) -> Option<Telescope> {
        self.get(key).cloned()
    }
}


// Unused
// pub struct GameDisplay {
//     pub width: u32,
//     pub height: u32,
//     pub canvas_id: String,
// }