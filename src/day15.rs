// use std::collections::HashMap;

use std::iter::Sum;

#[derive(Debug, Clone, Copy)]
struct Ingredient(i32,i32,i32,i32,i32);

trait Score<T> {
    fn score(self) -> T;
}

impl Score<i32> for Ingredient {
    fn score(self) -> i32 {
        self.0.max(0) * self.1.max(0) * self.2.max(0) * self.3.max(0) 
    }
}

impl std::ops::Mul<i32> for &Ingredient 
{
    type Output = Ingredient;
    fn mul(self, a: i32) -> Ingredient {
        Ingredient(self.0 * a, self.1 * a, self.2 * a, self.3 * a, self.4 * a)
    }
}

impl std::ops::Add<Ingredient> for Ingredient {
    type Output = Ingredient;
    fn add(self, a: Ingredient) -> Ingredient {
        return Ingredient(self.0 + a.0, self.1 + a.1, self.2 + a.2, self.3 + a.3, self.4 + a.4);
    }
}

impl Sum for Ingredient {
    fn sum<I>(iter: I) -> Self 
        where I: Iterator<Item = Ingredient>
    {
        iter.reduce(|x,y| { x + y })
            .unwrap_or(Ingredient(0,0,0,0,0))
    }
}

fn parse(txt: &str) -> Vec<Ingredient> {
    txt.lines().map(|x| {
        let x2 = x.replace(",","");
        let words: Vec<&str> = x2.split(' ').collect();
        Ingredient(words[2].parse().unwrap(),
            words[4].parse().unwrap(),
            words[6].parse().unwrap(),
            words[8].parse().unwrap(),
            words[10].parse().unwrap())
    }).collect()
}

fn total_score(amounts: [u8;4], ingredients: &Vec<Ingredient>) -> (i32, i32) {
    let totals: Ingredient = ingredients.iter()
        .zip(amounts.iter())
        .map(|(i, &c)| i * c as i32).sum();
    (totals.score(), totals.4)
}

fn hill_climb(start: &[u8;4], ingredients: &Vec<Ingredient>) -> [u8;4] {
    let mut amounts = *start;
    let mut sign: [i8; 4] = [2; 4];
    let mut max = total_score(amounts, ingredients);
    loop {
        let mut optimum_found = true;
        for i in 0..=3 {
            let k = (i as i8 + sign[i]).rem_euclid(4) as usize;
            loop {
                amounts[i] = amounts[i].saturating_add_signed(sign[i]);
                amounts[k] = amounts[k].saturating_add_signed(-sign[i]);
                let score = total_score(amounts, ingredients);
                
                if score > max {
                    max = score;
                    optimum_found = false;
                } else if score < max {
                    sign[i] *= -1;
                    // amounts[i] = amounts[i].saturating_add_signed(-sign[i]);
                    // amounts[k] = amounts[k].saturating_add_signed(sign[i]);
                    break;
                }
            }
        }
        if optimum_found {
            break
        }
    }
    amounts
}

fn find_nearby_max(start: &[u8;4], ingredients: &Vec<Ingredient>, calories: Option<i32>) -> i32 {
    let k = 7;
    (-k..k).flat_map(|a| {
        (-k..k).flat_map(move |b| {
            (-k..k).map(move |c| {
                let d = -a-b-c;
                let x = [
                    start[0].saturating_add_signed(a),
                    start[1].saturating_add_signed(b),
                    start[2].saturating_add_signed(c),
                    start[3].saturating_add_signed(d),
                ];
                let score = total_score(x, ingredients);
                if calories.is_none() || calories.unwrap() == score.1 {
                    return score.0;
                } else {
                    return 0;
                }
            })
        })
    }).max().unwrap()
}

pub fn day15(input: &str) -> (String, String) {
    //let start = Instant::now();
    let data = parse(input);
    let start: [u8; 4] = [25,25,25,25];
    let near_opt = hill_climb(&start, &data);

    let part1 = find_nearby_max(&near_opt, &data, None);
    let part2 = find_nearby_max(&near_opt, &data, Some(500));
    
    (format!("{part1:?}"), format!("{part2}"))
}