import { part1, part2 } from "./day6";

describe('day 6', () => {
	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: 'mjqjpqmgbljsphdztnvjfqwrcgsmlb', want: 7 },
				{ input: 'bvwbjplbgvbhsrlpgdmjqwftvncz', want: 5 },
				{ input: 'nppdvjthqldpwncqszvftbrmjlhg', want: 6 },
				{ input: 'nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg', want: 10 },
				{ input: 'zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw', want: 11 },
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});
	});

	describe('part 2', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: 'mjqjpqmgbljsphdztnvjfqwrcgsmlb', want: 19 },
				{ input: 'bvwbjplbgvbhsrlpgdmjqwftvncz', want: 23 },
				{ input: 'nppdvjthqldpwncqszvftbrmjlhg', want: 23 },
				{ input: 'nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg', want: 29 },
				{ input: 'zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw', want: 26 },
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});
	});
});
