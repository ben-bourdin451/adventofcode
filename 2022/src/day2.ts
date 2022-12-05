export function answers() {
	console.log("Day 2 - part 1: ", part1(input))
	console.log("Day 2 - part 2: ", part2(input))
}

export enum ShapeOpponent {
	'A' = 1,
	'B',
	'C',
}

export enum ShapeMe {
	'X' = 1,
	'Y',
	'Z',
}

enum Shapes {
	ROCK = 1,
	PAPER,
	SCISSORS,
}


export function part1(input: string): number {
	const rounds = input.split('\n');
	let score = 0;
	rounds.forEach(r => {
		const res = r.split(' ');
		const op = ShapeOpponent[res[0] as keyof typeof ShapeOpponent];
		const me = ShapeMe[res[1] as keyof typeof ShapeMe];
		score += getScore(op, me);
	});

	return score;
}

export function getScore(op: number, me: number): number {
	if (op == me) { // draw
		return me + 3;
	} else if (
		(op == Shapes.ROCK && me == Shapes.PAPER) ||
		(op == Shapes.PAPER && me == Shapes.SCISSORS) ||
		(op == Shapes.SCISSORS && me == Shapes.ROCK)) { // win
		return me + 6;
	}
	return me;
}

export function part2(input: string): number {
	const rounds = input.split('\n');
	let score = 0;
	rounds.forEach(r => {
		const res = r.split(' ');
		const op = ShapeOpponent[res[0] as keyof typeof ShapeOpponent];
		const me = ShapeMe[res[1] as keyof typeof ShapeMe];
		score += getPlay(op, me);
	});

	return score;
}

export function getPlay(op: number, me: number): number {
	if (me == 2) { // draw
		return op + 3;
	} else if (me == 3) { // win
		switch (op) {
			case Shapes.ROCK:
				return Shapes.PAPER + 6;
			case Shapes.PAPER:
				return Shapes.SCISSORS + 6;
			case Shapes.SCISSORS:
				return Shapes.ROCK + 6;
		}
	} else { // loss
		switch (op) {
			case Shapes.ROCK:
				return Shapes.SCISSORS;
			case Shapes.PAPER:
				return Shapes.ROCK;
			case Shapes.SCISSORS:
				return Shapes.PAPER;
		}
	}
	return 0;
}

const input = `C Y
C Y
B Y
A Z
B Z
A X
A Y
A Y
A X
A Y
B Y
A Y
B Y
B Y
B Z
B Z
B Z
B Z
A Y
B Z
A Y
B X
B Y
B X
A X
A X
B Z
A X
A X
B Z
B Z
B Y
B Z
B Z
B Z
B Y
A X
A X
B Z
A X
B X
B X
C Y
B Z
C X
A X
A Y
B Y
A Y
B X
A X
B Y
B Z
B Z
B Y
B Z
C Z
B X
B X
B Z
B Z
B Z
B Z
A Y
B X
A X
C X
B Y
B Z
A Y
B Z
B Z
B Y
B Y
B Z
B Y
B Z
A X
B X
B X
A X
A X
B Z
B Z
B Z
B Z
B Y
B X
B Z
A X
A Y
B Z
A Y
B Z
B Y
B Z
A X
B Y
A Y
B Z
A Z
A Y
A Y
C Y
B Z
B X
A Z
B Z
A X
B Z
A Z
B Z
A X
B Y
A X
B Y
B Z
B X
B X
B Z
B Z
B Z
A X
B Z
A X
B X
B Z
A X
C Z
B Z
B Z
B Y
B Y
B Y
B Z
B Z
A Y
B Z
B Z
C Y
C Z
A X
B Z
B X
B Z
B Z
B Y
A X
B Z
B Y
A Z
B Z
A X
A X
B Y
A Y
B Z
B Z
B X
B Y
A X
A Y
B X
C Z
A Y
B Z
B Z
B Z
A Z
C Y
B Y
B Y
B Z
C Y
B Y
B X
B X
B Z
C Z
A X
B Z
B Z
B Z
B Z
B X
B X
A X
A Z
A Z
A X
C Y
B X
A X
A Y
A X
B X
A Y
B Z
B Z
A Y
A X
B Y
B Z
B Z
A X
A Z
B Z
B X
A X
B Z
B Y
A Y
A Z
B X
A Y
B Z
A Z
B Z
A Y
B Z
B X
B Y
A Y
B Z
B Z
A X
A X
B Y
B Z
A X
B Z
B Z
B Z
B Z
B Z
A Z
B Z
B Z
B X
A Y
C X
B Z
B Y
B Y
B Z
B Z
B Z
B Y
B X
B Y
C X
B Z
A Z
A Y
C X
A X
B X
A X
B X
A Y
B Z
A Y
A Y
B Z
B X
B Z
A Y
B Z
B X
C Z
C X
C Z
B Y
B X
B Z
B Z
B Y
B Z
B Y
B Z
A X
B X
B Z
A X
B Z
B Y
B Z
A X
B Y
C Y
A Z
B Z
C Z
A Y
B Z
A Y
A Z
B Y
A X
A X
B Z
B Z
B Z
A Y
B Z
A Z
B Z
A Y
A Z
B Y
C Z
B Z
A Z
B X
B Z
B Y
B Z
A Z
A Z
B Z
B X
C Z
B X
B Z
B Y
A X
B Z
A X
B X
B Z
B Z
A X
B X
C X
C X
B X
B Y
B Z
B X
B Y
B Y
B Z
A Z
B Z
C X
A Y
C X
B Z
A Y
B Z
B Y
B Z
B X
A X
B X
B Z
A Y
A Y
B Z
B Z
B Z
A Y
B Z
B X
C X
B Z
B Z
C Y
B Z
C Y
B Z
C Y
C X
B Z
C Y
A Y
A Y
C Z
B Z
B X
B Z
B X
C Z
B Z
A Z
B Z
C Y
B Z
A X
A Y
B Y
B Y
B Z
A Y
B X
B Z
B Z
A Z
B X
A Y
A Y
B Z
B Z
B Z
C X
A Z
B X
C Z
B Y
B Z
B X
A Y
B Z
A X
B X
B Y
A Y
B Z
B Z
B Y
A Z
B Z
A X
B Y
A Y
A X
A Y
A X
A X
B X
B Y
B X
B Y
B Z
B Y
B X
A X
B Z
A Z
A X
B Z
B Y
A X
B Y
A X
B X
C Y
B X
B X
B X
C Y
B X
B Y
B Y
B Y
B Y
B X
A Y
C Y
B Z
B Z
B X
B Z
C Y
B Y
B Z
B Z
B Y
B Z
A X
B X
B Y
A X
C Y
B Z
A Y
B Z
B Z
B Z
A X
B Y
B Z
B Y
B Z
A X
B Z
A X
B Y
B Y
B X
B X
A Y
B Z
C Y
A X
A X
B X
B Z
B X
B X
B Z
B Z
A X
B X
B Z
A Z
B Y
A X
B Z
C Y
B Z
B X
B Z
B Z
A Y
A Y
B Z
B X
B Y
B Z
A Y
A Y
B X
B X
C X
B Z
C Z
B X
A Y
B Z
A Z
B Y
A Y
B Z
B X
B X
A Y
B Z
B Y
A X
B Y
B Y
B Y
B X
B Y
B Z
B Y
B Y
A X
C Z
B Z
B Z
A Z
B Z
B Z
A X
B X
A Y
A X
B X
C X
B X
B Z
B Y
A Z
A Y
B Y
B Z
B X
B X
B Z
B Z
B Y
A X
B Z
A Y
A X
B Y
B Z
B Z
B X
A X
A Y
C X
A Y
B Z
B Y
B Z
B Z
C Z
B Z
B X
A X
C Y
B X
B Z
A Z
A X
A Y
B Z
B Z
A Y
A Y
B X
A Y
A Y
B Z
A Y
B Y
B Z
A Y
A Z
B X
B X
B Y
B Z
B Z
A X
B X
C X
B Y
A Y
A X
A X
A X
B Y
A X
A Z
A Y
B X
A Y
B Z
C X
B X
B X
B Z
B X
B X
B Z
A X
B Z
B X
B Z
A Y
B Y
B Z
B Y
B Z
B Z
A X
B Z
A Y
C Z
A Y
B Z
A Y
B X
B Z
B X
C Y
A X
B Z
B Y
A X
A X
B X
B Z
A Z
B Z
B X
B X
B X
B Z
B Z
B Y
B X
B X
A X
B Z
B X
A X
A X
A X
B X
B Z
A Z
B X
B Y
B Y
B Z
B Z
C Z
A Y
A X
B Y
B X
B Z
B X
A X
B Z
B Z
B Y
B Z
B X
B Z
B Z
B X
B X
A X
A X
B X
B Z
B Z
C X
C X
B Z
B Z
B X
B Z
B Y
B X
A X
B Y
A Y
A X
B Z
B Z
C X
B Y
B Z
B Z
B X
B Z
B Z
C Z
B Z
B Y
B Z
B X
A X
B X
B Z
B Z
A Y
B Z
B Z
B Z
B Z
A X
B Z
B Z
B X
B Y
B Y
B Z
A X
B Z
B Y
B X
A X
B X
B Z
B Z
B Z
B X
B Z
B Z
B Z
A Y
B Z
C X
B Y
B Y
B Y
A Y
B Z
B Z
A X
C Z
B Z
B Z
B Z
B X
B Z
A Z
B Z
B Z
B Z
A Z
B Z
C Y
B X
A X
A Y
B Z
B Z
A Z
B Z
B X
A Z
B Y
B Z
B X
B Z
B X
B Y
B Z
A X
B Z
B Z
A Y
B X
B X
B X
A Z
C X
A Z
B Z
B Z
B Z
B X
A Y
C X
A Z
A Y
B X
B Z
B X
B Z
B Y
A Y
B X
C X
A Y
C Z
A X
B Z
B Z
A Z
B X
B Y
B Z
A Y
B Y
A X
A X
C Y
A Y
B X
A X
B Y
B X
B Y
A Y
A X
C Y
B Y
B Y
B Z
B Y
B X
B Z
B Z
B X
A X
B Z
B Z
B Z
B X
B Z
B X
B Z
B Z
B Y
B Y
B X
A X
B Z
B Y
A Y
B Z
B X
B Z
B Z
A Z
B Z
A Y
B Z
A X
B Z
B Z
A Y
B Z
A X
B Z
A Y
A Y
A Z
B X
B Z
B Y
A Z
C Z
B Z
A X
A X
B X
A Z
B X
B X
B Z
C X
B Z
B Z
B Z
B Z
B X
A Z
A Y
B Z
B Y
C Z
B Y
B Z
A Y
B X
B X
B Z
A X
A Y
B Z
B X
B Y
A Y
C Y
C Y
B Z
A Y
B Y
A Y
B Z
B Z
A X
B X
A X
B X
A Y
A X
B Z
A X
B Z
B X
B Z
B X
B X
A X
A Y
B Z
B X
B X
A X
A Y
A X
B Y
B Z
B Z
B Z
B Z
B Z
B X
B Y
A Y
B X
B Z
A Y
B X
A X
B Z
C X
B Y
A Y
A X
A X
B X
B X
B Z
B Z
B Z
B X
B Y
B X
B Z
B X
B Y
B X
B Z
B X
B Y
B X
B Z
B Z
B Z
A X
C X
C Y
A Y
B X
B X
A Y
B Z
B X
B Z
B X
B X
C X
B Z
B Z
B Y
A Z
A Z
C X
B X
C Y
B Z
C X
B Z
A Y
C Z
B X
B Y
A Y
B Y
B X
B Z
A Y
A Z
C X
B Z
A X
B X
B Z
C Z
A Y
B Z
B Z
A X
A X
B Z
B Y
C Z
B Z
B X
B Z
A X
A X
B Y
A X
B X
A Y
B Z
A Y
B Z
A Y
B Z
A X
B Y
B Z
B Z
B Z
A X
A X
B X
B Z
A X
B Z
A X
A X
B Z
B Z
B Z
B X
B Y
B Z
B Z
B X
B Z
B Y
B Z
C Y
B X
C Z
B Z
B Z
A Y
B Z
B X
A Y
B Z
B X
B Z
B Z
B Y
B Z
A Z
A Y
B Z
B Z
B Y
A Y
B Z
A X
B X
A Z
A X
B X
B Z
B X
B Z
B Z
B Y
B Y
B Z
B Y
B Z
B Z
A X
B X
B Y
B Y
C Z
A X
B Y
A Y
B Z
B Z
B Z
C X
B X
A Z
B Y
A X
C X
B Z
B Z
B Z
B Y
A Y
A Y
B Y
B Z
B Z
B Z
C X
A X
B Z
A Y
B X
B Z
B Z
B Z
B Z
B X
B Y
B Z
B Z
A Y
C X
A X
B Z
B Z
A Y
A Y
B Z
A Y
B Y
A X
B Z
B Z
A Y
B Z
B Z
A X
A X
B X
B Z
B Z
A X
B Z
B Z
C Y
B Z
A X
B Z
A X
B Z
A X
B X
A X
A Y
B Z
B X
A X
B Z
A X
A Z
B Z
B Z
B Z
A Y
B X
A X
B Y
A Y
B Z
B Z
B X
B Z
B X
B X
A X
B Z
A Y
A X
B X
A Z
B X
B Y
B Z
B X
B X
B Z
C X
A X
B Z
B Y
C Z
B Z
A Z
A Z
A X
A Y
B Z
B Z
B X
A Z
B Z
B Z
B X
B Y
B X
B Z
B X
B Z
A Y
A X
B Z
B X
B Y
B Z
B Z
B Z
B Z
C X
C X
B Z
B X
B Z
B Z
B Z
B X
B Z
B Z
B Z
B X
B Z
B Z
B Z
C Y
B Z
B Y
B Z
A Z
A Y
B Z
A Y
C X
A X
B X
A Y
B Z
A Y
B Z
B Y
B Z
B Z
C Y
B Y
B Z
B Z
B X
B Y
B Z
B Y
B Z
B X
B Y
B Z
B Z
B Z
B Y
B Y
B Z
B Z
C X
B Z
A Y
B Z
B Z
B Y
B Y
B Z
B Z
A Y
B Z
A Z
C X
A Y
A Y
A X
B Z
A X
C Y
A Z
C Y
C X
B Z
A X
A Y
B Z
B Z
B Z
B Z
B Y
A Z
A Y
B Y
A Y
A Z
B X
B Z
B Z
A X
C Y
B Z
B X
C X
A Z
B Z
B X
B Y
A X
A X
B Z
C X
B Z
B Z
B X
B Z
B Z
B Z
A X
B Z
B Y
B Z
B Z
B Y
B X
A X
B Z
A X
A X
B Y
B Z
B X
A Y
C X
B Y
A X
A X
A X
C Y
B X
C Z
A X
B Z
A Z
B Z
A X
A Y
B Z
A X
B X
A Y
A X
A Y
B X
B Y
B Z
B Y
C X
C Y
B Z
B Y
B Z
A X
C Z
B Z
A X
B X
A X
B X
B Z
B X
B Z
B Z
B Z
A X
A X
B Z
B X
B Z
B Z
B Z
A X
B Z
B X
A X
C Z
A Y
B Z
A Y
B Z
B Z
B Z
C Z
B Z
B Z
A X
B X
A X
B X
A X
B Z
B Z
B X
B Z
A X
A Y
A Y
A Y
B Z
B Z
B Z
B Z
A X
B Z
B Z
B Y
B Z
A Z
B Z
A Y
B Y
B Y
C Y
B X
B Z
B Y
B Z
B Y
B Z
B Z
A Y
B X
A Z
B Z
A X
B Y
A X
B Z
B Y
A X
B X
B Z
B Z
B Z
A Y
A Y
A X
A Y
A Y
B X
B Z
B Y
B Y
B X
B Y
B X
B X
B X
B Z
B Z
B Z
A Y
A Z
B Y
C X
B X
B Z
C Y
B Z
C Y
B Z
B Z
B Z
B X
A X
B Z
B Z
A X
B Z
B Z
B Z
B Z
B Z
B Z
B X
A X
A Y
B Y
B Z
B Y
B X
A Z
A X
B X
B Y
B Z
B Z
B Z
B X
C X
B X
A Z
A X
A Y
B Z
B Z
A X
B Z
A Y
B X
B Z
B Y
B Y
A X
B X
B Z
B Z
B Z
B Z
B Y
A Z
A Y
B X
A X
B Y
B Z
C Y
B Z
B Z
B X
A Z
A Y
B Y
B Z
A Y
B Y
A Y
A X
A Y
B Y
C X
C Z
B X
A Z
A X
B Y
B X
A X
B Z
A Y
A Z
B Z
B Z
B Z
A X
B Z
B Y
B Z
A X
A Y
B Z
B X
C X
B Z
A Y
B Z
B Z
B X
B Z
B Z
A Y
A X
B Z
C Y
A Y
B Y
B Z
A X
B Z
B Z
B Z
B Z
C X
B Y
B Z
B X
B Z
B Z
B Z
B Z
B Z
B Y
A X
B Y
A Y
A X
A X
B Z
B Z
B Z
C X
B X
B Z
A X
A X
B Y
A X
B Z
B Z
B X
B Y
B Y
B Y
A Y
A Y
A X
A X
B Z
B Y
B Y
B X
B Y
B Z
A Z
B Z
A Z
B X
B Y
C Y
B X
B X
A X
A X
A X
A X
B Z
B Z
A Z
B X
B X
B X
B X
B X
B Z
B Z
B Z
B Z
B Z
B Y
B Z
B Z
A Y
A Z
C Y
B Y
A X
B Z
B X
A Y
B Y
A Y
C Z
A X
A Y
B X
B X
C Z
B Y
A Y
A Y
A X
B X
A X
B X
A Y
B Z
B Z
A Z
B X
B Z
B Z
B Y
A Y
B Z
B Y
B Z
B Z
B Z
C X
B Y
A Y
B Z
A Y
A Y
A X
B X
B Y
C Y
C X
B Z
B Z
B Z
A Y
A Z
B Y
B X
B X
B Y
B Z
B Z
A Y
B Y
B Z
B Z
B Z
A X
C Y
B Z
A Z
C Z
B X
B Y
B Z
A Z
A X
B X
A X
C X
B Z
A X
B Y
C Y
B Y
B Z
B X
A Y
B Z
B Z
B Z
A X
B Z
B Z
B Z
B Z
A X
B Z
B X
B X
B Z
C X
A Y
B Z
B X
B Z
B Y
B Z
B Y
C Y
A X
B Y
B Z
B Z
B X
B Z
A X
B Z
A X
B Y
B Z
A X
B Z
B X
A Z
B Z
C X
B Z
B Y
B Y
B Y
B Z
B X
B X
A X
A X
B Z
B Z
A X
A X
A Z
A Y
C Z
B Z
B Y
C X
B X
B Y
B Y
A Y
B Z
B Z
B Y
C Y
B Z
A Y
B Z
B Z
B Z
B Z
A Y
B Z
B Z
B Z
A X
B X
A Y
B Y
B Z
B Z
B Z
A Y
B Z
B Z
B Z
B Y
B Y
B X
B X
B Y
C Z
B X
B Y
C Y
A Y
B Z
A Z
B Z
A Y
B X
B Z
B X
C Z
A Y
B X
B Z
A X
A X
B Z
B Z
B Y
B Z
C Y
C X
B Z
B Z
A X
B X
A Y
B Y
B Y
C Z
A X
B X
B Z
A Y
B Z
C X
B Z
A Z
C X
A Y
A Z
B Z
A Y
A X
B Z
B Y
B Z
B X
B Z
A X
B Z
A Z
B Z
B Z
B Y
A X
B Z
B X
B Y
B Y
A Y
A X
C Z
B Y
C Z
A X
B X
B Z
B X
C X
B Z
B Z
A X
A Y
A Z
A Z
B Z
B Z
A X
B Z
B Y
A X
A Y
B Z
B X
A Y
B Z
B Y
B X
B Z
A Z
B Y
A Y
A X
B Z
B Z
B Z
B Z
A X
B Y
B Z
B Z
B Z
A Z
B Y
A X
C Y
B Z
B Z
B Y
B X
B Z
A Y
A X
B Z
B Y
B Z
A Y
C X
B Y
B Z
B Z
A Y
B X
B Z
B Z
A X
B X
B Z
A Z
B Y
B Z
B Z
B Z
B Z
B Z
A Z
B X
A Y
C Y
B Z
A Y
B Z
B Z
A Y
B Z
B Z
A Y
A Y
B X
B Y
A X
B Z
A Y
B Z
A Z
B Z
B X
B Z
B Z
B Y
B Y
B Z
A X
B Y
B Z
B Z
B Z
B Y
A Y
B Z
B Y
B Z
B Z
B X
A X
B Y
A X
A X
B Z
A X
B Z
C X
B Y
B Z
B Z
B Y
A X
B Z
B Z
B Y
B X
B Z
B X
B Z
C Y
B Y
B Z
B Z
A Z
B Z
B Z
A Y
B Z
B Y
A X
B Z
B Z
B Z
B X
B Z
B Y
B Z
B X
C Z
B Y
B Z
B Z
B Y
A X
A X
B Y
A Y
B Z
B X
B Y
B Z
B Y
B Z
A X
C X
B Z
A Z
C Y
A X
B Y
B Y
A X
A X
B Y
B Z
C Y
B Z
B Y
A X
B Y
B Y
B Z
B X
B Z
C X
C Y
B X
A Y
B Z
B Z
B Z
A X
B Z
B Z
B X
B X
B X
B X
B Y
A Y
B Z
B Z
B Z
A Z
B X
B X
B X
B X
B Z
A Y
B Y
B Z
A X
A X
A X
B Y
B Z
B X
B Z
A Z
B Z
B Z
A Y
C Z
A X
B Z
C X
B Z
B Z
B Z
B Y
B Y
B Z
C X
B Z
A Y
B X
C X
A Y
C X
B Z
B X
A Z
B Z
C Y
B Y
B X
B Z
B Y
A Y
B X
A Z
B Z
B Z
C Z
B Z
B Y
B Z
A Y
C Y
B Z
B X
A Y
B Z
B Z
B Y
B Y
B Z
A X
A Y
A X
A Z
B Z
B Z
B Y
A Y
B Y
B Z
B Y
A Y
B X
A X
B Y
B Y
B X
B Z
B Y
C X
B Z
B Z
B Z
A X
B Z
B X
A Y
A Y
B Z
A Y
A Y
A Y
B Y
B Z
A X
C X
B Y
A X
C X
B Z
B Y
A Y
B X
B Z
B Y
B Y
B Z
B Z
A Y
B X
B X
A X
B Z
B X
B Z
B Z
C Z
C Y
A X
B X
C X
B X
A Z
A X
B Y
C X
B Y
A X
A X
A X
B X
B Z
B X
B Y
B Z
B Z
B Z
B Z
A X
A X
B X
B Z
A Z
A X
B Z
C Y
B Z
A X
B Z
A Z
B Z
B Z
A Y
B Y
B Z
B X
B Z
B Z
C Y
B Z
B X
A X
B Y
B Z
A Y
A X
B Z
B Y
B Z
B X
A Y
C Z
B X
B Z
A X
B X
B Z
B Z
A X
B Y
B X
B Z
B Z
B Y
B X
B X
C X
B Z
C X
B Z
C Y
A Y
C X
B X
B Y
B Z
B Y
B Z
A X
B Z
B Z
B Y
B Y
A X
A X
A Y
B Z
B Z
A Y
B Z
A X
B Z
B X
B X
B X
A Y
B Z
B Z
A Y
B Z
A Z
B Y
B X
A Z
A Z
B Z
A Y
B Z
B Y
B Y
B X
A Y
B X
B Z
B Z
B Y
B X
B Z
B Z
B Z
B Z
A X
B X
B X
B Z
A X
B X
B Z
B Z
B Y
A Y
B Z
B Y
B Y
A X
B X
B Y
C X
C X
B Y
A Y
A Z
B X
B Z
B Z
B Z
B Z
A X
A Y
A Y
B Y
C X
B Z
A Y
B X
C Z
B X
B Y
A Y
B X
B Y
B Z
B Y
B X
B Y
A Z
B Z
A X
B X
B X
B Z
B Y
B Y
B Y
A X
B X
B Y
B Z
A Y
A X
B X
B Z
B Z
C X
A Z
B Z
A X
A Y
C X
B Z
B Z
A X
B X
A Y
A Y
B Z
B Z
B X
A Z
B Z
B Y
A Y
B Z
B Y
A Y
B Y
B Z
A X
B Y
A X
A Y
A X
A X
B Y
A Z
B Z
A Y
B Z
A Z
A Y
B Z
B Y
B Z
B X`;
