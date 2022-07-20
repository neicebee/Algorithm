'''
Q. ACM-ICPC 인터넷 예선, Regional, 그리고 World Finals까지 이미 2회씩 진출해버린 kriii는 미련을 버리지 못하고 왠지 모르게 올해에도 파주 World Finals 준비 캠프에 참여했다.
대회를 뜰 줄 모르는 지박령 kriii를 위해서 격려의 문구를 출력해주자.
Input. 입력 없음
Output. 두 줄에 걸쳐 "강한친구 대한육군"을 한 줄에 한 번씩 출력한다.
'''

'''
# print문 2번 사용
import time

start = time.process_time_ns()
print("강한친구 대한육군")
print("강한친구 대한육군")
end = time.process_time_ns()

print(f"Code_time: {end-start:20d}ns")
'''

# for문 사용
import time

start = time.process_time_ns()
for i in range(0, 2):
    print("강한친구 대한육군")
end = time.process_time_ns()

print(f"Code time: {end-start:20d}ns")