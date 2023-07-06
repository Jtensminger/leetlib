/* 
        Given a string s that represents a DNA sequence, (For example, "ACGAATTCCG" is a DNA sequence.)
        return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.
        You may return the answer in any order. 
*/

/* sliding window strategy */
use std::collections::HashMap;
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let chars = s.chars().collect::<Vec<char>>();
        let mut sequences = chars.windows(10);
        let mut counts = HashMap::with_capacity(sequences.len());
        while let Some(subsequence) = sequences.next() {
                *counts.entry(subsequence.iter().collect::<String>()).or_insert(0) += 1;
        }
        let mut res = Vec::with_capacity(counts.len());
        for (subsequence, count) in counts.drain() {
                if count > 1 {
                        res.push(subsequence);
                }
        }
        res
}


#[cfg(test)]
pub mod tests {
        use super::*;
        use std::collections::HashSet;
        #[test]
        fn ext1() {
                let correct_responses = HashSet::from([
                        "AAAAACCCCC".to_string(),
                        "CCCCCAAAAA".to_string()
                ]);

                let sequences = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());
                for s in sequences {
                        assert!(correct_responses.contains(&s));
                }
        }
        #[test]
        fn ext2() {
                let correct_responses = HashSet::from([
                        "AAAAAAAAAA".to_string()
                ]);
                let sequences = find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string());
                for s in sequences {
                        assert!(correct_responses.contains(&s));
                }
        }

        #[test]
        fn ext3() {
                let mut correct_responses = HashSet::from([
                        "AAAAACCCCC".to_string(),
                        "CCCCCAAAAA".to_string()
                ]);

                let sequences = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string());
                for s in sequences {
                        assert!(correct_responses.contains(&s));
                        correct_responses.remove(&s);
                }
                assert!(correct_responses.is_empty())
                
        }
}


/* 
        let mut windows: Vec<usize> = (10..).take(s.len()).step_by(10).collect();
        if windows[windows.len() - 1] != s.len() {
                if windows[windows.len() - 1] < s.len() {
                        windows.push(s.len());
                } else {
                        windows.pop();
                        windows.push(s.len());
                }
        }

        for (i, e) in windows.iter().enumerate() {
              windows[i - 10]  
        }

 */