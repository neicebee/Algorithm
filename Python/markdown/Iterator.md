# 💻 Iterator

## 🤔 What is Iterator?

- 반복자: 순서대로 다음 값을 반환할 수 있는 객체를 의미
- 자체 내장된 next 매직 메서드

**Iterator 객체 구현**
```python
class Class_of_Iteration:
    def __init__(self, start_num, stop_num):
        self.pointer = start_num
        self.max = stop_num
    
    def __iter__(self):
        return self
    
    def __next__(self):
        if self.pointer > self.max:
            raise StopIteration
        else:
            self.pointer+=1
            return self.pointer-1

A = Class_of_Iteration(1, 3)
print(type(A))
print(A.__next__())
print(A.__next__())
print(A.__next__())
print(A.__next__()) # 예외발생: StopIteration
```

### 👉🏻 Collection Type & Sequence Type

- Collection: list, tuple, set, dict와 같이 여러 개의 요소를 갖는 데이터 타입
- Sequence: list, tuple, range, str과 같이 순서가 존재하는 데이터 타입

### 👉🏻 Iterable & Iterator

- 내부 요소들을 하나씩 반환할 수 있는 객체를 `Iterable` (반복 가능)하다고 함
- for문을 통해 순회할 수 있는 객체
- Iterable은 next X
- Iterator은 next O
- Iterable한 객체는 Iterator로 만들 수 있음
  - `iter()` 사용
    - _`iter()` 에 대한 부가적인 설명_
      - `iter(object to callable, sentinel)`
      - 반복을 끝낼 값을 지정할 수 있음
  - `.__iter__()` 사용
- Iterator은 두 함수로 다음 값을 반환할 수 있음
  - `next()`
  - `.__next__()`

### 👉🏻 Loop & Iterator

- Loop는 실행과 동시에 값을 계속 만들어내며 메모리 차지
- Iterator은 순서대로 다음 값을 반환할 수 있는 순회 가능한 객체를 생성한 수 필요한 시점에 next 함수로 작동하기에 효율성이 좋음
  - 데이터 생성을 미루는 방식을 lazyevaluation 이라고 칭함
- 파이썬 for문은 내부적으로 Iterator를 생성해 동작
  - 해당 객체의 iter를 생성해 next로 순회
  - StopIteration 발생 시 순회 중단
- Iterator은 예외 처리를 해야하는 while보다는 StopIteration을 쉽게 제어할 수 있는 for와 자주 쓰임

**Iterator Control 1**
```python
# while
import random

while_res = []

while 1:
    i = random.randint(0, 10)
    if i == 4:
        break
    else:
        while_res.append(i)

print(while_res)

# iterator
# lambda 매개변수 : 표현식
iter_res = [x for x in iter(lambda: random.randint(0, 10), 4)]
print(iter_res)
```

- random module을 사용한 코드


**Iterator Control 2**
```python
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
```

- while과 for이 StopIterator을 어떻게 처리하는지에 대한 코드