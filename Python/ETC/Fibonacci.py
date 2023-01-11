# 예제값: 1100

# dict을 사용한 재귀함수 => 272 마이크로초
def fibo(n, __fibo_arr={0: 0, 1: 1}):
    if n in __fibo_arr: return __fibo_arr[n]

    __fibo_arr[n] = fibo(n-1)+fibo(n-2)
    return __fibo_arr[n]

num = int(input())
print(fibo(num))
    
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