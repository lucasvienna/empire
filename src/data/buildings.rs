use crate::models::building::NewBuilding;

pub const DEFAULT_BUILDINGS: [NewBuilding; 3] = [
    NewBuilding {
        name: "Castle",
        faction: Some("H"),
        max_level: 15,
    },
    NewBuilding {
        name: "Honor Hold",
        faction: Some("O"),
        max_level: 15,
    },
    NewBuilding {
        name: "Council Hall",
        faction: Some("E"),
        max_level: 15,
    },
];
