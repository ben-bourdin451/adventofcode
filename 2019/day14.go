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

func newLab(r map[string]reaction, ore uint64) *lab {
	c := map[string]uint64{
		"ORE":  ore,
		"FUEL": 0,
	}
	return &lab{
		reactions: r,
		chemicals: c,
	}
}

func day14Part1(in []string) int {
	reactions := make(map[string]reaction, len(in))
	for _, r := range in {
		reaction := parseReaction(r)
		reactions[reaction.output.name] = reaction
	}

	l := newLab(reactions, math.MaxUint64)
	fuel := chemical{n: 1, name: "FUEL"}

	return int(l.makeChemicalRec(fuel))
}

func (l *lab) multiplyChems(m uint64) {
	for k := range l.chemicals {
		if k != "ORE" {
			l.chemicals[k] *= m
		}
	}
}

func (l *lab) makeChemicalRec(c chemical) uint64 {
	ore := l.chemicals["ORE"]

	reaction := l.reactions[c.name]
	for _, chem := range reaction.input {
		// create as many as required based on leftovers
		for n := l.chemicals[chem.name]; n < chem.n; n = l.chemicals[chem.name] {
			l.makeChemicalRec(chemical{chem.n, chem.name})
		}

		// use chem in reaction
		l.chemicals[chem.name] -= chem.n
	}

	// produce output of reaction, i.e. original input
	l.chemicals[c.name] += reaction.output.n

	return ore - l.chemicals["ORE"]
}

func (l *lab) makeChemicalProc(chem chemical) uint64 {
	q := []chemical{chem}

	oreUsed := uint64(0)
	leftovers := make(map[string]uint64)

	for len(q) > 0 {
		// pop chem to create
		c := q[0]
		q = q[1:]

		if c.name == "ORE" {
			oreUsed += c.n

		} else if left, ok := leftovers[c.name]; ok && left >= c.n {
			leftovers[c.name] -= c.n

		} else {
			need := c.n - left
			r := l.reactions[c.name]
			m := uint64(math.Ceil(float64(need) / float64(r.output.n)))
			for _, in := range r.input {
				q = append(q, chemical{name: in.name, n: m * in.n})
			}

			leftovers[c.name] += m*r.output.n - need
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

	l := newLab(reactions, math.MaxUint64)
	fuel := chemical{name: "FUEL", n: 1}

	// make 1 fuel
	oreUsed := l.makeChemicalRec(fuel)

	// copy leftovers
	leftovers := make(map[string]uint64)
	for k, v := range l.chemicals {
		leftovers[k] = v
	}

	// calculate ore & leftovers to almost make 1e12
	q := uint64(math.Floor(float64(1e12 / oreUsed)))
	l.multiplyChems(q)

	total := oreUsed * q
	for total < 1e12 {
		used := l.makeChemicalRec(fuel)
		total += used
	}

	return int(l.chemicals["FUEL"] - 1)
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
