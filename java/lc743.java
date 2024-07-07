import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        int[][] graph = new int[n + 1][n + 1];
        for (int i = 1; i <= n; i++) {
            Arrays.fill(graph[i], -1);
        }
        for (int[] time: times ) {
            graph[time[0]][time[1]] = time[2];
        }
        int ans = 0;
        Set<Integer> Q = IntStream.rangeClosed(1, n).boxed().collect(Collectors.toSet());
        int[] dist = new int[n + 1];

        PriorityQueue<List<Integer>> q = new PriorityQueue<>((x, y) -> x.get(0) - y.get(0));
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[k] = 0;
        q.offer(Arrays.asList(0, k));
        while (!q.isEmpty()) {
            List<Integer> pair = q.remove();
            int u = pair.get(1);
            if (!Q.contains(u)) continue;
            Q.remove(u);
            int w = pair.get(0);
            ans = w;
            for (int v: Q) {
                if (graph[u][v] != -1) {
                    int newDist = w + graph[u][v];
                    if (newDist < dist[v]) {
                        dist[v] = newDist;
                        q.offer(Arrays.asList(newDist, v));
                    }
                }
            }
        }
        return Q.isEmpty() ? ans : -1;
    }
}