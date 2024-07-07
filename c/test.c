#include <stdio.h>

struct point {
    int x;
    int y;
};

struct point makepoint(int x, int y) {
    struct point tmp;
    tmp.x = x;
    tmp.y = y;
    return tmp;
}

struct point addpoint(struct point p1, struct point p2) {
    p1.x += p2.x;
    p1.y += p2.y;
    return p1;
}


int main(void) {
    
    struct point px = makepoint(1, 2);

    struct point py = px;
    py = addpoint(px, py);
    printf("%d, %d\n", py.x, py.y);
}