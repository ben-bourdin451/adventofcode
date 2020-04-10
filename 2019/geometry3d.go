package adventofcode

import (
	"fmt"
	"math"
	"regexp"
	"strconv"
)

type point3D struct {
	x, y, z int
}

func (p *point3D) String() string {
	return fmt.Sprintf("<x=%3d, y=%3d, z=%3d>", p.x, p.y, p.z)
}

func newPoint3DFromString(s string) *point3D {
	r := regexp.MustCompile(`^<x=(?P<x>.+), y=(?P<y>.+), z=(?P<z>.+)>`)
	match := r.FindStringSubmatch(s)

	x, y, z := 0, 0, 0
	for i, name := range r.SubexpNames() {
		n, _ := strconv.Atoi(match[i])
		switch name {
		case "x":
			x = n
		case "y":
			y = n
		case "z":
			z = n
		}
	}
	return &point3D{x, y, z}
}

func (p *point3D) add(other *point3D) *point3D {
	return &point3D{p.x + other.x, p.y + other.y, p.z + other.z}
}

func (p *point3D) sum() int {
	return int(math.Abs(float64(p.x)) + math.Abs(float64(p.y)) + math.Abs(float64(p.z)))
}
