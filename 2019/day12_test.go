package adventofcode

import (
	"fmt"
	"testing"
)

func TestDay12Part1(t *testing.T) {
	cases := []struct {
		in    []string
		steps int
		want  int
	}{
		{
			[]string{
				"<x=-1, y=0, z=2>",
				"<x=2, y=-10, z=-7>",
				"<x=4, y=-8, z=8>",
				"<x=3, y=5, z=-1>",
			},
			10,
			179,
		},
		{
			[]string{
				"<x=-8, y=-10, z=0>",
				"<x=5, y=5, z=10>",
				"<x=2, y=-7, z=3>",
				"<x=9, y=-8, z=-3>",
			},
			100,
			1940,
		},
	}

	for _, c := range cases {
		got := day12Part1(c.in, c.steps)
		if got != c.want {
			t.Errorf("got %v, want %v", got, c.want)
		}
	}
}

func TestDay12Part1Final(t *testing.T) {
	in := []string{
		"<x=14, y=4, z=5>",
		"<x=12, y=10, z=8>",
		"<x=1, y=7, z=-10>",
		"<x=16, y=-5, z=3>",
	}

	got := day12Part1(in, 1000)
	want := 0
	fmt.Printf("Day 12, part 1 answer: %v\n", got)
	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
}

func TestDay12Part2(t *testing.T) {
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
		got := day12Part2(c.in)
		if got != c.want {
			t.Errorf("got %v, want %v", got, c.want)
		}
	}
}

func TestDay12Part2Final(t *testing.T) {
	in, err := readStrings("./day12_input.txt")
	if err != nil {
		t.Error("Error while reading input", err)
	}

	got := day12Part2(in)
	want := 0
	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
	fmt.Printf("Day 12, part 2 answer: %v\n", got)
}
