#include <algorithm>
#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;
class Solution {
   public:
    string longestDiverseString(int a, int b, int c) {
        string answer;
        vector<pair<int, char>> numbers;
        numbers.push_back(make_pair(a, 'a'));
        numbers.push_back(make_pair(b, 'b'));
        numbers.push_back(make_pair(c, 'c'));
        sort(numbers.begin(), numbers.end(), );

        while (!done(&numbers)) {
            if (numbers[0] >= 2) {
                numbers[0]
            }
        }
        return answer;
    }

    bool comparePair(pair<int, char> a, pair<int, char> b) {
        return a.first > b.first;
    }

    bool done(vector<int>* numbers) {
        int zeros = 0;
        for (int i = 0; i < 3; i++) {
            if (numbers->at(i) == 0) {
                zeros++;
            }
        }
        return zeros < 2;
    }
};