package day21

// Part1 plays a game of dirac dice with a deterministic d100 (rolls 1, then 2, ...), and returns
// the score of the losing player by the number of times the die was rolled during the game.
// Dirac dice is played by rolling a die 3 times, then moving a pawn that many times
// around a board marked [1,10].  The square the player lands on is added to the player's score.
// The game ends as a win for any player whose score reaches at least 1000.
func Part1(player1Start, player2Start int) int {
	players := []player{
		{space: player1Start},
		{space: player2Start},
	}

	turn := 0
	die := d100{next: 1}

	for players[0].score < 1000 && players[1].score < 1000 {
		players[turn].move(die.roll(3))
		turn = (turn + 1) % len(players)
	}

	return min(players[0].score, players[1].score) * die.rolls
}

type player struct {
	space int
	score int
}

// move moves this player's pawn around a board marked [1,10] and adds the square the player lands on to it's score.
func (p *player) move(roll int) {
	p.space = (p.space-1+roll)%10 + 1
	p.score += p.space
}

// d100 is a deterministic d100 that rolls 1, then 2, ..., then 100, then 1, etc.
type d100 struct {
	next  int
	rolls int
}

// roll rolls the die the given number of times, returning the sum of the rolls.
func (die *d100) roll(times int) int {
	die.rolls += times

	value := 0

	for i := 0; i < times; i++ {
		value += die.next
		die.next = die.next%100 + 1
	}

	return value
}
