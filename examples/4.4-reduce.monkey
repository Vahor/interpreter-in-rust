let reduce = fn(arr, initial, f) {
    let iter = fn(arr, result) {
        if (len(arr) == 0) {
            result
        } else {
            iter(rest(arr), f(first(arr), result))
        }
    };

    iter(arr, initial);
};

let sum = fn(arr) {
    reduce(arr, 0, fn(x, y) { x + y });
};

sum([1, 2, 3, 4, 5]);

