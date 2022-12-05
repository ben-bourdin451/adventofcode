import { getBadge, getUniqueItem, part1, part2 } from "./day3";

describe('day 3', () => {
	const testInput = `vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`;

	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 157 },
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});

		it('should calculate priorities', () => {
			const cases = [
				{ input: 'vJrwpWtwJgWrhcsFMMfFFhFp', want: 16 },
			]

			cases.forEach(c => expect(getUniqueItem(c.input)).toEqual(c.want));
		});
	});

	describe('part 2', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 70 },
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});

		it('should be able to get badge priorities', () => {
			const cases = [
				{ input: testInput.split('\n').slice(0, 3), want: 18 },
				{ input: testInput.split('\n').slice(3), want: 52 },
			]

			cases.forEach(c => expect(getBadge(c.input)).toEqual(c.want));
		});
	});
});
