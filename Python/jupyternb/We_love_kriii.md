# We Love Kriii

Q. ACM-ICPC 인터넷 예선, Regional, 그리고 World Finals까지 이미 2회씩 진출해버린 kriii는 미련을 버리지 못하고 왠지 모르게 올해에도 파주 World Finals 준비 캠프에 참여했다. 대회를 뜰 줄 모르는 지박령 kriii를 위해서 격려의 문구를 출력해주자.

Input. 입력 없음

Output. 두 줄에 걸쳐 "강한친구 대한육군"을 한 줄에 한 번씩 출력

## Think

print문을 두 번 사용하는 것과 for문을 사용하는 두 가지 방법이 떠올랐다.

print문을 두 번 사용하는 것이 for문을 사용하는 것보다 훨씬 빠르지 않을까?


```python
print("강한친구 대한육군")
print("강한친구 대한육군")
```

    강한친구 대한육군
    강한친구 대한육군


![print_code](https://raw.githubusercontent.com/neicebee/Algorithm/main/Python/Photo/wlk_print_code.png)

![print_result](https://raw.githubusercontent.com/neicebee/Algorithm/main/Python/Photo/wlk_print_result.png)

`time.process_time_ns()`를 사용하여 보는 코드의 시간은 14000ns에서 21000ns 사이


```python
for i in range(0, 2):
    print("강한친구 대한육군")
```

    강한친구 대한육군
    강한친구 대한육군


![for_code](https://raw.githubusercontent.com/neicebee/Algorithm/main/Python/Photo/wlk_for_code.png)

![for_result](https://raw.githubusercontent.com/neicebee/Algorithm/main/Python/Photo/wlk_for_result.png)

for문을 사용한 코드는 16000ns에서 25000ns 사이로 나온다.

미세한 차이이지만 `print()`를 사용하는 것이 빠르다.

# 👂🏻 ADD TIP - 코드의 실행시간

## 🕰 Time Module

1. time.time()
   - UTC 시간인 1970/01/01 자정부터 현재까지 흐른 시간을 반환
   - 코드의 실행시간을 구하는 가장 보편적인 함수
   - `time.time_ns()`: 1초 미만의 실행시간을 나노초 단위로 반환
2. time.perf_counter()
   - 성능 카운터(Performance Counter)
   - 짧은 지속시간을 측정하는 가장 높은 해상도를 가진 시계의 값`(소수부를 가진 초)`을 반환
   - `time.perf_counter_ns()`: 1초 미만의 실행시간을 나노초 단위로 변환
3. time.process_time()
   - 현재 프로세스의 시스템과 사용자 CPU 시간 합계의 값`(소수부를 가진 초)`을 반환
   - 수면 중 경과시간은 포함하지 않음
   - CPU가 실제로 해당 프로세스에 대해 작업한 시간만을 측정
   - `time.process_time_ns()`: 1초 미만의 실행시간을 나노초 단위로 변환

>실제로 코드의 연산 시간만 비교한다면 `process_time()`, 전체적인 프로그램이 돌아가는 시간을 비교한다면 `perf_counter()`