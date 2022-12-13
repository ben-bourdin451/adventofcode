import { parseFile, part1, part2 } from "./day7";

describe('day 7', () => {
	const testInput = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`;

	describe('part 1', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 95437 },
			]

			cases.forEach(c => expect(part1(c.input)).toEqual(c.want));
		});

		it('should be able to parse file inputs', () => {
			const cases = [
				{ input: '29116 f', want: [29116, 'f'] },
				{ input: '14848514 b.txt', want: [14848514, 'b.txt'] },
			]

			cases.forEach(c => expect(parseFile(c.input)).toEqual(c.want));
		})
	});

	describe('part 2', () => {
		it('should pass test inputs', () => {
			const cases = [
				{ input: testInput, want: 24933642 },
			]

			cases.forEach(c => expect(part2(c.input)).toEqual(c.want));
		});
	});
});
