#[allow(dead_code)]
pub fn solution() {
    println!("Part 1: {}", deterministic_score(8, 7));
}

/// Plays a game of dirac dice with a deterministic d100 (rolls 1, then 2, ...), and returns
/// the score of the losing player by the number of times the die was rolled during the game.
/// Dirac dice is played by rolling a die 3 times, then moving a pawn that many times
/// around a board marked 1..=10.  The square the player lands on is added to the player's score.
/// The game ends as a win for any player whose score reaches at least 1000.
fn deterministic_score(player1_start: i64, player2_start: i64) -> i64 {
    let mut player1_score = 0;
    let mut player1_space = player1_start;

    let mut player2_score = 0;
    let mut player2_space = player2_start;

    let mut die = DeterministicD100::default();
    let mut rolls = 0;

    loop {
        rolls += 3;
        if turn(&mut die, &mut player1_space, &mut player1_score) {
            return rolls * player2_score;
        }

        rolls += 3;
        if turn(&mut die, &mut player2_space, &mut player2_score) {
            return rolls * player1_score;
        }
    }
}

fn turn(die: &mut DeterministicD100, space: &mut i64, score: &mut i64) -> bool {
    *space = (((*space - 1) + die.next() + die.next() + die.next()) % 10) + 1;
    *score += *space;

    *score >= 1000
}

#[derive(Debug)]
struct DeterministicD100 {
    next: i64,
}

impl Default for DeterministicD100 {
    fn default() -> Self {
        DeterministicD100 { next: 1 }
    }
}

impl DeterministicD100 {
    fn next(&mut self) -> i64 {
        let value = self.next;
        self.next = self.next % 100 + 1;
        value
    }
}

#[test]
fn deterministic_sample() {
    assert_eq!(739785, deterministic_score(4, 8));
}