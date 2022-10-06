# 💻 Baekjoon Loop Stage

## ⚙️ Multiplication Table

>Q. N을 입력받은 뒤, 구구단 N단을 출력하는 프로그램을 작성하시오. 출력 형식에 맞춰서 출력하면 된다.

>Input. 첫째 줄에 N이 주어진다. N은 1보다 크거나 같고, 9보다 작거나 같다.

>Output. 출력형식과 같게 N*1부터 N*9까지 출력한다.

```python
N = int(input())
for i in range(1,10): print(f'{N} * {i} = {N*i}')
```

```python
N=n=int(input())
exec("print(N, '*', n//N, '=', n); n+=N;"*9)
```

## ⚙️ Two Integer

>Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

>Input. 첫째 줄에 테스트 케이스의 개수 T가 주어진다.
각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)

>Output. 각 테스트 케이스마다 A+B를 출력한다.

```python
T = int(input()); exec("print(eval('+'.join(input())));"*T)
```

- [eval()과 exec()에 대해서](https://it-neicebee.tistory.com/130)

```python
T = int(input())
cnt = 0

while T>cnt:
    A, B = map(int, input().split())
    print(A+B)
    cnt+=1
```

## ⚙️ Sum

>Q. n이 주어졌을 때, 1부터 n까지 합을 구하는 프로그램을 작성하시오.

>Input. 첫째 줄에 n (1 ≤ n ≤ 10,000)이 주어진다.

>Output. 1부터 n까지 합을 출력한다.

```python
print(sum(range(1, int(input())+1)))
```

## ⚙️ Receipt

>Q. 준원이는 저번 주에 살면서 처음으로 코스트코를 가 봤다. 정말 멋졌다. 
그런데, 몇 개 담지도 않았는데 수상하게 높은 금액이 나오는 것이다! 
준원이는 영수증을 보면서 정확하게 계산된 것이 맞는지 확인해보려 한다.
영수증에 적힌,
    1. 구매한 각 물건의 가격과 개수
    2. 구매한 물건들의 총 금액
을 보고, 구매한 물건의 가격과 개수로 계산한 총 금액이 영수증에 적힌 총 금액과 일치하는지 검사해보자.

>Input. 첫째 줄에는 영수증에 적힌 총 금액 X가 주어진다.
둘째 줄에는 영수증에 적힌 구매한 물건의 종류의 수 N이 주어진다.
이후 N개의 줄에는 각 물건의 가격 a와 개수 b가 공백을 사이에 두고 주어진다.

>Output. 구매한 물건의 가격과 개수로 계산한 총 금액이 영수증에 적힌 총 금액과 일치하면 Yes를 출력한다. 일치하지 않는다면 No를 출력한다.

>Limit. 1 ≤ X ≤ 1\,000\,000\,000
1 ≤ N ≤ 100
1 ≤ a ≤ 1\,000\,000
1 ≤ b ≤ 10

```python
X, _, *e = open(0)
X = int(X)
for line in e: a, b = map(int, line.split()); X-=a*b
print("YNeos"[X!=0::2])
```

```python
X = int(input())
N = int(input())

sum = 0

for _ in range(N):
    a, b = map(int, input().split())
    sum += a*b

print("Yes" if X==sum else "No")
```

## ⚙️ Quick Sum

>Q. 본격적으로 for문 문제를 풀기 전에 주의해야 할 점이 있다. 입출력 방식이 느리면 여러 줄을 입력받거나 출력할 때 시간초과가 날 수 있다는 점이다.
Python을 사용하고 있다면, input 대신 sys.stdin.readline을 사용할 수 있다. 
단, 이때는 맨 끝의 개행문자까지 같이 입력받기 때문에 문자열을 저장하고 싶을 경우 .rstrip()을 추가로 해 주는 것이 좋다.
또한 입력과 출력 스트림은 별개이므로, 테스트케이스를 전부 입력받아서 저장한 뒤 전부 출력할 필요는 없다. 테스트케이스를 하나 받은 뒤 하나 출력해도 된다.

>Input. 첫 줄에 테스트케이스의 개수 T가 주어진다. T는 최대 1,000,000이다. 다음 T줄에는 각각 두 정수 A와 B가 주어진다. A와 B는 1 이상, 1,000 이하이다.

>Output. 각 테스트케이스마다 A+B를 한 줄에 하나씩 순서대로 출력한다.

[참고하기 1](https://it-neicebee.tistory.com/115)

[참고하기 2](https://it-neicebee.tistory.com/118)

### 메모리 초과 코드
```python
T = int(input())
exec("print(eval('+'.join(sys.stdin.readline().rstrip().split())));"*T)
```
- 입력값을 계속 읽고 메모리에 저장하기에 메모리 초과가 뜬 것이 아닐까하는 추측임
- 이 코드에서 sys.stdin이 아닌 open(0)을 사용하면 파일 객체를 닫지 않았다는 오류가 발생함

### 기본 코드
```python
import sys

T = int(input())

for _ in range(T):
    A, B = sys.stdin.readline().rstrip().split()
    print(int(A) + int(B))
```
- 무난한 통과 코드임

### 숏코드
```python
import sys

for n in [*sys.stdin][1:]:
    print(sum(map(int, n.split())))
```
- sys를 사용하지 않고 open(0) 객체를 사용해도 괜찮은 것 같다. 반복해서 파일을 여는 것이 아니기 때문에 파일 객체를 닫아야 한다는 필수 조건도 피할 수 있어보임

## ⚙️ Sum 2

>Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

>Input. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

>Output. 각 테스트 케이스마다 "Case #x: "를 출력한 다음, A+B를 출력한다. 테스트 케이스 번호는 1부터 시작한다.


### 내 풀이
```python
import sys

T=0
for n in [*sys.stdin][1:]:
    T+=1
    print(f"Case #{T}:", sum(map(int, n.split())))
```

- 굳이 sys 모듈을 import하여 사용할 필요는 없고 open(0)으로 치환하여 사용해도 괜찮을 것같다.

### 기본 코드
```python
import sys

T = int(input())

for x in range(T):
    A, B = sys.stdin.readline().rstrip().split()
    print(f"Case #{x+1}: {int(A)+int(B)}")
```

## ⚙️ Sum 3

>Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

>Input. 첫째 줄에 테스트 케이스의 개수 T가 주어진다.
각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)

>Output. 각 테스트 케이스마다 "Case #x: A + B = C" 형식으로 출력한다. x는 테스트 케이스 번호이고 1부터 시작하며, C는 A+B이다.

```python
T = 0

for A,_,B,_ in [*open(0)][1:]:
    T+=1
    print(f"Case #{T}: {A} + {B} = {int(A)+int(B)}")
```

## ⚙️ Star

>Q. 첫째 줄에는 별 1개, 둘째 줄에는 별 2개, N번째 줄에는 별 N개를 찍는 문제

>Input. 첫째 줄에 N(1 ≤ N ≤ 100)이 주어진다.

>Output. 첫째 줄부터 N번째 줄까지 차례대로 별을 출력한다.

### 내 풀이
```python
N=n=int(input()); exec("n-=1;print('*'*(N-n));"*N)
```
- exec를 사용
  
### 숏코드
```python
for i in range(int(input())): print('*'*-~i)
```
- 비트 반전 연산자 `~`를 사용한 코드
- [비트 연산자에 대한 글(비트 반전 연산자와 2의 보수에 대한 상세한 글 포함)](https://it-neicebee.tistory.com/133)

## ⚙️ Star 2

>Q. 첫째 줄에는 별 1개, 둘째 줄에는 별 2개, N번째 줄에는 별 N개를 찍는 문제
하지만, 오른쪽을 기준으로 정렬한 별(예제 참고)을 출력하시오.

>Input. 첫째 줄에 N(1 ≤ N ≤ 100)이 주어진다.

>Output. 첫째 줄부터 N번째 줄까지 차례대로 별을 출력한다.

```
출력예제
    *
   **
  ***
 ****
*****
```

```python
N=int(input())

for i in range(N):
    print(f"{'*'*(i+1):>{N}}")
```

- [파이썬의 문자열 포메팅에 대해서](https://it-neicebee.tistory.com/134)

## ⚙️ Less than X

>Q. 정수 N개로 이루어진 수열 A와 정수 X가 주어진다. 이때, A에서 X보다 작은 수를 모두 출력하는 프로그램을 작성하시오.

>Input. 첫째 줄에 N과 X가 주어진다. (1 ≤ N, X ≤ 10,000)
둘째 줄에 수열 A를 이루는 정수 N개가 주어진다. 주어지는 정수는 모두 1보다 크거나 같고, 10,000보다 작거나 같은 정수이다.

>Output. X보다 작은 수를 입력받은 순서대로 공백으로 구분해 출력한다. X보다 작은 수는 적어도 하나 존재한다.

```python
N, X, *A = map(int, open(0).read().split())
for i in A:
    i<X!=print(i, end=" ")
```

## ⚙️ Sum 4

>Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

>Input. 입력은 여러 개의 테스트 케이스로 이루어져 있다.
각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)
입력의 마지막에는 0 두 개가 들어온다.

>Output. 각 테스트 케이스마다 A+B를 출력한다.

```python
for A,_,B,_ in [*open(0)]:
    int(A)+int(B)!=0==print(f"{int(A)+int(B)}")
```

### while을 사용한 숏코드

```python
while A:=eval("+".join(input())):print(A)
```

## ⚙️ Sum 5

>Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.

>Input. 입력은 여러 개의 테스트 케이스로 이루어져 있다.
각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)

>Output. 각 테스트 케이스마다 A+B를 출력한다.

### Code 1
```python
for A,_,B,_ in [*open(0)]:
    print(int(A)+int(B))
```

### Code 2
```python
for i in open(0):
    print(sum(map(int, i.split())))
```

### Short Code
```python
for i in open(0):
    print(sum(b'%a'%i)%24)
```
- 바이트 객체를 사용한 풀이법