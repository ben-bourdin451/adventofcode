package adventofcode

import (
	"fmt"
	"testing"
)

func TestDay13Part1_blockCount(t *testing.T) {
	type instructions struct {
		p    point
		tile int64
	}
	cases := []struct {
		in   []instructions
		want int64
	}{
		{
			[]instructions{
				{point{1, 2}, 0},
			},
			0,
		},
		{
			[]instructions{
				{point{1, 2}, 0},
				{point{1, 2}, 1},
				{point{1, 2}, 2},
			},
			1,
		},
		{
			[]instructions{
				{point{1, 2}, 2},
				{point{1, 2}, 1},
				{point{1, 2}, 2},
			},
			2,
		},
	}

	for _, c := range cases {
		a := newArcade(10)
		for _, i := range c.in {
			a.handleInstruction(i.p, i.tile)
		}
		got := a.blockCount
		if got != c.want {
			t.Errorf("got %v, want %v", got, c.want)
		}
	}
}

func TestDay13Part1Final(t *testing.T) {
	in, err := readStrings("./day13_input.txt")
	if err != nil {
		t.Error("Error while reading input", err)
	}

	got := day13Part1(in[0])
	want := int64(205)
	fmt.Printf("Day 13, part 1 answer: %v\n", got)
	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestDay13Part2(t *testing.T) {
	cases := []struct {
		in   []string
		want int
	}{
		{
			[]string{
				"",
			},
			0,
		},
	}

	for _, c := range cases {
		got := day13Part2(c.in)
		if got != c.want {
			t.Errorf("got %v, want %v", got, c.want)
		}
	}
}

func TestDay13Part2Final(t *testing.T) {
	in, err := readStrings("./day13_input.txt")
	if err != nil {
		t.Error("Error while reading input", err)
	}

	got := day13Part2(in)
	want := 0
	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
	fmt.Printf("Day 13, part 2 answer: %v\n", got)
}
