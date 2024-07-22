use crate::models::building::NewBuilding;
use crate::models::building_level::NewBuildingLevel;

pub const DEFAULT_BUILDINGS: [NewBuilding; 3] = [
    NewBuilding {
        name: "Castle",
        faction: 0,
        max_level: 15,
        max_count: 1,
    },
    NewBuilding {
        name: "Honor Hold",
        faction: 1,
        max_level: 15,
        max_count: 1,
    },
    NewBuilding {
        name: "Council Hall",
        faction: 2,
        max_level: 15,
        max_count: 1,
    },
];
