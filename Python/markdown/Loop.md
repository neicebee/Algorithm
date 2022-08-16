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