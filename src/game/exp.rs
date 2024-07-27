const CONSTANT: f32 = 0.25;
const BASE: f32 = 50.0;

// let's try a sqrt approach:

// level = (sqrt(100(2experience+25))+50)/100
// experience =(level^2+level)/2*100-(level*100)

pub fn level_from_xp(xp: i32) -> i32 {
    // level = sqrt(XP) * constant
    // -> (1/c)^2 becomes the base -> b*xp^2-b
    ((xp as f32).sqrt() * CONSTANT).ceil() as i32
}

pub fn xp_from_level(level: i32) -> i32 {
    // XP = (level / constant)^2
    (level as f32 / CONSTANT).powf(2.0).ceil() as i32
}

pub fn lvl_xp(level: i32) -> i32 {
    // experience =(level^2+level)/2*100-(level*100)
    ((level.pow(2) + level) as f32 / 2.0 * BASE - level as f32 * BASE).ceil() as i32
}

fn xp_lvl(xp: i32) -> i32 {
    // level = (sqrt(100(2experience+25))+50)/100
    (((BASE * (2 * (xp + 25)) as f32).sqrt()) / BASE).ceil() as i32
}

#[test]
fn test_level_from_xp() {
    for level in 1..100 {
        let xp = xp_from_level(level);
        let new_level = level_from_xp(xp);
        assert_eq!(level, new_level);
    }
}

#[test]
fn test_xp_from_level() {
    for level in 1..100 {
        let xp = lvl_xp(level);
        let new_level = xp_lvl(xp);
        assert_eq!(level, new_level);
    }
}
