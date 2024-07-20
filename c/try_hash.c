#include <glib.h>
#include <stdio.h>

int main() {
    GHashTable* hash_set = g_hash_table_new(g_int_hash, g_int_equal);

    int* num1 = g_malloc(sizeof(int));
    *num1 = 1;
    g_hash_table_add(hash_set, num1);

    int* num2 = g_malloc(sizeof(int));
    *num2 = 2;
    g_hash_table_add(hash_set, num2);

    int key = 1;
    gboolean found = g_hash_table_contains(hash_set, &key);
    printf("Contains 1: %s\n", found ? "true" : "false");

    g_hash_table_remove(hash_set, &key);
    found = g_hash_table_contains(hash_set, &key);
    printf("Contains 1 after removal: %s\n", found ? "true" : "false");

    g_hash_table_destroy(hash_set);
    g_free(num1);
    g_free(num2);

    return 0;
}
