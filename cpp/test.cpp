#include <vector>
#include <iostream>
using namespace std;


void print(const vector<int> & v) {
    for (int c: v) {
        cout << c << " ";
    }
    cout << endl;
}
int main() {
    vector<int> v1{1, 2, 3};
    vector<int> v2{4, 5, 6};

    vector<int>* vr1 = &v1;
    vector<int>* vr2 = &v2;

    print(*vr1);
    print(*vr2);
    
    vector<int>* tmp = vr1;
    vr1 = vr2;
    vr2 = tmp;

    print(*vr1);
    print(*vr2);
}