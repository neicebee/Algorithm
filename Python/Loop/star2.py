'''
Q. 첫째 줄에는 별 1개, 둘째 줄에는 별 2개, N번째 줄에는 별 N개를 찍는 문제
하지만, 오른쪽을 기준으로 정렬한 별(예제 참고)을 출력하시오.
Input. 첫째 줄에 N(1 ≤ N ≤ 100)이 주어진다.
Output. 첫째 줄부터 N번째 줄까지 차례대로 별을 출력한다.

출력예제
    *
   **
  ***
 ****
*****
'''

# N=int(input())

# for i in range(N):
#     print("{0:>{1}}".format('*'*(i+1), N))

# N=int(input())

# for i in range(N):
#     print(f"%{N}s" % ('*'*(i+1)))

N=int(input())

for i in range(N):
    print(f"{'*'*(i+1):>{N}}")