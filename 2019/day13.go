package adventofcode

import (
	"fmt"
)

const (
	tile_empty = iota
	tile_wall
	tile_block
	tile_paddle
	tile_ball
)

const (
	joystick_left = iota - 1
	joystick_neutral
	joystick_right
)

type arcade struct {
	screen [][]int64
	b      *bounds

	score      int64
	blockCount int64
	ball       *point
	paddle     *point
}

func newArcade(size int) *arcade {
	s := make([][]int64, size)
	for y := range s {
		s[y] = make([]int64, size)
	}

	return &arcade{
		screen:     s,
		b:          &bounds{0, 0, 0, 0},
		blockCount: 0,
		ball:       &point{0, 0},
		paddle:     &point{0, 0},
	}
}

func (a *arcade) handleInstruction(p point, tile int64) {
	// score update
	if p.x == -1 && p.y == 0 {
		a.score = tile
		return
	}

	a.screen[p.y][p.x] = tile
	a.b.update(p)

	switch tile {
	case tile_block:
		a.blockCount++
	case tile_ball:
		// a.draw()
		a.ball = &p
	case tile_paddle:
		a.paddle = &p
	}
}

func (a *arcade) joystickPosition() int64 {
	if a.ball.x > a.paddle.x {
		return joystick_right
	} else if a.ball.x < a.paddle.x {
		return joystick_left
	}
	return joystick_neutral
}

func (a *arcade) start(mem []int64) {
	in, done := make(chan int64), make(chan bool, 1)
	out := make(chan int64)

	go intcode(mem, in, out, done)
	for {
		select {
		case x := <-out:
			y := <-out
			t := <-out
			a.handleInstruction(point{int(x), int(y)}, t)

		case in <- a.joystickPosition():

		case <-done:
			return
		}
	}
}

func (a *arcade) draw() {
	fmt.Println("score:", a.score)
	for y := a.b.minY; y < a.b.maxY; y++ {
		for x := a.b.minX; x <= a.b.maxX; x++ {

			switch a.screen[y][x] {
			case tile_ball:
				fmt.Printf("*")
			case tile_block:
				fmt.Printf("#")
			case tile_paddle:
				fmt.Printf("-")
			case tile_wall:
				fmt.Printf("`")
			default:
				fmt.Printf(".")
			}

		}
		fmt.Println("")
	}
	fmt.Println("")
	fmt.Println("")
}

func day13Part1(argv string) int64 {
	mem := initCodes(argv)

	a := newArcade(100)
	a.start(mem)

	return a.blockCount
}

func day13Part2(argv string) int64 {
	mem := initCodes(argv)
	mem[0] = 2 // insert 2 coins

	a := newArcade(100)
	a.start(mem)

	return a.score
}
