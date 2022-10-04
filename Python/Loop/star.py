'''
Q. 첫째 줄에는 별 1개, 둘째 줄에는 별 2개, N번째 줄에는 별 N개를 찍는 문제
Input. 첫째 줄에 N(1 ≤ N ≤ 100)이 주어진다.
Output. 첫째 줄부터 N번째 줄까지 차례대로 별을 출력한다.
'''

# 내 코딩
# N=n=int(input()); exec("n-=1;print('*'*(N-n));"*N)

# 숏코딩
for i in range(int(input())): print('*'*-~i)