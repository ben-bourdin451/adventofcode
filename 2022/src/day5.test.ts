import { part1, part2, extractMoves } from "./day5";

describe('day 5', () => {
	const testInput = ['',
		'    [D]    ',
		'[N] [C]    ',
		'[Z] [M] [P]',
		' 1   2   3 ',
		'',
		'move 1 from 2 to 1',
		'move 3 from 1 to 3',
		'move 2 from 2 to 1',
		'move 1 from 1 to 2'
	].join('\n');

	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 'CMZ' },
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});

		it('should be able to understand move instructions', () => {
			const cases = [
				{ input: 'move 1 from 2 to 1', want: [1, 2, 1] },
			]

			cases.forEach(c => expect(extractMoves(c.input)).toEqual(c.want));
		});
	});

	describe('part 2', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 'MCD' },
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});
	});
});
