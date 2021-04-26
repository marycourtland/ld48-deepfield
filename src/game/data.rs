use super::types::*;

pub fn game_objects() -> Vec<AstroObject> {
    vec![
        astro!(Star: "sirius", "Sirius", [
            1 => "You can see the Dog Star! Your eyes must be working."
            10 => "You've observed Sirius B, the double star to Sirius A!"
        ]),
        astro!(Galaxy: "m31", "M31 Andromeda Galaxy", [
            3 => "Andromeda is larger than you thought."
            6 => "You can make out the core of the Andromeda Galaxy."
            12 => "Hey! You can see the spiral arms of the Andromeda Galaxy!"
        ]),
        astro!(AlienShip: "aliens", "aliens!!", [
            7 => "Huh, that looks weird."
            8 => "Umm.. it looks green?"
            11 => "WTF?? That is definitely a flying saucer!"
        ])
    ]
}

/// Return each telescope type, indexed by key
pub fn game_telescopes() -> TelescopeIndex {
    let mut telescopes = vec![
        telescope!("eye" => "The naked eye", 4, "Nature's built-in telescope"),
        telescope!("refractor_2in" => "Cheap 2\" refractor", 10, "You have a more powerful scope than Galileo did!"),
        telescope!("reflector_6in" => "Solid 6\" reflector", 13, "Reflectors are much more compact than refractors"),
        telescope!("dobsonian_20in" => "A 20\" Dobsonian", 18, "Basically a big bucket for light"),
        telescope!("keck" => "The Keck Observatory 10M", 20, "Built on sacred Hawaiian land"),
    ];
    telescopes
        .iter_mut()
        .fold(TelescopeIndex::new(),|mut hmap, t| { 
            hmap.insert(t.key.clone(), t.clone());
            hmap
        })
}


