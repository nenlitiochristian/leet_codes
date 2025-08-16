function minEatingSpeed(piles: number[], h: number): number {
    const biggestPile = Math.max(...piles);

    function getTime(speed: number) {
        let time = 0;
        for (let pile of piles) {
            time += Math.ceil(pile / speed);
        }
        return time;
    }

    let left = 1;
    let right = biggestPile;
    let min = biggestPile;
    while (left <= right) {
        const middle = Math.floor((left + right) / 2);
        if (getTime(middle) <= h) {
            min = middle;
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }

    return min;
};

console.log(minEatingSpeed([3, 6, 7, 11], 8))
console.log(minEatingSpeed([30, 11, 23, 4, 20], 5))
console.log(minEatingSpeed([30, 11, 23, 4, 20], 6))