# Test Score

Q. 시험 점수를 입력받아 90 ~ 100점은 A, 80 ~ 89점은 B, 70 ~ 79점은 C, 60 ~ 69점은 D, 나머지 점수는 F를 출력하는 프로그램을 작성하시오.

Input. 첫째 줄에 시험 점수가 주어진다. 시험 점수는 0보다 크거나 같고, 100보다 작거나 같은 정수이다.

Output. 시험 성적을 출력한다.

## Think

단순 조건문과 조건들을 Group화 시키고 반복문을 사용하는 방법이 떠올랐다.

하지만 이 알고리즘 문제는 조건문을 사용하는 문제이기 때문에 정석적으로 풀었다.


```python
score = int(input())

if 90 <= score <= 100:
    print('A')
elif 80 <= score < 90:
    print('B')
elif 70 <= score < 80:
    print('C')
elif 60 <= score < 70:
    print('D')
else:
    print('F')
```

    B


**아래는 반복문을 사용해본 코드이다**


```python
score_group = {'A':range(90, 101), 'B':range(80, 90), 
                'C':range(70, 80), 'D':range(60, 70),
                'F':range(0, 60)}

score = int(input())

for rating, score_range in score_group.items():
    if score in score_range:
        print(rating)
        break
```

>비효율적이다 ㅋㅋ
