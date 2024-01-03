use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

pub struct Input {
    reactions: HashMap<String, Reaction>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let reactions = value
            .lines()
            .map(str::trim)
            .map(Reaction::from)
            .map(|reaction| (reaction.output.name.clone(), reaction))
            .collect();
        Self { reactions }
    }
}

#[derive(Clone, Debug)]
struct Chemical {
    name: String,
    amount: usize,
}

impl From<&str> for Chemical {
    fn from(value: &str) -> Self {
        let (amount, name) = value.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();
        let name = name.to_string();
        Self { amount, name }
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    output: Chemical,
    input: Vec<Chemical>,
}

impl From<&str> for Reaction {
    fn from(value: &str) -> Self {
        let (input, output) = value.split_once(" => ").unwrap();
        let output: Chemical = output.into();
        let input: Vec<_> = input.split(", ").map(Chemical::from).collect();
        Self { output, input }
    }
}

fn calculate(reactions: &HashMap<String, Reaction>, total_fuel: usize) -> usize {
    let mut total_ore = 0;
    let fuel = Chemical {
        name: "FUEL".into(),
        amount: total_fuel,
    };
    let mut chemicals = VecDeque::from([fuel]);
    let mut leftovers: HashMap<String, usize> = HashMap::new();
    while let Some(chemical) = chemicals.pop_front() {
        if chemical.name == "ORE" {
            total_ore += chemical.amount;
        } else {
            let leftover = leftovers.entry(chemical.name.clone()).or_default();
            let reaction = &reactions[&chemical.name];
            if *leftover >= chemical.amount {
                *leftover -= chemical.amount;
            } else {
                let amount = chemical.amount - *leftover;
                let multiplier = amount.div_ceil(reaction.output.amount);
                *leftover = reaction.output.amount * multiplier - amount;
                for mut chems in reaction.input.iter().cloned() {
                    chems.amount *= multiplier;
                    chemicals.push_back(chems);
                }
            }
        }
    }
    total_ore
}

pub fn part1(input: &Input) -> usize {
    calculate(&input.reactions, 1)
}

pub fn part2(input: &Input) -> usize {
    const MAX_ORE: usize = 1_000_000_000_000;

    let ore = calculate(&input.reactions, 1);
    let max_fuel = MAX_ORE / ore;

    let mut low = max_fuel;
    let mut high = max_fuel * 2;
    while low <= high {
        let mid = (high + low) / 2;
        let ore = calculate(&input.reactions, mid);
        match ore.cmp(&MAX_ORE) {
            Ordering::Equal => break,
            Ordering::Greater => high = mid - 1,
            Ordering::Less => low = mid + 1,
        }
    }
    low - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        const INPUT: &str = r#"10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL"#;

        assert_eq!(part1(&INPUT.into()), 31);
    }

    #[test]
    fn test_part1_ex2() {
        const INPUT: &str = r#"9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL"#;

        assert_eq!(part1(&INPUT.into()), 165);
    }

    #[test]
    fn test_part1_ex3() {
        const INPUT: &str = r#"157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;

        assert_eq!(part1(&INPUT.into()), 13312);
    }

    #[test]
    fn test_part1_ex4() {
        const INPUT: &str = r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF"#;

        assert_eq!(part1(&INPUT.into()), 180697);
    }

    #[test]
    fn test_part1_ex5() {
        const INPUT: &str = r#"171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX"#;

        assert_eq!(part1(&INPUT.into()), 2210736);
    }

    #[test]
    fn test_part2_ex1() {
        const INPUT: &str = r#"157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#;

        assert_eq!(part2(&INPUT.into()), 82892753);
    }

    #[test]
    fn test_part2_ex2() {
        const INPUT: &str = r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF"#;

        assert_eq!(part2(&INPUT.into()), 5586022);
    }

    #[test]
    fn test_part2_ex3() {
        const INPUT: &str = r#"171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX"#;

        assert_eq!(part2(&INPUT.into()), 460664);
    }
}
