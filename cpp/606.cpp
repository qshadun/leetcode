/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <string>
using namespace std;
class Solution
{
public:
    string tree2str(TreeNode *root)
    {
        if (root == nullptr)
            return "";
        string ans = to_string(root->val);
        if (root->left || root->right)
        {
            ans.push_back('(');
            ans += tree2str(root->left);
            ans.push_back(')');
        }
        if (root->right) {
            ans.push_back('(');
            ans += tree2str(root->right);
            ans.push_back(')');
        }
        return ans;
    }
};