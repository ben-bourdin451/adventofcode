import { part1, input, part2, popMax } from "./day1";

describe('day 1', () => {
	const testInput = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`;

	describe('part 1', () => {
		it('should pass problem statement', () => {
			expect(part1(testInput)).toBe(24000);
		});
		it('should be the correct answer', () => {
			expect(part1(input)).toBe(71924);
		});
	});

	describe('part 2', () => {
		it('should pass problem statement', () => {
			expect(part2(testInput)).toBe(45000);
		});
		it('should be the correct answer', () => {
			expect(part2(input)).toBe(210406);
		});
	});

	describe('pop max', () => {
		it('should work', () => {
			const cases = [
				{
					input: [1, 2, 3],
					want: 3,
				},
				{
					input: [13, 42, 37],
					want: 42,
				},
			]

			cases.forEach(c => expect(popMax(c.input)).toEqual(c.want));
		});
	});
});
