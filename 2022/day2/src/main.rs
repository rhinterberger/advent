use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");
    let all_moves = parse_input(input_string);

    let total_score = play_game(&all_moves);
    println!("{total_score}");

    let total_score = play_game2(&all_moves);
    println!("{total_score}");
}


fn play_game(moves: &Vec<[i32;2]>) -> i32 {
    moves.iter().fold(0, |points, current_move|
        points + score(current_move)
    )
}

fn play_game2(moves: &Vec<[i32;2]>) -> i32 {
    moves.iter().fold(0, |points, current_move|
        points + score(&select_move(current_move))
    )
}

fn score(game_move: &[i32; 2]) -> i32 {
    let points = (game_move[0] + game_move[1]) % 3 * 3 + game_move[1];
    points
}

fn select_move(game_move: &[i32;2]) -> [i32;2] {
    let my_move = 1 + (4 + game_move[1] - game_move[0]) % 3;
    [game_move[0],my_move]
}

fn parse_input(input_string: String) -> Vec<[i32;2]> {
    input_string
        .lines()
        .map(prepare_moves )
        .collect::<Vec<[i32;2]>>()
}

fn prepare_moves(movestring: &str) -> [i32;2] {

    let line = movestring.replace(" ", "");
    let mut chars = line.chars();

    [convert_letters(chars.next().unwrap()), convert_letters(chars.next().unwrap())]
}

fn convert_letters(game_move: char) -> i32 {
    match game_move {
        'C'|'X' => 1,
        'B'|'Y' => 2,
        'A'|'Z' => 3,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_win() {
        assert_eq!(score(&[convert_letters('A'),convert_letters('Y')]),8);
    }

    #[test]
    fn should_lose() {
        assert_eq!(score(&[convert_letters('B'),convert_letters('X')]),1);
    }

    #[test]
    fn should_draw() {
        assert_eq!(score(&[convert_letters('C'),convert_letters('Z')]),6);
    }

    const ALL_MOVES :&str = "A Y
B X
C Z";

    #[test]
    fn should_return_15() {
        let input = parse_input(ALL_MOVES.to_string());
        assert_eq!(play_game(&input),15);
    }

    #[test]
    fn should_return_12() {
        let input = parse_input(ALL_MOVES.to_string());

        assert_eq!(play_game2(&input),12);
    }

}