#include <string>
using namespace std;

class Solution
{
public:
    string sortVowels(string s)
    {
        char vowels[10] = {'A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'};
        int counts[10];
        fill_n(counts, 10, 0);

        for (char c : s)
        {
            switch (c)
            {
            case 'A':
                ++counts[0];
                break;
            case 'E':
                ++counts[1];
                break;
            case 'I':
                ++counts[2];
                break;
            case 'O':
                ++counts[3];
                break;
            case 'U':
                ++counts[4];
                break;
            case 'a':
                ++counts[5];
                break;
            case 'e':
                ++counts[6];
                break;
            case 'i':
                ++counts[7];
                break;
            case 'o':
                ++counts[8];
                break;
            case 'u':
                ++counts[9];
                break;
            default:;
            }
        }
        string ans;
        ans.reserve(s.size());

        int pos = 0;
        for (char c : s)
        {
            switch (c)
            {
            case 'A':
            case 'E':
            case 'I':
            case 'O':
            case 'U':
            case 'a':
            case 'e':
            case 'i':
            case 'o':
            case 'u':
                while (counts[pos] == 0)
                {
                    ++pos;
                }
                --counts[pos];
                ans.push_back(vowels[pos]);
                break;
            default:
                ans.push_back(c);
            }
        }

        return ans;
    }
};
