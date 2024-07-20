#include <glib.h>

int distributeCandies(int* candyType, int candyTypeSize) {
    GHashTable* hash_set = g_hash_table_new(g_int_hash, g_int_equal);
    int half = candyTypeSize /2;
    for (int i = 0; i < candyTypeSize; i++) {
        int* num1 = g_malloc(sizeof(int));
        *num1 = candyType[i];
        g_hash_table_add(hash_set, num1);
    }
    int kinds = g_hash_table_size(hash_set);
    if kinds > half {
        return half;
    } else {
        return kinds;
    }

}