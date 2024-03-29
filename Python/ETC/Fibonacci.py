# # 일반항 연산
# def general_fibo(n):
#     sqrt_5 = 5 ** (1/2)
#     result = 1 / sqrt_5 * ( ((1 + sqrt_5) / 2) ** n  - ((1 - sqrt_5) / 2) ** n )
#     return int(result)

# 단순한 for문 사용
def simple_fibo(n):
    if n <= 1:
        return n
    a, b = 0, 1
    
    for _ in range(n-1):
        a, b = b, a+b
    
    return b

# 행렬 연산
def matrix_fibo(n):
    # 2*2 배열 크기 고정값
    SIZE = 2
    # fibonacci 행렬 연산의 기본값
    BASE = [[1, 1], [1, 0]]
    # 행렬의 곱셈에 대한 항등원
    IDENTITY = [[1, 0], [0, 1]]

    # 행렬의 곱셉 연산 함수
    def square_matrix_operation(a, b, size=SIZE):
        # 결과값을 담을 배열 선언 및 초기화
        new = [[0 for _ in range(size)] for _ in range(size)]
        
        for i in range(size):
            for j in range(size):
                for k in range(size):
                    new[i][j] += a[i][k] * b[k][j]
        return new

    # 행렬의 n승을 구하기 위한 판단 함수
    def matrix_judgment(n):
        # 최종 결과물
        matrix = IDENTITY.copy()
        # 임시적인 배열
        tmp = BASE.copy()
        k = 0

        # 0 이상의 정수 k에 대한 자연수 n이 2**k 를 포함하는지 판단하는 코드
        while 2 ** k <= n:
            # 비트 연산자인 &와 <<를 사용하여 포함 여부 판단
            if n & (1 << k) != 0:
                matrix = square_matrix_operation(matrix, tmp)
            k+=1
            # BASE를 통해 계속 2**k 행렬을 저장하는 코드 
            tmp = square_matrix_operation(tmp, tmp)
        
        return matrix

    return matrix_judgment(n)[0][1]

mf = matrix_fibo(72)
sf = simple_fibo(72)

print(f"gf: {mf}\nsf: {sf}\n{True}") if mf == sf else print(f"gf: {mf}\nsf: {sf}\n{False}")

# num = int(input())
# print(fibo(num))

# 예제값: 1100

# # dict을 사용한 재귀함수 => 272 마이크로초
# def fibo(n, __fibo_arr={0: 0, 1: 1}):
#     if n in __fibo_arr: return __fibo_arr[n]

#     __fibo_arr[n] = fibo(n-1)+fibo(n-2)
#     return __fibo_arr[n]

# num = int(input())
# print(fibo(num))
    
# # 재귀적 동적 계획법 => 374 마이크로초
# def fibo(n):
#     fibo_arr = [0, 1]

#     def iterator(n):
#         if n<2: return n
#         try:
#             if fibo_arr[n]: return fibo_arr[n]
#         except IndexError:
#             pass

#         fibo_arr.append(iterator(n-1)+iterator(n-2))
#         return fibo_arr[n]
    
#     return iterator(n)

# num = int(input())
# print(fibo(num))

# # 재귀함수 - 배열 사용 X => 요청시간 초과
# def fibo(n):
#     return n if n<=1 else fibo(n-1) + fibo(n-2)

# num = int(input())
# print(fibo(num))

# # for문 => 137 마이크로초
# def fibo(n):
#     fibo_arr = [0, 1]
    
#     for i in range(len(fibo_arr), n+1):
#         fibo_arr[i%2] = fibo_arr[(n-1)%2] + fibo_arr[(n-2)%2]
    
#     return fibo_arr[n%2]

# num = int(input())
# print(fibo(num))

# # 람다 표현식 => 요청시간 초과
# fibo = lambda n: n if n<=1 else fibo(n-1) + fibo(n-2)

# num = int(input())
# print(fibo(num))