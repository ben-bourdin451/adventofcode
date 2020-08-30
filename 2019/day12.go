package adventofcode

import (
	"fmt"
)

type universe struct {
	moons []*moon
}

func newUniverse(in []string) *universe {
	u := &universe{[]*moon{}}
	for _, s := range in {
		u.moons = append(u.moons, &moon{newPoint3DFromString(s), &point3D{0, 0, 0}})
	}
	return u
}

func (u *universe) tick() {
	// gravity
	for i := 0; i < len(u.moons); i++ {
		for j := i + 1; j < len(u.moons); j++ {
			applyGravity(u.moons[i], u.moons[j])
		}
	}

	// velocity
	for _, m := range u.moons {
		m.applyVelocity()
	}
}

func (u *universe) print(s int) {
	fmt.Println("After", s, "steps:")
	for _, m := range u.moons {
		fmt.Printf("pos=%v, vel=%v\n", m.pos, m.velocity)
	}
	fmt.Println()
}

func (u *universe) isApex() (bool, bool, bool) {
	x, y, z := true, true, true
	for _, m := range u.moons {
		if m.velocity.x != 0 {
			x = false
		}
		if m.velocity.y != 0 {
			y = false
		}
		if m.velocity.z != 0 {
			z = false
		}
	}

	return x, y, z
}

type moon struct {
	pos      *point3D
	velocity *point3D
}

func (m *moon) potentialEnergy() int {
	return m.pos.sum()
}

func (m *moon) kineticEnergy() int {
	return m.velocity.sum()
}

func (m *moon) String() string {
	return fmt.Sprintf("pos=%v, vel=%v", m.pos, m.velocity)
}

func (m *moon) applyVelocity() {
	m.pos = m.pos.add(m.velocity)
}

func applyGravity(m1, m2 *moon) {
	if m1.pos.x > m2.pos.x {
		m1.velocity.x--
		m2.velocity.x++
	} else if m1.pos.x < m2.pos.x {
		m1.velocity.x++
		m2.velocity.x--
	}

	if m1.pos.y > m2.pos.y {
		m1.velocity.y--
		m2.velocity.y++
	} else if m1.pos.y < m2.pos.y {
		m1.velocity.y++
		m2.velocity.y--
	}

	if m1.pos.z > m2.pos.z {
		m1.velocity.z--
		m2.velocity.z++
	} else if m1.pos.z < m2.pos.z {
		m1.velocity.z++
		m2.velocity.z--
	}
}

func day12Part1(in []string, steps int) int {
	u := newUniverse(in)

	for step := 0; step < steps; step++ {
		u.tick()
	}

	total := 0
	for _, m := range u.moons {
		total += m.potentialEnergy() * m.kineticEnergy()
	}

	return total
}

func day12Part2(in []string) int {
	u := newUniverse(in)

	steps := 0
	x, y, z := 0, 0, 0
	for x == 0 || y == 0 || z == 0 {
		u.tick()
		steps++

		xapex, yapex, zapex := u.isApex()
		if x == 0 && xapex {
			x = steps * 2
		}

		if y == 0 && yapex {
			y = steps * 2
		}

		if z == 0 && zapex {
			z = steps * 2
		}
	}

	return lcmm([]int{x, y, z})
}
