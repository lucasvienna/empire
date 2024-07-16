use crate::models::faction::NewFaction;

pub const DEFAULT_FACTIONS: [NewFaction; 4] = [
    NewFaction {
        id: 0,
        name: "Neutral",
    },
    NewFaction {
        id: 1,
        name: "Humans",
    },
    NewFaction {
        id: 2,
        name: "Orcs",
    },
    NewFaction {
        id: 3,
        name: "Elves",
    },
];
