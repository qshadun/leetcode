var restoreArray = function(adjacentPairs) {
    const graph = new Map();
    for (let [a, b] of adjacentPairs) {
        if (!graph.has(a)) {
            graph.set(a, [b]);
        } else {
            graph.get(a).push(b);
        }
        if (!graph.has(b)) {
            graph.set(b, [a]);
        } else {
            graph.get(b).push(a);
        }
    }

    let prev = 0, cur = 0;
    const ans = [];
    for (let [k, v] of graph) {
        if (v.length == 1) {
            prev = k;
            cur = v[0];
            ans.push(k);
            ans.push(v);
            break;
        }
    }

    for (let i = 2; i < graph.size; i++) {
        for (let num of graph.get(cur)) {
            if (num != prev) {
                prev = cur;
                cur = num;
                ans.push(cur);
                break;
            }
        }
    }
    return ans;
};