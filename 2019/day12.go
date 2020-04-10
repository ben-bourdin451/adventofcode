package adventofcode

import "fmt"

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
	moons := []*moon{}
	for _, s := range in {
		moons = append(moons, &moon{newPoint3DFromString(s), &point3D{0, 0, 0}})
	}

	for step := 0; step < steps; step++ {
		// fmt.Println("After", step, "steps:")
		// for _, m := range moons {
		// 	fmt.Println(m.String())
		// }
		// fmt.Println()

		// gravity
		for i := 0; i < len(moons); i++ {
			for j := i + 1; j < len(moons); j++ {
				applyGravity(moons[i], moons[j])
			}
		}

		// velocity
		for _, m := range moons {
			m.applyVelocity()
		}
	}

	total := 0
	for _, m := range moons {
		total += m.potentialEnergy() * m.kineticEnergy()
	}

	return total
}

func day12Part2(in []string) int {
	return 0
}
