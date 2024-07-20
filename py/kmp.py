def kmp_prefix(p):
    m = len(p)
    ans = [-1] * m
    k = -1
    for q in range(1, m):
        while k > -1 and p[k + 1] != p[q]:
            k = ans[k]
        if p[k+1] == p[q]:
            k += 1
        ans[q] = k
    return ans

def search(t, p):
    n, m = len(t), len(p)
    kmp = kmp_prefix(p)
    q = -1
    for i in range(n):
        while q > -1 and p[q + 1] != t[i]:
            q = kmp[q]
        if p[q + 1] == t[i]:
            q += 1
            if q == m-1:
                print(f'found at shift {i-m+1}: {t[i-m+1:i+1]}')
                q = kmp[q]

if __name__ == '__main__':
    p = 'ababaca'
    print(kmp_prefix(p))
    search('acaababacababacaabdefetdfababacaasda', p)