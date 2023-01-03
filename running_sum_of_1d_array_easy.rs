//https://leetcode.com/problems/running-sum-of-1d-array/

impl Solution {

    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {

        let mut stack_answer: Vec<i32> = Vec::with_capacity(nums.len());



        for num in 0..nums.len() {

            stack_answer.push(nums[0..=num].to_vec().iter().sum());

        }



        stack_answer

    }

}
