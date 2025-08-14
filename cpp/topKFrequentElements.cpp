#include <algorithm>
#include <iostream>
#include <set>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
   public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> freqMap;

        for (auto num : nums) {
            freqMap[num]++;
        }

        vector<int> numbers;

        for (auto num : freqMap) {
            numbers.push_back(num.first);
        }

        sort(numbers.begin(), numbers.end(),
             [&freqMap](int e1, int e2) { return freqMap[e1] > freqMap[e2]; });

        vector<int> answer;
        for (int i = 0; i < k; i++) {
            answer.push_back(numbers[i]);
        }
        return answer;
    }
};