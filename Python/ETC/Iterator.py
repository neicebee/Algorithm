'''
What is Iterator?
    - 반복자: 순서대로 다음 값을 반환할 수 있는 객체를 의미
    - 자체 내장된 next 함수를 통해 가져올 수 있음

Collection Type & Sequence Type
    - Collection: list, tuple, set, dict와 같이 여러 개의 요소를 갖는 데이터 타입
    - Sequence: list, tuple, range, str과 같이 순서가 존재하는 데이터 타입

Iterable & Iterator
    - 내부 요소들을 하나씩 반환할 수 있는 객체를 Iterable(반복 가능)하다고 함
    - for문을 통해 순회할 수 있는 객체
    - Iterable은 next 함수 X
    - Iterator은 next 함수 O
    - Iterable한 객체는 Iterator로 만들 수 있음
        - iter() 사용
        - .__iter__() 사용
    - Iterator은 두 함수로 다음 값을 반환할 수 있음
        - next()
        - .__next__()

for & Iterator
    - 파이썬 for문은 내부적으로 Iterator를 생성해 동작
    - 해당 객체의 iter를 생성해 next로 순회
    - Stopiteration 발생 시 순회 중단
'''

class Iteration:
    def __init__(self, stop_num):
        self.pointer = 0
        self.stop_num = stop_num
    
    def __iter__(self):
        return self
    
    def __next__(self):
        if self.stop_num > self.pointer:
            self.pointer+=1
            return self.pointer
        else:
            raise StopIteration

class Iterable:
    def __init__(self, stop_num):
        self.stop_num = stop_num

    def __iter__(self):
        return Iterator(self.stop_num)

class Iterator:
    def __init__(self, stop_num):
        self.pointer = 0
        self.stop_num = stop_num
    
    def __next__(self):
        if self.stop_num > self.pointer:
            self.pointer += 1
            return self.pointer
        else:
            raise StopIteration

A = Iteration(3)
print(A)

num_list_1 = [i for i in range(5)].__iter__()
print(num_list_1)

for i in num_list_1:
    print(i, end=' ')
# list_iterator object
print('\n')

num_list_2 = (j for j in range(5))
print(num_list_2)

for j in num_list_2:
    print(j, end=' ')
# generator object
print('\n')

num_list_3 = iter(range(5))
print(num_list_3)

for x in num_list_3:
    print(x, end=' ')
# range_iterator object
print('\n')

num_list_4 = map(lambda x: x, [0, 1, 2, 3, 4])
print(num_list_4)

for y in num_list_4:
    print(y, end=' ')
# map object