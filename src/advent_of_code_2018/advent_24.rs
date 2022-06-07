use std::str::FromStr;
use regex::Regex;

pub fn solve()
{}


#[derive(PartialEq, Debug)]
enum AttackType {
    Fire,
    Radiation,
    Bludgeoning,
    Slashing,
    Cold,
}

impl FromStr for AttackType {

    type Err = ();

    fn from_str(input: &str) -> Result<AttackType, Self::Err> {
        let lowercase_input: &str = &input.to_lowercase();
        match lowercase_input {
            "fire"  => Ok(AttackType::Fire),
            "radiation"  => Ok(AttackType::Radiation),
            "bludgeoning"  => Ok(AttackType::Bludgeoning),
            "slashing" => Ok(AttackType::Slashing),
            "cold" => Ok(AttackType::Cold),
            _      => Err(()),
        }
    }
}

struct Group {
    units: i32,
    hit_points: i32,
    attack_damage: i32,
    attack_type: AttackType,
    initiative: i32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

impl FromStr for Group {

    type Err = ();

    fn from_str(input: &str) -> Result<Group, Self::Err> {
        let lowercase_input: &str = &input.to_lowercase();
        let re = Regex::new(
            r"(?P<units>[\d]+) units each with (?P<hit_points>[\d]+) hit points (?P<mods>.*\s)?with an attack that does (?P<attack_damage>[\d]+) (?P<attack_type>[\w]+) damage at initiative (?P<initiative>[\d]+)"
        ).unwrap();
        let matches = re.captures(lowercase_input).unwrap();
        let units = matches.name("units").unwrap().as_str().parse().unwrap();
        let hit_points = matches.name("hit_points").unwrap().as_str().parse().unwrap();
        let attack_damage = matches.name("attack_damage").unwrap().as_str().parse().unwrap();
        let attack_type = matches.name("attack_type").unwrap().as_str().parse().unwrap();
        let initiative = matches.name("initiative").unwrap().as_str().parse().unwrap();

        let re_weak = Regex::new(r"weak to ([\w\s,]+)").unwrap();
        let re_immune = Regex::new(r"immune to ([\w\s,]+)").unwrap();

        let weak_matches = re_weak.captures(lowercase_input);
        let immune_matches = re_immune.captures(lowercase_input);

        let weaknesses: Vec<AttackType> = if weak_matches.is_some() {
            weak_matches.unwrap().get(1).unwrap().as_str().split(",").map(|s| s.replace("weak to ", "").trim().parse().unwrap()).collect()
        } else {
            vec![]
        };
        let immunities: Vec<AttackType> = if immune_matches.is_some() {
            immune_matches.unwrap().get(1).unwrap().as_str().split(",").map(|s| s.replace("immune to", "").trim().parse().unwrap()).collect()
        } else {
            vec![]
        };

        Ok(Group {
            units,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses,
            immunities,
        })
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parse_group_1()
    {
        let t = "18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10";
        let group: Group = t.parse().unwrap();
        assert_eq!(group.units, 18);
        assert_eq!(group.hit_points, 729);
        assert_eq!(group.attack_damage, 8);
        assert_eq!(group.attack_type, AttackType::Radiation);
        assert_eq!(group.initiative, 10);
        assert_eq!(group.weaknesses, vec![AttackType::Fire]);
        assert_eq!(group.immunities, vec![AttackType::Cold, AttackType::Slashing]);
    }

    #[test]
    fn test_parse_group_2()
    {
        let t = "371 units each with 8033 hit points with an attack that does 204 bludgeoning damage at initiative 15";
        let group: Group = t.parse().unwrap();
        assert_eq!(group.units, 371);
        assert_eq!(group.hit_points, 8033);
        assert_eq!(group.attack_damage, 204);
        assert_eq!(group.attack_type, AttackType::Bludgeoning);
        assert_eq!(group.initiative, 15);
        assert_eq!(group.weaknesses, vec![]);
        assert_eq!(group.immunities, vec![]);
    }

    #[test]
    fn test_parse_group_3()
    {
        let t = "1907 units each with 1530 hit points (immune to fire, bludgeoning; weak to radiation) with an attack that does 7 fire damage at initiative 9";
        let group: Group = t.parse().unwrap();
        assert_eq!(group.weaknesses, vec![AttackType::Radiation]);
        assert_eq!(group.immunities, vec![AttackType::Fire, AttackType::Bludgeoning]);
    }
}
