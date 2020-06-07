package adventofcode

import "fmt"

const (
	tile_empty = iota
	tile_wall
	tile_block
	tile_paddle
	tile_ball
)

type arcade struct {
	screen     [][]int64
	b          *bounds
	blockCount int64
}

func newArcade(size int) *arcade {
	s := make([][]int64, size)
	for y := range s {
		s[y] = make([]int64, size)
	}

	return &arcade{s, &bounds{0, 0, 0, 0}, 0}
}

func (a *arcade) handleInstruction(p point, tile int64) {
	a.screen[p.y][p.x] = tile
	a.b.update(p)

	if tile == tile_block {
		a.blockCount++
	}
}

func (a *arcade) start(mem []int64, in chan int64, done chan bool) {
	out := make(chan int64)

	go intcode(mem, in, out)

	// listen for output
	for x := range out {
		// read other 2 vars
		y := <-out
		t := <-out

		a.handleInstruction(point{int(x), int(y)}, t)
	}

	done <- true
}

func (a *arcade) draw() {
	fmt.Println("screen bounds:", a.b)
	for y := a.b.minY; y < a.b.maxY; y++ {
		for x := a.b.minX; x < a.b.maxX; x++ {

			switch a.screen[y][x] {
			case tile_ball:
				fmt.Printf("*")
			case tile_block:
				fmt.Printf("#")
			case tile_paddle:
				fmt.Printf("_")
			case tile_wall:
				fmt.Printf("`")
			}

		}
		fmt.Println("")
	}
}

func day13Part1(argv string) int64 {
	mem := initCodes(argv)

	a := newArcade(100)
	in, done := make(chan int64, 1), make(chan bool, 1)
	a.start(mem, in, done)
	<-done

	return a.blockCount
}

func day13Part2(in []string) int {
	return 0
}
