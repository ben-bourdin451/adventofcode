import { part1, part2 } from "./day4";

describe('day 4', () => {
	const testInput = `2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8`;

	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 2 },
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});
	});

	describe('part 2', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 4 },
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});
	});
});
