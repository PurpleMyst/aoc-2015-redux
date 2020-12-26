// TODO: optimize this to do just one pass
use std::convert::TryFrom;

const PROPERTIES: usize = 5;
const INGREDIENTS: usize = 4;

const INITIAL_TEASPOONS: i16 = 100;

const CALORIE_TARGET: i16 = 500;

type Ingredient = [i16; PROPERTIES];
type Ingredients = [Ingredient; INGREDIENTS];

fn parse_ingredient(line: &str) -> Ingredient {
    let mut ingredient = [0; PROPERTIES];
    line.split(' ')
        .skip(2)
        .step_by(2)
        .zip(ingredient.iter_mut())
        .for_each(|(val, dest)| *dest = val.trim_end_matches(',').parse().unwrap());
    ingredient
}

fn score(ingredient: &Ingredient) -> u64 {
    ingredient
        .iter()
        .take(PROPERTIES - 1)
        .try_fold(1, |acc, &property| {
            Result::<u64, std::num::TryFromIntError>::Ok(acc * u64::try_from(property)?)
        })
        .unwrap_or(0)
}

fn recurse(
    ingredients: &[Ingredient],
    teaspoons_left: i16,
    base: Ingredient,
    filter: &impl Fn(&Ingredient) -> bool,
) -> Option<Ingredient> {
    if teaspoons_left == 0 {
        return Some(base);
    }

    if let Some((ingredient, ingredients)) = ingredients.split_first() {
        (0..=teaspoons_left)
            .filter_map(|teaspoons_used| {
                let mut base = base;

                ingredient
                    .iter()
                    .zip(base.iter_mut())
                    .for_each(|(&property, dest)| *dest += teaspoons_used * property);

                recurse(ingredients, teaspoons_left - teaspoons_used, base, filter)
            })
            .filter(filter)
            .max_by_key(score)
    } else {
        if teaspoons_left == 0 {
            Some(base)
        } else {
            None
        }
    }
}

#[inline]
pub fn solve() -> (u64, u64) {
    let mut ingredients: Ingredients = [[0; PROPERTIES]; INGREDIENTS];

    include_str!("input.txt")
        .lines()
        .map(parse_ingredient)
        .zip(ingredients.iter_mut())
        .for_each(|(val, dest)| *dest = val);

    (
        score(&recurse(&ingredients, INITIAL_TEASPOONS, [0; PROPERTIES], &|_| true).unwrap()),
        score(
            &recurse(
                &ingredients,
                INITIAL_TEASPOONS,
                [0; PROPERTIES],
                &|ingredient| ingredient.last() == Some(&CALORIE_TARGET),
            )
            .unwrap(),
        ),
    )
}
