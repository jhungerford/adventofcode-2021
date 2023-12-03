package day21

import (
	"fmt"
	"testing"
)

func TestPart1(t *testing.T) {
	t.Parallel()

	want := 739785

	if got := Part1(4, 8); got != want {
		t.Errorf("Part1() = %v, want %v", got, want)
	}
}

func Test_d100_roll(t *testing.T) {
	t.Parallel()

	tests := []struct {
		next     int
		wantRoll int
		wantNext int
	}{
		{1, 6, 4},
		{4, 15, 7},
		{99, 200, 2},
	}
	for _, tt := range tests {
		t.Run(fmt.Sprintf("roll(%v)", tt.next), func(t *testing.T) {
			times := 3
			die := d100{
				next:  tt.next,
				rolls: 0,
			}
			if got := die.roll(times); got != tt.wantRoll || die.next != tt.wantNext || die.rolls != times {
				t.Errorf("roll(%v) = %v, want %v.  Next = %v, want %v.  Rolls = %v, want %v",
					tt.next, tt.wantRoll, got, die.next, tt.wantNext, die.rolls, times)
			}
		})
	}
}
