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
* 

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

- 동적 계획법은 무작정 반복 코드에도 적용이 가능 -> 반복적 동적 계획법
- **재귀적 동적 계획법은 함수를 호출하는 데 따르는 오버헤드가 발생하기 때문에 절대적으로는 반복적 동적 계획법보다 시간이 오래 걸림**
  - 하지만 둘 다 시간복잡도는 $O(n)$
  - 오버헤드(overhead) : 어떤 처리를 하기 위해 들어가는 간접적인 시간, 메모리 등을 칭함