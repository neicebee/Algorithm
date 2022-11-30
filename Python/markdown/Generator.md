# 💻 Generator

## 🤔 What is Generator?

- 발생자: Iterator를 생성하는 객체 -> Special Iterator
  - 모든 Generator는 Iterator에 속함
- Comprehension 문법 혹은 함수 내부 yield 키워드로 생성 가능
- Lazy함
  
### 👉🏻 Make Generator with Comprehension

```python
# Comprehension Generator
com_gen = (x for x in range(5))
print(type(com_gen)) # <class 'generator'>
for _ in range(5):
    print(com_gen.__next__())
```

- Comprehension 문법으로 생성된 Generator를 `Generator Expression` 이라고 칭함
- List Comprehension은 [] 사용, Generator Expression은 () 사용

### 👉🏻 Make Generator with yield

```python
# yield Generator 1
def generator_1():
    for i in range(5):
        yield i
print(type(generator_1())) # <class 'generator'>
num = generator_1()
for _ in range(5):
    print(num.__next__())
```

- 함수 내에서 yield를 사용하면 그 함수는 무조건 Generator가 됨
- yield가 함수 내에서 전혀 사용되지 않더라도 Generator가 됨
- next로 값 접근 가능
- Generator는 Iterator이기 때문에 불러올 값이 존재하지 않으면 StopIterator가 발생함

### 👉🏻 yield Keyword

```python
_check_list = ['current', 'next', 'end']
# yield Generator 2
def generator_2():
    for i in range(len(_check_list)):
        yield _check_list[i]
check1 = generator_2()
print(check1.__next__())
print(check1.__next__())
print(check1.__next__())
```

- return과 비슷하게 동작하는 듯 싶지만 큰 차이가 있음

  1. Generator는 yield로 값을 반환함
  2. next나 for 문으로 값들을 반환받을 수 있는데 yield가 호출된다고 해서 함수가 종료되지 않음
  3. yield는 호출되면 값을 반환하고 잠시 함수를 정지시킴
  4. 다음 next가 호출되면 정지된 시점부터 로직 실행
  5. return은 호출되면 함수 종료
  6. Generator 내부에 return을 사용 시 현재 값에 상관없이 StopIterator 발생

### 👉🏻 Generator로 range() 구현해보기

```python
# Generator range
def generator_range(start, stop):
    while start < stop:
        yield start
        start += 1
print(generator_range(0, 5)) # <generator object generator_range at 0x________>
result = [x for x in generator_range(0, 5)]
print(result)
```

### 👉🏻 yield from

```python
# yield from
def generator_from():
    yield from _check_list
check2 = generator_from()
print(check2.__next__())
print(check2.__next__())
print(check2.__next__())
```

- iterable한 객체의 요소 하나씩 반환 가능 => for문 순회 기능
- Generator 객체 전달도 가능

### 👉🏻 send

```python
# Generator send
def generator_send():
    main_value = 0
    while 1:
        main_value = yield
        yield main_value * 2
gen = generator_send()
print(gen) # <generator object generator_send at 0x________>
gen.__next__()
print(gen.send(100)) # 200
gen.__next__()
print(gen.send(500)) # 1000
```

- send 함수는 메인인 함수 호출 영역에서 Generator 내부로 값 전달 가능
- 변수에 yield를 할당하여 사용 가능
- 즉, 양방향 통신이 가능

### 💯 **Lazyevaluation**

- Iterator에 대한 글에서 lazyevaluation에 대해 언급했었음
  - Lazyevaluation(지연 평가): 값을 미리 정해놓는 것이 아닌 필요한 값을 그때그때 처리함 => 메모리 효율성이 좋음
- Iterator는 `지연 평가 구조체`
- 즉, Iterator의 부분집합인 Generator도 `지연 평가 구조체` 임