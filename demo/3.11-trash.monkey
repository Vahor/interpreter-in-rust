let counter = fn(x) {
    if (x > 400) {
        return true;
    } else {
        let foobar = 9999;
        counter(x + 1);
    }
};

counter(0);