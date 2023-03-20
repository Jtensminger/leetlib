

pub fn longest_palindrome(s: String) -> i32 {
        let mut table: [i32; 58] = [0; 58];
        let mut found_odd = false;
        s.as_bytes().iter().for_each(|ch| table[(ch - b'A') as usize] +=  1);

        let sum: i32 = table.iter_mut().fold(0, |sum, count| {
                if *count == 0 { sum } 
                else {
                        found_odd = found_odd || *count % 2 == 1;
                        sum + (*count / 2) * 2
                }
        });

        sum + if found_odd {1} else {0}
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
               assert_eq!(7, longest_palindrome(String::from("abccccdd"))) 
        }

        #[test]
        fn ext2() {
                assert_eq!(1, longest_palindrome(String::from("Aa"))) 
         }

        #[test]
        fn ext3() {
                assert_eq!(2, longest_palindrome(String::from("aa"))) 
        }

        #[test]
        fn ext4() {
                let w = String::from("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth");
                // println!("{}", (b'Z' - b'A') as usize);
                assert_eq!(983, longest_palindrome(w)) 
        }
}


/*
NOTES:

- Every non-empty String contains atleast 1 palindrome with length of 1


- if we generate every permutation of string, we could check valid_palindrome just like lc_0125
- then just take the max between the current permutation & the last longest palindrome

- I think we could lower time copmlexity by just tracking counts on each combo instead of generating it.

- we could store the counts in a hashtable-like structure, just like lc_0383 or via hashing letters manually & storing in array

- This means this is a dynamic programming problem using Strings



Dynamic Programming: the two main properties of a problem that suggest that the problem can be solved using DP: 
        1) Overlapping Subproblems (Using Tabulation on this problem since no recursion)
        2) Optimal Substructure



Algorithm:

        1) simulate a hashtable w/ a 52 length array filled, initialized w/ 0s
                'A' - current letter as hashing function. e.g 'A' - 'A' = 0

        2) create max_odd variable to hold largest count of an odd number

        3) create even_sum variable to hold the running sum of all the even counts
        

        4) For each letter, 
                3a) increment it's location in the array table arr['A'-current_letter]+=1
        
        5) For each hashtable entry:
                5a) if curr_letter % 2 != 0: max_odd = Max(max_odd, current_letter)
                5b) return Sum(even_counts) + max_odd

 */