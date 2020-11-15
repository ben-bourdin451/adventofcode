package adventofcode

import (
	"math"
	"regexp"
	"strconv"
	"strings"
)

type reaction struct {
	input  []chemical
	output chemical
}

type chemical struct {
	n    uint64
	name string
}

type lab struct {
	reactions map[string]reaction
	chemicals map[string]uint64
}

func newLab(r map[string]reaction) *lab {
	return &lab{
		reactions: r,
		chemicals: make(map[string]uint64),
	}
}

func day14Part1(in []string) int {
	reactions := make(map[string]reaction, len(in))
	for _, r := range in {
		reaction := parseReaction(r)
		reactions[reaction.output.name] = reaction
	}

	l := newLab(reactions)

	return int(l.makeChemical(chemical{name: "FUEL", n: 1}))
}

// Recursive function doesn't work with batches as we need breadth first resolution not depth first
// Order in which the reactions are done matters due to how leftovers are generated & used
// Recursive function without batches does work however is very innefficient
func (l *lab) makeChemicalRec(r reaction, batches uint64) uint64 {
	ore := uint64(0)

	for _, chem := range r.input {
		need := chem.n * batches
		if left, ok := l.chemicals[chem.name]; ok && left >= need {
			if chem.name == "ORE" {
				ore += need
			}
			l.chemicals[chem.name] -= need
			continue
		}

		want := l.reactions[chem.name]
		amount := need - l.chemicals[chem.name]
		b := uint64(math.Ceil(float64(amount) / float64(want.output.n)))

		if chem.name != "ORE" {
			ore += l.makeChemicalRec(want, b)
		}

		// use what we need
		l.chemicals[chem.name] -= amount
	}

	// produce output of reaction
	l.chemicals[r.output.name] += r.output.n * batches

	return ore
}

func (l *lab) makeChemical(chem chemical) uint64 {
	oreUsed := uint64(0)
	leftovers := make(map[string]uint64)

	q := []chemical{chem}
	for len(q) > 0 {
		// pop chem to create
		c := q[0]
		q = q[1:]

		if c.name == "ORE" {
			oreUsed += c.n

		} else if left, ok := leftovers[c.name]; ok && c.n <= left {
			leftovers[c.name] -= c.n

		} else {
			need := c.n - left
			r := l.reactions[c.name]
			m := uint64(math.Ceil(float64(need) / float64(r.output.n)))
			for _, in := range r.input {
				q = append(q, chemical{name: in.name, n: m * in.n})
			}

			leftovers[c.name] = m*r.output.n - need
		}
	}

	return oreUsed
}

func day14Part2(in []string) int {
	reactions := make(map[string]reaction, len(in))
	for _, r := range in {
		reaction := parseReaction(r)
		reactions[reaction.output.name] = reaction
	}

	l := newLab(reactions)

	// make 1 fuel to have initial estimate
	oreUsed := l.makeChemical(chemical{name: "FUEL", n: 1})

	// binary search
	low := math.Floor(float64(1e12 / oreUsed))
	upper := low * low
	for upper-low > 1 {
		guess := math.Floor((upper + low) / 2)
		if used := l.makeChemical(chemical{name: "FUEL", n: uint64(guess)}); used > 1e12 {
			upper = guess
		} else if used < 1e12 {
			low = guess
		} else {
			return int(guess)
		}
	}

	return int(low)
}

func parseReaction(r string) reaction {
	rsplit := strings.Split(r, "=>")

	in := []chemical{}
	re := regexp.MustCompile(`([0-9]+) ([A-Z]+)`)
	for _, c := range strings.Split(rsplit[0], ",") {
		matches := re.FindStringSubmatch(c)
		n, _ := strconv.ParseUint(matches[1], 10, 64)
		in = append(in, chemical{n: n, name: matches[2]})
	}

	outMatches := re.FindStringSubmatch(rsplit[1])
	nOut, _ := strconv.ParseUint(outMatches[1], 10, 64)

	return reaction{
		input:  in,
		output: chemical{n: nOut, name: outMatches[2]},
	}
}
