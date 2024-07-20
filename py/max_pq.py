class Item:
    def __init__(self, id, key):
        self.id = id
        self.key = key

    def __repr__(self):
        return f'({self.id}, {self.key})'
    
class MaxPriorityQueue:
    def __init__(self):
        self.heap = [0]
        self.id_to_idx = {}
    

    def _max_heapify(self, i):
        l = 2 * i
        r = 2 * i + 1
        if l < len(self.heap) and self.heap[l].key > self.heap[i].key:
            largest = l
        else:
            largest = i
        if r < len(self.heap) and self.heap[r].key > self.heap[largest].key:
            largest = r
        if largest != i:
            self.id_to_idx[self.heap[i].id] = largest
            self.id_to_idx[self.heap[largest].id] = i
            self.heap[i], self.heap[largest] = self.heap[largest], self.heap[i]
            self._max_heapify(largest)

    def peek(self):
        if len(self.heap) == 1:
            return None
        else:
            return self.heap[1]
    
    def pop(self):
        if len(self.heap) == 1:
            return None
        else:
            ans = self.heap[1]
            del self.id_to_idx[ans.id]
            if len(self.heap) > 2:
                self.heap[1] = self.heap.pop()
                self.id_to_idx[self.heap[1].id] = 1
                self._max_heapify(1)
            else:
                self.heap.pop()
            return ans
    
    def increase_key(self, id, k):
        i = self.id_to_idx[id]
        x = self.heap[i]
        if k < x.key:
            raise ValueError("cannot decrease key")
        x.key = k
        while i > 1 and self.heap[i // 2].key < self.heap[i].key:
            self.id_to_idx[self.heap[i].id] = i // 2
            self.id_to_idx[self.heap[i // 2].id] = i
            self.heap[i//2], self.heap[i] = self.heap[i], self.heap[i//2]
            i = i // 2

    def insert(self, x):
        self.heap.append(x)
        k = x.key
        x.key = float('-inf')
        self.id_to_idx[x.id] = len(self.heap) - 1
        self.increase_key(x.id, k)


if __name__ == '__main__':
    pq = MaxPriorityQueue()
    for i in range(1, 11):
        pq.insert(Item(i, i))
    pq.increase_key(2, 6)
    for i in range(10):
        print(pq.pop())
        