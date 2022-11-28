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
            - iter(object to callable, sentinel)
            - 반복을 끝낼 값을 지정할 수 있음
        - .__iter__() 사용
    - Iterator은 두 함수로 다음 값을 반환할 수 있음
        - next()
        - .__next__()

Loop & Iterator
    - Loop는 실행과 동시에 값을 계속 만들어내며 메모리 차지
    - Iterator은 순서대로 다음 값을 반환할 수 있는 순회 가능한 객체를 생성한 후 필요한 시점에 next 함수로 작동하기에 효율성이 좋음
        - 데이터 생성을 미루는 방식을 lazyevaluation 이라고 칭함
    - 파이썬 for문은 내부적으로 Iterator를 생성해 동작
        - 해당 객체의 iter를 생성해 next로 순회
        - Stopiteration 발생 시 순회 중단
    - Iterator는 예외 처리를 해야 하는 while보다는 StopIteration을 쉽게 제어할 수 있는 for와 함께 자주 쓰임
'''

# iterator 객체 구현해보기
# class Class_of_Iteration:
#     def __init__(self, start_num, stop_num):
#         self.pointer = start_num
#         self.max = stop_num
    
#     def __iter__(self):
#         return self
    
#     def __next__(self):
#         if self.pointer > self.max:
#             raise StopIteration
#         else:
#             self.pointer+=1
#             return self.pointer-1

# A = Class_of_Iteration(1, 3)
# print(type(A))
# print(A.__next__())
# print(A.__next__())
# print(A.__next__())
# print(A.__next__()) # 예외발생: StopIteration

# while
import random

while_res = []

while 1:
    i = random.randint(0, 10)
    if i == 2:
        break
    else:
        while_res.append(i)

print(while_res)

# iterator
# lambda 매개변수 : 표현식
iter_res = [x for x in iter(lambda: random.randint(0, 10), 2)]
print(iter_res)

N = ['f1r3', '_', 'r41n']
# while
while_iter = iter(N)
while 1:
    try:
        i = next(while_iter)
    except StopIteration:
        break
    print(i, end='')

# for
for_iter = iter(N)
for _ in range(len(N)):
    print(for_iter.__next__(), end='')

# num_list_1 = [i for i in range(5)].__iter__()
# print(num_list_1)

# for i in num_list_1:
#     print(i, end=' ')
# # list_iterator object
# print('\n')

# num_list_2 = (j for j in range(5))
# print(num_list_2)

# for j in num_list_2:
#     print(j, end=' ')
# # generator object
# print('\n')

# num_list_3 = iter(range(5))
# print(num_list_3)

# for x in num_list_3:
#     print(x, end=' ')
# # range_iterator object
# print('\n')

# num_list_4 = map(lambda x: x, [0, 1, 2, 3, 4])
# print(num_list_4)

# for y in num_list_4:
#     print(y, end=' ')
# # map object