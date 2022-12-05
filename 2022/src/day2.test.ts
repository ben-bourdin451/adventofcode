import { getPlay, getScore, part1, part2, ShapeMe, ShapeOpponent } from "./day2";

describe('day 2', () => {
	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{
					input: `A Y
B X
C Z`,
					want: 15,
				},
				{
					input: `C Y
C Y
B Y
A Z
B Z`,
					want: 21,
				},
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});

		it('should calculate scores', () => {
			const cases = [
				{ op: 'A', me: 'X', want: 4 },
				{ op: 'A', me: 'Y', want: 8 },
				{ op: 'A', me: 'Z', want: 3 },
				{ op: 'B', me: 'X', want: 1 },
				{ op: 'B', me: 'Y', want: 5 },
				{ op: 'B', me: 'Z', want: 9 },
				{ op: 'C', me: 'X', want: 7 },
				{ op: 'C', me: 'Y', want: 2 },
				{ op: 'C', me: 'Z', want: 6 },
			]

			cases.forEach(c => {
				const op = ShapeOpponent[c.op as keyof typeof ShapeOpponent];
				const me = ShapeMe[c.me as keyof typeof ShapeMe];
				expect(getScore(op, me)).toEqual(c.want);
			});
		});
	});

	describe('part 2', () => {
		it('should pass test inputs', () => {
			const cases = [
				{
					input: `A Y
B X
C Z`,
					want: 12,
				},
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});

		it('should calculate scores', () => {
			const cases = [
				{ op: 'A', me: 'X', want: 3 },
				{ op: 'A', me: 'Y', want: 4 },
				{ op: 'A', me: 'Z', want: 8 },
				{ op: 'B', me: 'X', want: 1 },
				{ op: 'B', me: 'Y', want: 5 },
				{ op: 'B', me: 'Z', want: 9 },
				{ op: 'C', me: 'X', want: 2 },
				{ op: 'C', me: 'Y', want: 6 },
				{ op: 'C', me: 'Z', want: 7 },
			]

			cases.forEach(c => {
				const op = ShapeOpponent[c.op as keyof typeof ShapeOpponent];
				const me = ShapeMe[c.me as keyof typeof ShapeMe];
				expect(getPlay(op, me)).toEqual(c.want);
			});
		});
	});
});
