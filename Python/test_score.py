'''
Q. 시험 점수를 입력받아 90 ~ 100점은 A, 80 ~ 89점은 B, 70 ~ 79점은 C, 60 ~ 69점은 D, 나머지 점수는 F를 출력하는 프로그램을 작성하시오.
Input. 첫째 줄에 시험 점수가 주어진다. 시험 점수는 0보다 크거나 같고, 100보다 작거나 같은 정수이다.
Output. 시험 성적을 출력한다.
'''

import time

start = time.process_time_ns()
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
stop = time.process_time_ns()

print(f'RunTime: {stop-start:20d}ns')

""" import time

start = time.process_time_ns()
score_group = {'A':range(90, 101), 'B':range(80, 90), 
                'C':range(70, 80), 'D':range(60, 70),
                'F':range(0, 60)}

score = int(input())

for rating, score_range in score_group.items():
    if score in score_range:
        print(rating)
        break
stop = time.process_time_ns()

print(f'RunTime: {stop-start:20d}ns') """