use regex::Regex;
use std::cmp::Ordering;
use std::str::FromStr;

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

#[derive(PartialEq, Debug)]
struct Group {
    units: i32,
    hit_points: i32,
    attack_damage: i32,
    attack_type: AttackType,
    initiative: i32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
}

impl Group {

    fn effective_power(&self) -> i32
    {
        self.units * self.attack_damage
    }

    fn damage_from_attack(&self, group: &Group) -> i32
    {
        let multiplier = if
            self.immunities.contains(&group.attack_type) { 0 }
            else if self.weaknesses.contains(&group.attack_type) { 2 }
            else { 1 };
        group.effective_power() * multiplier
    }

    fn apply_damage_from_attack(&mut self, group: &Group)
    {
        let units_lost = self.damage_from_attack(&group) / self.hit_points;
        self.units -= units_lost;
    }

    fn better_target(attacker: &Group, target_1: &Group, target_2: &Group) -> Ordering
    {
        // if target_1.damage_from_attack(attacker) < target_2.damage_from_attack(attacker) {
        //     return Ordering::Less;
        // } else
        if target_1.effective_power() < target_2.effective_power() {
            return Ordering::Less;
        }
        if target_1.initiative < target_2.initiative {
            return Ordering::Less;
        }
        Ordering::Equal

    }

    fn find_target(&self, enemies: &mut Vec<&Group>) -> Option<&Group>
    {
        if enemies.len() == 0 {
            return None;
        }
        let target: Option<&Group> = None;
        // enemies.sort_unstable_by(Group::better_target);
        target
    }

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
    const DEFAULT_GROUP: Group = Group {
        units: 10,
        hit_points: 10,
        attack_damage: 10,
        attack_type: AttackType::Fire,
        initiative: 4,
        weaknesses: vec![],
        immunities: vec![],
    };

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
        // What if the group has no weaknesses or immunities?
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
        // What if "weak to" and "immune to" are in a different order?
        let t = "1907 units each with 1530 hit points (immune to fire, bludgeoning; weak to radiation) with an attack that does 7 fire damage at initiative 9";
        let group: Group = t.parse().unwrap();
        assert_eq!(group.units, 1907);
        assert_eq!(group.hit_points, 1530);
        assert_eq!(group.attack_damage, 7);
        assert_eq!(group.attack_type, AttackType::Fire);
        assert_eq!(group.initiative, 9);
        assert_eq!(group.weaknesses, vec![AttackType::Radiation]);
        assert_eq!(group.immunities, vec![AttackType::Fire, AttackType::Bludgeoning]);
    }

    #[test]
    fn test_doing_damage()
    {
        let attacker = Group {units: 25, attack_damage: 3, ..DEFAULT_GROUP};
        let mut defender = Group {units: 10, hit_points: 10, ..DEFAULT_GROUP};
        defender.apply_damage_from_attack(&attacker);
        assert_eq!(defender.units, 3);
    }

    #[test]
    fn test_sorting_targets()
    {
        let target_1 =  Group {hit_points: 1, initiative: 3, ..DEFAULT_GROUP};
        let target_2 =  Group {hit_points: 1, initiative: 1, ..DEFAULT_GROUP};
        let target_3 =  Group {hit_points: 1, initiative: 12, ..DEFAULT_GROUP};
        let attacker = Group {..DEFAULT_GROUP};
        let mut targets = vec![target_1, target_2, target_3];
        let sort_function = |t1: &Group, t2: &Group| { return Group::better_target(&attacker, t1, t2);};

        let aa = targets.sort_by(sort_function);
        println!();
    }

    #[test]
    fn test_sorting_i()
    {
        let mut xs: Vec<i32> = vec! [8, 1, 9, 10, 7, 90];
        let sort_fn = |x1: &i32, x2: &i32| { if x1 < x2 {return Ordering::Less;} else {return Ordering::Greater;};};
        xs.sort_by(sort_fn);
        println!();
    }

}
