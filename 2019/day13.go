package adventofcode

const (
	tile_empty = iota
	tile_wall
	tile_block
	tile_paddle
	tile_ball
)

type arcade struct {
	blockCount int64
}

func (a *arcade) handleInstruction(p point, tile int64) {
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

func day13Part1(argv string) int64 {
	mem := initCodes(argv)

	a := arcade{}
	in, done := make(chan int64, 1), make(chan bool, 1)
	in <- 0 // send black as 1st input
	a.start(mem, in, done)

	<-done // wait for bot to be finished

	return a.blockCount
}

func day13Part2(in []string) int {
	return 0
}
