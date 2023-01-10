# # 재귀함수 - 배열 사용 O
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

# # 재귀함수 - 배열 사용 X
# def fibo(n):
#     return n if n == 0 or 1 else fibo(n-1) + fibo(n-2)

# num = int(input())
# print(fibo(num))

# # for문
# fibo = [0, 1]

# n = int(input())

# for i in range(len(fibo), n+1):
#     fibo[i%2] = fibo[(n-1)%2] + fibo[(n-2)%2]

# print(fibo[n%2])