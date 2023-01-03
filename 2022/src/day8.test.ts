import { part1, part2 } from "./day8";

describe('day 8', () => {
	const testInput = `30373
25512
65332
33549
35390`;

	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 21 },
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});
	});

	describe('part 2', () => {
		it.only('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 8 },
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});
	});
});
