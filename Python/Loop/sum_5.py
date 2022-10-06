'''
Q. 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오.
Input. 입력은 여러 개의 테스트 케이스로 이루어져 있다.
각 테스트 케이스는 한 줄로 이루어져 있으며, 각 줄에 A와 B가 주어진다. (0 < A, B < 10)
Output. 각 테스트 케이스마다 A+B를 출력한다.
'''

# for A,_,B,_ in [*open(0)]:
#     print(int(A)+int(B))

for i in open(0):
    print(sum(map(int, i.split())))

# for i in open(0):
#     print(sum(b'%a'%i))