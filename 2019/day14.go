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
	c := map[string]uint64{
		"ORE":  math.MaxUint64,
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

	l := newLab(reactions)

	l.makeChemical(chemical{n: 1, name: "FUEL"})

	return int(math.MaxUint64 - l.chemicals["ORE"])
}

func (l *lab) makeChemical(c chemical) {
	reaction := l.reactions[c.name]

	if l.chemicals[reaction.output.name] > c.n {
		l.chemicals[reaction.output.name] -= c.n
		return
	}

	// create each chem required for reaction
	for _, chem := range reaction.input {
		// create as many as required
		n := l.chemicals[chem.name]
		for n < chem.n {
			l.makeChemical(chem)
			n = l.chemicals[chem.name]
		}

		// use newly created chem in reaction
		l.chemicals[chem.name] -= chem.n
	}

	// produce output of reaction, i.e. original input
	l.chemicals[c.name] += reaction.output.n
}

func day14Part2(in []string) int {
	return 0
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

	return reaction{input: in, output: chemical{n: nOut, name: outMatches[2]}}
}
