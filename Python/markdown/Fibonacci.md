# 💻 Fibonacci Sequence _in Python_

## 💁‍♀️ What _Fibonacci Sequence_?
- 단순한 단조 증가(monotonically increasing) 수열로 앞의 두 수를 더해가며 생성되는 수열
  * $F_{i+1} = F_{i} + F_{i-1}$
    * `Fibonacci Seq` 공식 : $F_{i} = F_{i-1} + F_{i-2}$
  * $F_{0} = 0$
  * $F_{1} = F_{2} = 1$

- 위의 수식만 보면 Fibonacci 수열의 n번째 수를 찾는 코드는 쉽게 구현이 가능
- 아래에서 서술할 코드들에 1100을 각각 할당하여 주피터 노트북의 `%%timeit` 을 사용해 처리 시간을 확인해보자

## 👀 단순 재귀
- 단순히 공식만 대입하여 재귀적으로 풀이해보자

```python
def fibo(n):
    return n if n<=1 else fibo(n-1) + fibo(n-2)

num = int(input())
print(fibo(num))
```
* 공식에 따라 0번째 수는 0이며 1번째 수는 1
* n이 2 이상일 경우는 재귀 처리하며 값 반환
* n의 인자값에 32 이상의 숫자가 할당되면 결과값 반환 속도가 눈에 띄게 느려짐
* 필자의 터미널에서는 43 이상의 숫자부터 프로세스가 멈추는 것으로 확인됨
* **1100 값 할당 -> 연산 불가**

```python
fibo = lambda n: n if n<=1 else fibo(n-1) + fibo(n-2)

num = int(input())
print(fibo(num))
```
* 단순 재귀 코드를 람다표현식으로 나타낸 코드
* **1100 값 할당 -> 연산 불가**

- **_해당 방법은 함수가 한 번 호출되면 다시 두 번 호출되기 때문에 시간복잡도는 $O(2^n)$ 이 됨_**

## 👀 무작정 반복
- 보통 Fibonacci Sequence는 재귀함수에 대한 개념을 습득하려고 공부하는 알고리즘이지만 무작정 반복을 통해 결과값을 얻을 수 있음

```python
def fibo(n):
    fibo_arr = [0, 1]
    
    for i in range(len(fibo_arr), n+1):
        fibo_arr[i%2] = fibo_arr[(n-1)%2] + fibo_arr[(n-2)%2]
    
    return fibo_arr[n%2]

num = int(input())
print(fibo(num))
```
* Fibonacci Sequence 정의에 맞게 변수 초기화 후 n번째까지 반복을 통해 모든 항을 구함
* **1100 값 할당 -> 138 µs ± 1.1 µs**

```python
def fibo(n):
    if n <= 1: return n
    a, b = 0, 1
    
    for _ in range(n-1):
        a, b = b, a+b
    
    return b

num = int(input())
print(fibo(num))
```
* 단순 변수 두 개만으로 구현한 같은 방식의 코드
* **1100 값 할당 -> 42.1 µs ± 228 ns**

- **_해당 방법은 입력값인 n만큼 계산하기 때문에 시간복잡도는 $O(n)$_**
- 시간복잡도로 비교할 시 단순 재귀보다 훨씬 효율적인 것을 알 수 있음

## 👀 재귀적 동적 계획법

- 동적 계획법 : 부분문제를 해결하면 해당 값을 저장하는 공간 사용
- 배열을 사용한 재귀적 풀이
- 저장공간에 있는 부분문제의 값을 찾고 없으면 계산

```python
def fibo(n):
    fibo_arr = [0, 1]

    def iterator(n):
        if n<2: return n
        try:
            if fibo_arr[n]: return fibo_arr[n]
        except IndexError:
            pass

        fibo_arr.append(iterator(n-1)+iterator(n-2))
        return fibo_arr[n]
    
    return iterator(n)

num = int(input())
print(fibo(num))
```
* **1100 값 할당 -> 372 µs ± 1.27 µs**

- 동적 계획법은 무작정 반복 코드에도 적용이 가능 -> 반복적 동적 계획법
- **재귀적 동적 계획법은 함수를 호출하는 데 따르는 오버헤드가 발생하기 때문에 절대적으로는 반복적 동적 계획법보다 시간이 오래 걸림**
  - 하지만 둘 다 시간복잡도는 $O(n)$
  - 오버헤드(overhead) : 어떤 처리를 하기 위해 들어가는 간접적인 시간, 메모리 등을 칭함

```python
def fibo(n, __fibo_arr={0: 0, 1: 1}):
    if n in __fibo_arr: return __fibo_arr[n]

    __fibo_arr[n] = fibo(n-1)+fibo(n-2)
    return __fibo_arr[n]

num = int(input())
print(fibo(num))
```
* 파이썬 함수의 동작 방식을 활용한 재귀적 동적 계획법의 또 다른 코드
* _보통 함수 내에 선언된 데이터는 함수가 호출될 때 생성되고 함수가 종료될 때 폐기되며 자원 회수도 이루어짐_
  * **_위 코드와 같이 원하는 데이터를 함수 정의부에 적으면 그 자료구조는 함수가 정의될 때 생성되어 함수가 호출되거나 종료되거나 상관없이 함수 자체가 메모리에서 지워지기 전까지 값이 유지됨_**
  * 함수 실행 시 데이터가 꾸준히 변화하고 값이 보존되기에 예상하지 못한 에러를 마주할 수 있는 방식
  * 파이썬의 내부 동작 방식을 활용하기에 파이썬에서만 구현 가능한 방식
* **1100 값 할당 -> 272 µs ± 2.43 µs**

## 👀 행렬 연산
- 행렬 연산의 Fibonacci 공식
  - $(\frac{F_{n+1}} {F_{n}}$ $\frac{F_{n}} {F_{n-1}})$ $=$ $(\frac{1} {1}$ $\frac{1} {0})^n$
- 행렬 연산 중 곱셈과 거듭제곱에 대한 포스팅
  - (https://mathbang.net/562)[https://mathbang.net/562]
- $n^{100}$ 을 구하려면 $n$ 을 100번 곱해 구할 수 있음
  - 모든 자연수는 2의 제곱수의 합으로 나타낼 수 있음
    - $n^{100} = n^{64}*n^{32}*n^{4}$
- 다음과 같은 방법을 적용하여 부분문제의 값을 저장해두고 필요 시 꺼내서 사용하는 방식을 구현

```python
def fibo(n):
    # 2*2 배열 크기 고정값
    SIZE = 2
    # fibonacci 행렬 연산의 기본값
    BASE = [[1, 1], [1, 0]]
    # 행렬의 곱셈에 대한 항등원
    IDENTITY = [[1, 0], [0, 1]]

    # 행렬의 곱셉 연산 함수
    def square_matrix_operation(a, b, size=SIZE):
        # 결과값을 담을 배열 선언 및 초기화
        new = [[0 for _ in range(size)] for _ in range(size)]
        
        for i in range(size):
            for j in range(size):
                for k in range(size):
                    new[i][j] += a[i][k] * b[k][j]
        
        print(new)
        return new

    # 행렬의 n승을 구하기 위한 판단 함수
    def matrix_judgment(n):
        # 최종 결과물
        matrix = IDENTITY.copy()
        # 임시적인 배열
        tmp = BASE.copy()
        k = 0

        # 0 이상의 정수 k에 대한 자연수 n이 2**k 를 포함하는지 판단하는 코드
        while 2 ** k <= n:
            # 비트 연산자인 &와 <<를 사용하여 포함 여부 판단
            if n & (1 << k) != 0:
                print(f'matrix: {matrix}'); matrix = square_matrix_operation(matrix, tmp)
            k+=1
            # BASE를 통해 계속 2**k 행렬을 저장하는 코드 
            print(f'tmp: {tmp}'); tmp = square_matrix_operation(tmp, tmp)
        
        return matrix

    return matrix_judgment(n)[0][1]

num = int(input())
print(fibo(num))
```