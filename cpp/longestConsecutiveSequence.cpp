#include <algorithm>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
   public:
    int max(int a, int b) { return a > b ? a : b; }

    int get_parent(unordered_map<int, pair<int, int>>& map, int num) {
        if (map[num].first != num) {
            map[num].first =
                get_parent(map, map[num].first);  // Path compression
        }

        return map[num].first;
    }

    int longestConsecutive(vector<int>& nums) {
        unordered_map<int, pair<int, int>>
            map;  // Key: number, Value: (parent, count of items in the set)

        for (int num : nums) {
            if (map.find(num) != map.end()) {
                continue;  // Skip if the number is already in the map
            }

            map[num] = {num, 1};
            if (map.find(num - 1) != map.end() &&
                map.find(num + 1) != map.end()) {
                auto parent = get_parent(map, num - 1);
                auto other_set_count = map[num + 1].second;
                map[num + 1].first = parent;
                map[num].first = parent;

                map[parent].second += 1 + other_set_count;
            }

            else if (map.find(num - 1) != map.end()) {
                auto parent = get_parent(map, num - 1);
                map[parent].second += 1;
                map[num].first = parent;
            }

            else if (map.find(num + 1) != map.end()) {
                map[num + 1].first = num;
                map[num].second = map[num + 1].second + 1;
            }
        }

        int longest = 0;
        for (auto val : map) {
            longest = max(val.second.second, longest);
        }

        return longest;
    }
};