/* #TAGS[String] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/ransom-note/] */

use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() == 0 || magazine.len() == 0 { return false }
        if ransom_note.len() > magazine.len() { return false }

        let mut ran_b = ransom_note.into_bytes().into_iter();
        let mut mag_b = magazine.into_bytes().into_iter();

        // (item, count)
        let mut hm: HashMap<u8, i32> = HashMap::new();
        mag_b.for_each(|e| *hm.entry(e).or_insert(0) += 1);

        while let Some(m) = ran_b.next() {
                match hm.get_mut(&m) {
                        Some(count) => {
                                if *count == 0 { return false; }
                                *count -= 1;
                        },
                        None => return false
                }
        }
        true
}


#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ext1() {
                let ransom_note = String::from("a");
                let magazine = String::from("a");
                assert_eq!(true, can_construct(ransom_note, magazine));
        }

        #[test]
        fn ext2() {
                let ransom_note = String::from("a");
                let magazine = String::from("b");
                assert_eq!(false, can_construct(ransom_note, magazine));
        }

        #[test]
        fn ext3() {
                let ransom_note = String::from("b");
                let magazine = String::from("a");
                assert_eq!(false, can_construct(ransom_note, magazine));
        }

        #[test]
        fn ext4() {
                let ransom_note = String::from("a");
                let magazine = String::from("aa");
                assert_eq!(true, can_construct(ransom_note, magazine));
        }

        #[test]
        fn ext5() {
                let ransom_note = String::from("aa");
                let magazine = String::from("a");
                assert_eq!(false, can_construct(ransom_note, magazine));
        }
}

/*
Constraints on the input:
        1 <= ransomeNote.length, magazine.length <= 10^5
                implication -> !ransomNote.is_empty() && !magazine.is_empty() -> don't need this because input is always at least 1 item
                invariant -> randomNote.length > 0 && magazine.length > 0

        ransomNote can be constructed by using the letters from magazine
                implication -> each character in ransomNote must exist at least once in magazine

                implication  -> ransomeNote.length <= mag.length is necessary but not sufficient
                        valid:      ran[a] < mag[ab]
                        invalid:    ran[a] < mag[bb]
                
                
                implication -> Sum(ran) <= Sum(mag) is necessary but not sufficient
                        if we think of characters as ascii numbers, the ability to do math doesn't help solve the problem
                                98..=122 -> a..=z
                                valid:    ran[a] == mag[a] -> Sum([97]) == Sum([97])
                                valid:    ran[a] == mag[aa] -> Sum([97]) <= Sum([97, 97]) -> 194
                                valid:    ran[a] != mag[ab] -> Sum([97]) <= Sum([97, 98]) -> 195
                                invalid:  ran[a] != mag[b] -> Sum([97]) <= Sum([98])
                                invalid: ran[aa] != mag[a] -> Sum([97, 97]) -> 194 >= Sum([97])
                                invalid: ran[b] != mag[a] -> Sum([98]) >= Sum([97])
                                invalid: ran[ba] != mag[a] -> Sum([98, 97]) -> 195 >= Sum([97])
                                ran[] mag[] -> false
                                        [true] ran.length > mag.length
                                        [true] Sum(ran) 0 <= Sum(mag) 0

                                ran[a] mag[a] -> true
                                        [true] ran.length <= mag.length
                                        [true] Sum(ran) 97 <= Sum(mag) 97

                                ran[a] mag[b] -> false
                                        [true] ran.length <= mag.length
                                        [true] Sum(ran) 97 <= Sum(mag) 98

                                ran[b] mag[a] -> false
                                        [true] ran.length <= mag.length
                                        [false] Sum(ran) 98 <= Sum(mag) 97
                                
                                ran[a] mag[aa] -> true
                                        [true] ran.length <= mag.length
                                        [true] Sum(ran) -> 97 > Sum(mag) -> 194
                                        implication -> Sum(ran) > Sum(mag)
                
        Each letter in magazine can only be used once in ransomNote
                implication -> each unique item in randsomNote appears >= 1 in magazine

        is a String
                implication -> Strings are UTF-8 encoded -> sequence of 8-bit bytes
                implication -> Each character is encoded as 1 or 2 bytes
                implication -> Accessing requires a check if it's 1 or 2 byte char before accessing it
                implication -> Accessing nth character is 2*n operations

        only hold lowercase english letters                         
                implication -> lowercase english letters can fit into ASCII
                implication -> Each character is only 1 byte
                implication -> Accessing nth ASCI character  == nth byte
                implication -> Accessing nth character is n operations
                implication -> converting String to ascii chars 2x as fast        



Bruteforce: 
        convert both strings to ascii iterator
        ransomeNote.length == 0 || magazine.length == 0: return false      -> checks valid words
        ransomeNote.length > magazine.length: return false                 -> checks min required # of chars

*/