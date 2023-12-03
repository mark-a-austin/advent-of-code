use regex::Regex;
advent_of_code::solution!(2);

fn extract_color_sets(reg: Regex, sets: &&str) -> u32 {
    let colour_matches: Vec<Vec<&str>> = reg
        .captures_iter(sets)
        .map(|c| {
            c.iter()
                .map(|m| m.unwrap().as_str().split_whitespace().next().unwrap())
                .collect()
        })
        .collect();

    let colour_max: u32 = colour_matches
        .into_iter()
        .map(|l| l[0].parse::<u32>().unwrap())
        .max()
        .unwrap();

    colour_max
}

fn is_valid_game(game: &str) -> u32 {
    let valid_red = 12;
    let valid_green = 13;
    let valid_blue = 14;

    let find_digit = Regex::new(r"\d+").unwrap();
    let find_red = Regex::new(r"\d+ red").unwrap();
    let find_blue = Regex::new(r"\d+ blue").unwrap();
    let find_green = Regex::new(r"\d+ green").unwrap();

    // Grabbing game id

    let game_split: Vec<&str> = game.split(':').collect();
    let game_id: Vec<&str> = find_digit
        .find_iter(game_split.first().unwrap())
        .map(|mat| mat.as_str())
        .collect();
    let game_id: u32 = game_id[0].parse().unwrap();

    // Extracting sets
    let sets = game_split.last().unwrap();

    // Extracting reds, blues, and greens
    let reds = extract_color_sets(find_red, sets);
    let blues = extract_color_sets(find_blue, sets);
    let greens = extract_color_sets(find_green, sets);

    let is_valid: bool = reds <= valid_red && blues <= valid_blue && greens <= valid_green;
    if is_valid {
        return game_id;
    }
    return 0;
}

fn find_minimum_of_game(game: &str) -> u32 {
    let find_red = Regex::new(r"\d+ red").unwrap();
    let find_blue = Regex::new(r"\d+ blue").unwrap();
    let find_green = Regex::new(r"\d+ green").unwrap();

    let game_split: Vec<&str> = game.split(':').collect();
    let sets = game_split.last().unwrap();

    let reds = extract_color_sets(find_red, sets);
    let blues = extract_color_sets(find_blue, sets);
    let greens = extract_color_sets(find_green, sets);

    reds * blues * greens
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let valid_games: Vec<u32> = lines.into_iter().map(|g| is_valid_game(g)).collect();
    return Some(valid_games.into_iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let minimum_powers: Vec<u32> = lines.into_iter().map(|g| find_minimum_of_game(g)).collect();
    return Some(minimum_powers.into_iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
