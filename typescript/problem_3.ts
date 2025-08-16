function lengthOfLongestSubstring(s: string): number {
    let longest = 0;
    let i = 0;
    while (i < s.length) {
        const charSet = new Set<number>();
        let length = 0;
        for (let j = i; j < s.length; j++) {
            if (charSet.has(s.charCodeAt(j))) {
                break;
            }

            charSet.add(s.charCodeAt(j));
            length += 1;
        }

        longest = Math.max(longest, length);
        i++;
    }
    return longest;
};

console.log(lengthOfLongestSubstring("abcabcbb"))