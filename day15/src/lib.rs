const PROPERTIES: usize = 5;
const INGREDIENTS: usize = 4;

const INITIAL_TEASPOONS: i32 = 100;

const CALORIE_TARGET: i32 = 500;

type Ingredient = [i32; PROPERTIES];

fn parse_ingredient(line: &str) -> Ingredient {
    let mut ingredient = [0; PROPERTIES];
    line.split(' ')
        .skip(2)
        .step_by(2)
        .zip(ingredient.iter_mut())
        .for_each(|(val, dest)| *dest = val.trim_end_matches(',').parse().unwrap());
    ingredient
}

fn score(ingredient: &Ingredient) -> i32 {
    ingredient
        .iter()
        .take(PROPERTIES - 1)
        .try_fold(1, |acc, &cur| if cur < 0 { None } else { Some(acc * cur) })
        .unwrap_or(0)
}

fn spoonify(accumulator: Ingredient, ingredient: &Ingredient, teaspoons: i32) -> Ingredient {
    let mut next = accumulator;

    ingredient
        .iter()
        .zip(next.iter_mut())
        .for_each(|(&property, dest)| *dest += teaspoons * property);

    next
}

fn recurse(
    ingredients: &[Ingredient],
    teaspoons_left: i32,
    accumulator: Ingredient,

    part1: &mut i32,
    part2: &mut i32,
) {
    let (ingredient, ingredients) = ingredients.split_first().unwrap();

    if ingredients.is_empty() {
        // If this is the last ingredient, we must use up all the remaining teaspoons,
        // and we've also reached the end of the line
        let next = spoonify(accumulator, ingredient, teaspoons_left);
        let next_score = score(&next);

        // Maximize part 1 indiscriminately
        if *part1 < next_score {
            *part1 = next_score;
        }

        // Maximize part 2 indiscriminately, considering only recipes which have the given number of calories
        if next.last() == Some(&CALORIE_TARGET) && *part2 < next_score {
            *part2 = next_score;
        }
    } else {
        // Otherwise, recurse through all possibilities
        (0..=teaspoons_left).for_each(|teaspoons_used| {
            let next = spoonify(accumulator, ingredient, teaspoons_used);
            recurse(
                ingredients,
                teaspoons_left - teaspoons_used,
                next,
                part1,
                part2,
            )
        })
    }
}

#[inline]
pub fn solve() -> (i32, i32) {
    let mut ingredients = [[0; PROPERTIES]; INGREDIENTS];

    include_str!("input.txt")
        .lines()
        .map(parse_ingredient)
        .zip(ingredients.iter_mut())
        .for_each(|(val, dest)| *dest = val);

    let mut part1 = 0;
    let mut part2 = 0;

    recurse(
        &ingredients,
        INITIAL_TEASPOONS,
        [0; PROPERTIES],
        &mut part1,
        &mut part2,
    );

    (part1, part2)
}
