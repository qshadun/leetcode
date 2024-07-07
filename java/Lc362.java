import java.util.*;
public class Lc362 {
    
}

class HitCounter {
    int total = 0;
    Deque<int[]> q = new ArrayDeque<>();
    public HitCounter() {
        
    }
    
    public void hit(int timestamp) {
        if (q.isEmpty() || q.getLast()[1] < timestamp) {
            q.addLast(new int[]{timestamp, 1});
        } else {
            q.getLast()[1] += 1;
        }
        total += 1;
    }
    
    public int getHits(int timestamp) {
        while(!q.isEmpty() && q.getFirst()[0] < timestamp - 300 + 1) {
            total -= q.getFirst()[1];
            q.removeFirst();
        }
        return total;
    }
}