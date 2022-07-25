# Numbers Compare

Q. 두 정수 A와 B가 주어졌을 때, A와 B를 비교하는 프로그램을 작성하시오.

Input. 첫째 줄에 A와 B가 주어진다. A와 B는 공백 한 칸으로 구분되어져 있다.

Output. 첫째 줄에 다음 세 가지 중 하나를 출력한다.
- A가 B보다 큰 경우에는 '>'를 출력한다.
- A가 B보다 작은 경우에는 '<'를 출력한다.
- A와 B가 같은 경우에는 '=='를 출력한다.

Limit. -10,000 ≤ A, B ≤ 10,000


```python
a, b = map(int, input().split())

if a > b:
    print(">")
elif a < b:
    print("<")
else:
    print("==")
```

**`split()`을 사용하면 공백을 기준으로 값을 입력할 수 있음**

아래는 sys.stdin.readline()을 사용해본 코드이다.


```python
import sys
a, b = map(int, sys.stdin.readline().rstrip().split())

if a > b:
    print(">")
elif a < b:
    print("<")
else:
    print("==")
```

input()과 sys.stdin.readline()의 차이점은 아래의 글에서 볼 수 있다.

[input()과 sys.stdin.readline() 알아보기](https://it-neicebee.tistory.com/115)
