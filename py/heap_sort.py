class MaxHeap:
    def __init__(self, data):
        self.data = [0] + data
        self.heap_size = len(data)
        self.build_heap()

    def heapify(self, i):
        l = 2 * i
        r = 2 * i + 1
        if l <= self.heap_size and self.data[l] > self.data[i]:
            largest = l
        else:
            largest = i
        if r <= self.heap_size and self.data[r] > self.data[largest]:
            largest = r
        if largest != i:
            self.data[i], self.data[largest] = self.data[largest], self.data[i]
            self.heapify(largest)
    
    def build_heap(self):
        for i in range(self.heap_size//2, 0, -1):
            self.heapify(i)
    
    def sort(self):
        n = self.heap_size
        for i in range(n, 1, -1):
            self.data[i], self.data[1] = self.data[1], self.data[i]
            self.heap_size -= 1
            self.heapify(1)
        return self.data[1:]


if __name__ == '__main__':
    hp = MaxHeap(list(range(1, 17)))
    print(hp.sort())

    hp = MaxHeap([])
    print(hp.sort())
