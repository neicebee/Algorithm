'''
Q. 첫째 줄에는 별 1개, 둘째 줄에는 별 2개, N번째 줄에는 별 N개를 찍는 문제
Input. 첫째 줄에 N(1 ≤ N ≤ 100)이 주어진다.
Output. 첫째 줄부터 N번째 줄까지 차례대로 별을 출력한다.
'''

# N=n=int(input()); exec("n-=1;print('*'*(N-n));"*N)

# for i in range(int(input())):
#     print(f"i: {i}\n-i: {-i}\n~i: {~i}\n-~i: {-~i}")
#     print("=======")

# num1 = 162
# num2 = ~num1
# print(bin(num2))

'''
~ 비트 반전 연산자: bitwise NOT operator
비트 NOT 연산자로서 0은 1로 1은 0으로 바꾸는 연산자.
"비트를 뒤집는다.", "비트 반전" 이라고 불리움.
-x: x의 부호를 바꾼다. x의 값이 0인 경우는 값이 변하지 않는다.
~x: x의 모든 비트를 뒤집는다. 예를 들어 x의 2진수 표현이 0000 1010일 경우 ~x는 1111 0101이 된다.
https://www.acmicpc.net/problem/16122
2의 보수
signed int와 unsigned int
'''