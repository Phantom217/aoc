use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub(crate) struct Solver(());

impl Solver {
    pub fn new() -> Self {
        let solver = Self(());
        assert_solver_day!(solver);
        solver
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        21
    }

    fn solve(&self, input: &str) -> Solution {
        let foods = input.lines().map(Food::from).collect::<Vec<Food>>();

        let part1 = part1(&foods);
        let part2 = part2(&foods);

        (part1, part2).into()
    }
}

fn part1(foods: &[Food]) -> usize {
    let all_ingredients = Food::all_ingredients(foods);
    let allergen_map = Food::allergen_map(foods);

    let has_allergens = allergen_map
        .values()
        .flat_map(HashSet::iter)
        .cloned()
        .collect();
    let no_allergens: HashSet<_> = all_ingredients.difference(&has_allergens).collect();

    let mut count = 0;
    for Food {
        ingredients,
        allergens: _,
    } in foods
    {
        count += ingredients
            .iter()
            .filter(|i| no_allergens.contains(i))
            .count();
    }
    count
}

fn part2(foods: &[Food]) -> String {
    let mut allergen_map: Vec<_> = Food::allergen_map(foods).into_iter().collect();

    for idx in 1..allergen_map.len() {
        allergen_map.sort_by_key(|(_, ingredients)| ingredients.len());

        let (single_ingredients, multi_ingredients) = allergen_map.split_at_mut(idx);

        if let Some(current_ingredient) = single_ingredients.last().unwrap().1.iter().next() {
            for ingredients in multi_ingredients {
                ingredients.1.retain(|i| i != current_ingredient);
            }
        }
    }

    allergen_map.sort_by_key(|(allergen, _)| *allergen);

    let mut display = String::new();
    for (_, ingredients) in allergen_map {
        let ingredient = ingredients.iter().next().unwrap();
        display.push_str(ingredient);
        display.push(',');
    }
    display.pop();

    display
}

#[derive(Debug, PartialEq, Eq)]
struct Food<'f> {
    ingredients: HashSet<&'f str>,
    allergens: HashSet<&'f str>,
}

impl Food<'_> {
    fn all_ingredients(foods: &[Self]) -> HashSet<&str> {
        foods
            .iter()
            .flat_map(|food| food.ingredients.iter())
            .cloned()
            .collect()
    }

    fn allergen_map(foods: &[Self]) -> HashMap<&str, HashSet<&str>> {
        let mut allergen_map = HashMap::new();
        for Food {
            ingredients,
            allergens,
        } in foods
        {
            for a in allergens.iter() {
                let set = allergen_map
                    .entry(*a)
                    .or_insert_with(|| ingredients.clone());
                *set = &*set & ingredients;
            }
        }
        allergen_map
    }
}

impl<'f, 's: 'f> From<&'s str> for Food<'f> {
    fn from(line: &'s str) -> Self {
        let mut iter = line.split(" (contains ");
        let ingredients = iter.next().unwrap().split(' ').collect();
        let allergens = iter
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(',')
            .map(str::trim)
            .collect();
        Self {
            ingredients,
            allergens,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_parse() {
        let expected = vec![
            Food {
                ingredients: HashSet::from(
                    ["mxmxvkd", "kfcds", "sqjhc", "nhms"]
                        .iter()
                        .cloned()
                        .collect(),
                ),
                allergens: HashSet::from(["dairy", "fish"].iter().cloned().collect()),
            },
            Food {
                ingredients: HashSet::from(
                    ["trh", "fvjkl", "sbzzf", "mxmxvkd"]
                        .iter()
                        .cloned()
                        .collect(),
                ),
                allergens: HashSet::from(["dairy"].iter().cloned().collect()),
            },
            Food {
                ingredients: HashSet::from(["sqjhc", "fvjkl"].iter().cloned().collect()),
                allergens: HashSet::from(["soy"].iter().cloned().collect()),
            },
            Food {
                ingredients: HashSet::from(["sqjhc", "mxmxvkd", "sbzzf"].iter().cloned().collect()),
                allergens: HashSet::from(["fish"].iter().cloned().collect()),
            },
        ];
        let actual = INPUT.lines().map(Food::from).collect::<Vec<Food>>();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_food_candidates() {
        let dairy: HashSet<&str> = HashSet::from(["mxmxvkd"].iter().cloned().collect());
        let fish: HashSet<&str> = HashSet::from(["mxmxvkd", "sqjhc"].iter().cloned().collect());
        let soy: HashSet<&str> = HashSet::from(["sqjhc", "fvjkl"].iter().cloned().collect());
        let expected: HashMap<&str, HashSet<&str>> = HashMap::from(
            [("dairy", dairy), ("fish", fish), ("soy", soy)]
                .iter()
                .cloned()
                .collect(),
        );

        let foods = INPUT.lines().map(Food::from).collect::<Vec<Food>>();
        let actual = Food::allergen_map(&foods);

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let foods = INPUT.lines().map(Food::from).collect::<Vec<Food>>();
        let expected = 5;
        let actual = part1(&foods);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let foods = INPUT.lines().map(Food::from).collect::<Vec<Food>>();
        let expected = String::from("mxmxvkd,sqjhc,fvjkl");
        let actual = part2(&foods);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day21.txt");

        let expected: Solution = (
            2061,
            String::from("cdqvp,dglm,zhqjs,rbpg,xvtrfz,tgmzqjz,mfqgx,rffqhl"),
        )
            .into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
