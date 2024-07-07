import heapq
from collections import defaultdict
class Twitter:

    def __init__(self):
        self.follow_dict = defaultdict(set) # int -> set(int)
        self.tweet_dict = defaultdict(list) # 
        self.total_count = 0


    def postTweet(self, userId: int, tweetId: int) -> None:
        self.tweet_dict[userId].append((self.total_count, tweetId))
        self.total_count += 1

    def getNewsFeed(self, userId: int) -> List[int]:
        all_followed_tweets = [self.tweet_dict[userId]] + [self.tweet_dict[uid] for uid in self.follow_dict[userId]]
        hp = []
        ans = []
        for i, tweets in enumerate(all_followed_tweets):
            if tweets:
                heapq.heappush(hp, (-tweets[-1][0], tweets[-1][1], i, len(tweets) - 1))
        while hp:
            neg_count, tid, idx, t_idx = heapq.heappop(hp)
            ans.append(tid)
            if len(ans) == 10:
                break
            if t_idx > 0:
                heapq.heappush(hp, (-all_followed_tweets[idx][t_idx - 1][0], all_followed_tweets[idx][t_idx - 1][1], idx, t_idx - 1))
        return ans


    def follow(self, followerId: int, followeeId: int) -> None:
        self.follow_dict[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followeeId in self.follow_dict[followerId]:
            self.follow_dict[followerId].remove(followeeId)


# Your Twitter object will be instantiated and called as such:
# obj = Twitter()
# obj.postTweet(userId,tweetId)
# param_2 = obj.getNewsFeed(userId)
# obj.follow(followerId,followeeId)
# obj.unfollow(followerId,followeeId)