function generateParenthesis(n) {
    var solutions = [];
    openOrCloseParenthesis(solutions, "", n, 0);
    return solutions;
}
;
function openOrCloseParenthesis(solutions, accumulative, n, opened) {
    // we always either make a new parenthesis or close an existing one
    // base cases
    if (n === 0) {
        for (var i = 0; i < opened; i++) {
            accumulative += ")";
        }
        solutions.push(accumulative);
        return;
    }
    // open
    openOrCloseParenthesis(solutions, accumulative + "(", n - 1, opened + 1);
    // close
    if (opened) {
        openOrCloseParenthesis(solutions, accumulative + ")", n, opened - 1);
    }
}
console.log(generateParenthesis(3));
console.log(generateParenthesis(1));
