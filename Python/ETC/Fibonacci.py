def fibo(n):
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
        
        print(new)
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
            tmp = square_matrix_operation(tmp, tmp)
        
        return matrix

    return matrix_judgment(n)[0][1]

num = int(input())
print(fibo(num))

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

# # 재귀함수 - 배열 사용 O => 1.11 밀리초
# fibo_arr = [0, 1]

# def fibo(n):
#     try:
#         return fibo_arr[n]
#     except IndexError:
#         fibo_arr.append(fibo(n-1) + fibo(n-2))
#         return fibo_arr[n]

# num = int(input())
# print(fibo(num))
# print(fibo_arr)

# # 재귀함수 - 배열 사용 X => 요청시간 초과
# def fibo(n):
#     return n if n<=1 else fibo(n-1) + fibo(n-2)

# num = int(input())
# print(fibo(num))

# # for문 => 137 마이크로초
# fibo = [0, 1]

# n = int(input())

# for i in range(len(fibo), n+1):
#     fibo[i%2] = fibo[(n-1)%2] + fibo[(n-2)%2]

# print(fibo[n%2])

# 람다 표현식 => 요청시간 초과
# fibo = lambda n: n if n<=1 else fibo(n-1) + fibo(n-2)

# num = int(input())
# print(fibo(num))