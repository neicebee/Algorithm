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