#include <deque>
#include <utility>
using namespace std;

class HitCounter {
public:
    HitCounter(): total(0), q() {
        
    }
    
    void hit(int timestamp) {
        if (q.empty() || q.back().first < timestamp) {
            q.push_back({timestamp, 1});
        } else {
            q.back().second += 1;
        }
        total += 1;
    }
    
    int getHits(int timestamp) {
        while(!q.empty() && q.front().first <= timestamp - 300) {
            total -= q.front().second;
            q.pop_front();
        }
        return total;
    }
private:
    int total;
    deque<pair<int, int>> q;
};

int main() {

}