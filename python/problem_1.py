class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """

        numbers_map = {}
        for i in range(len(nums)):
            numbers_map[nums[i]] = i

        for i in range(len(nums)):
            diff = target - nums[i]
            print(nums[i], diff)
            diff_idx = numbers_map.get(diff)

            if diff_idx is not i and diff_idx is not None:
                return [i, diff_idx]

        return []
