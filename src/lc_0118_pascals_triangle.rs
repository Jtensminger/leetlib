/*  Dynamic Programming
 	Overlapping Sub-problems: 
		are repeating the same inputs to the same function
	Optimal Sub-structure:
		Optimal solution can be constructed from optimal solutions of its subproblems
	Memoization: 
		storing a calculated value for future use so you don’t have to calculate it again
	Base Case: 
		problem that we know the answer to, that can be solved without any more recursive calls or iterations

*/ 	

/* Solution Pattern

	Identify Base Case
	
	Identify Overlapping Sub-problems (i.e. repeated input parameters)
	
	Choose either Recursive Memoization or bottom-up iterative approach
		Iterative: 
			build solution iteratively by starting from base case
		Iterative + Memoization: 
			build solution iteratively by starting from base case, storing sub-problems in a table
		Recursive + Memoization: 
			define a recursive function & memo table
			recurse from the input down to the base case, store sub-problems in a table
	
*/

/* Notes on Pascal's Triangle
        generate the nth row of triange as Vec<i32>
	you have all values of previous rows (i.e. n rows, if n > 1, n-1 rows)
	row.len() == nth row
	row[n-1][0] = 1 && row[n-1][n-1] = 1
	
	total calculations per row: (1..=n-2), row[n-1][1..=n-2]
		50% of the calculations are repeated because the triangel is symetrical
			row[n-1][n-n] = 1
			row[n-1][n-1] = 1
			n%2 == 0: i=(1..n)
				
				row[n-1][i] = row[n-2][i] + row[n-2][i-1]
		
			n%2 != 0: i=(1..=(n/2)+1)
				row[n-1][i] = row[n-2][i] + row[n-2][i-1]
			row[n-1][i] = row[n-2][i] + row[n-2][i-1]
		

	[1] 	     n=1, 
	[1,1]     n=2, 
	[1,2,1]   n=3, 
	[1,3,3,1] n=4,	

*/


pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
	// base_cases = [1] & [1,1] & Memo Table = Pascal
	let mut pascal = vec![vec![1]];
	let num_rows = num_rows as usize;
	
	if num_rows == 1 {
		return pascal
	}
	pascal.push(vec![1,1]);
	
	// build solution iteratively by starting from base case, storing sub-problems in a table
	for r in 3..=num_rows  {
		let mut current_row = vec![1; r]; // all rows start & end with 1
		let previous_row = &pascal[r-2];

		// override the 1s with calculated values
		for i in 1..r-1 {
			current_row[i] = previous_row[i] + previous_row[i-1];		
		}

		pascal.push(current_row);
	}
	pascal
}


mod test {
	use super::*;

	#[test]
	fn ext1() {
		let solution = vec![
			vec![1],	// n = 1
			vec![1,1],	// n = 2
			vec![1,2,1],	// n = 3
			vec![1,3,3,1],	// n = 4
			vec![1,4,6,4,1], // n = 5
			vec![1,5,10,10,5,1], // n = 6
			vec![1,6,15,20,15,6,1] // n = 7
		];
		assert_eq!(solution, generate(7))
	}

	#[test]
	fn ext2() {
		let solution = vec![vec![1]];
		assert_eq!(solution, generate(1))
	}
}
