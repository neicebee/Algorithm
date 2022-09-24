'''
Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.
Input. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.
Output. 각 테스트 케이스마다 "Case #x: "를 출력한 다음, A+B를 출력한다. 테스트 케이스 번호는 1부터 시작한다.
'''

import sys

T=0
for n in [*sys.stdin][1:]:
    T+=1
    print(f"Case #{T}:", sum(map(int, n.split())))

# 일반적인 코드
# T = int(input())

# for x in range(T):
#     A, B = sys.stdin.readline().rstrip().split()
#     print(f"Case #{x+1}: {int(A)+int(B)}")