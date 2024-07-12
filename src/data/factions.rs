use crate::models::faction::NewFaction;

pub const DEFAULT_FACTIONS: [NewFaction; 3] = [
    NewFaction {
        id: "H",
        name: "Humans",
    },
    NewFaction {
        id: "O",
        name: "Orcs",
    },
    NewFaction {
        id: "E",
        name: "Elves",
    },
];
