package adventofcode

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

var day12Input = []string{
	"<x=14, y=4, z=5>",
	"<x=12, y=10, z=8>",
	"<x=1, y=7, z=-10>",
	"<x=16, y=-5, z=3>",
}

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
	got := day12Part1(day12Input, 1000)
	want := 6423
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
				"<x=-1, y=0, z=2>",
				"<x=2, y=-10, z=-7>",
				"<x=4, y=-8, z=8>",
				"<x=3, y=5, z=-1>",
			},
			2772,
		},
		{
			[]string{
				"<x=-8, y=-10, z=0>",
				"<x=5, y=5, z=10>",
				"<x=2, y=-7, z=3>",
				"<x=9, y=-8, z=-3>",
			},
			4686774924,
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
	got := day12Part2(day12Input)
	want := 327636285682704
	if got != want {
		t.Errorf("got %v, want %v", got, want)
	}
	fmt.Printf("Day 12, part 2 answer: %v\n", got)
}

func TestDay12_applyGravity(t *testing.T) {
	type args struct {
	}
	tests := []struct {
		name  string
		m1    *moon
		m2    *moon
		want1 point3D
		want2 point3D
	}{
		{
			"start from 0",
			&moon{&point3D{3, 2, 8}, &point3D{0, 0, 0}},
			&moon{&point3D{5, 2, 6}, &point3D{0, 0, 0}},
			point3D{1, 0, -1},
			point3D{-1, 0, 1},
		},
		{
			"update velocity",
			&moon{&point3D{9, -2, 8}, &point3D{3, 0, -9}},
			&moon{&point3D{5, 5, 8}, &point3D{1, -4, 6}},
			point3D{2, 1, -9},
			point3D{2, -5, 6},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			applyGravity(tt.m1, tt.m2)

			assert.Equal(t, tt.want1, *tt.m1.velocity)
			assert.Equal(t, tt.want2, *tt.m2.velocity)
		})
	}
}

func TestDay12_applyVelocity(t *testing.T) {
	type fields struct {
		name     string
		pos      *point3D
		velocity *point3D
	}
	tests := []struct {
		m    *moon
		want *point3D
	}{
		{
			&moon{&point3D{1, 2, 3}, &point3D{-2, 0, 3}},
			&point3D{-1, 2, 6},
		},
	}

	for _, tt := range tests {
		t.Run("", func(t *testing.T) {
			tt.m.applyVelocity()

			assert.Equal(t, *tt.want, *tt.m.pos)
		})
	}
}
