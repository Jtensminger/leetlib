/* #TAGS[Binary Tree, Binary Search] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/first-bad-version/] */

pub struct ProductLog {
        versions: Vec<i32>,
        first_bad_version: i32,
        checks: u32
}

impl ProductLog {
        pub fn new(version_count: i32, first_bad_version: i32) -> Self {
                Self { 
                        versions: (1..=version_count).collect(),
                        first_bad_version,
                        checks: 0_u32
                }
        }

        pub fn first_bad_version(&mut self, _n: i32) -> i32 {
                let v = self.versions.clone();
                let i = v.partition_point(|x| !self.is_bad_version(*x));
                self.versions[i]
        }


        fn is_bad_version(&mut self, n: i32) -> bool {
                self.checks += 1;
                n >= self.first_bad_version
        }
}


#[cfg(test)]
mod test {
    use super::*;

        #[test]
        fn ext1() {
                let mut log = ProductLog::new(5, 4);
                let output = log.first_bad_version(5);
                
                println!("checks: {}", log.checks);
                assert_eq!(output, log.first_bad_version);
        }

        #[test]
        fn ext2() {
                let mut log = ProductLog::new(1, 1);
                let output = log.first_bad_version(1);
                
                
                assert_eq!(output, log.first_bad_version);
        }
}