# fibo_arr = [0, 1]

# def fibo(n):
#     try:
#         print(f"fibo: {n}")
#         return fibo_arr[n]
#     except:
#         print(f"Repeat!! {n}")
#         return fibo(n-1) + fibo(n-2)

# num = int(input())
# print(fibo(num))

fibo_arr = [0, 1, 1]

n = int(input())

for i in range(len(fibo_arr), n+1):
    print(fibo_arr)
    fibo_arr[i%3] = fibo_arr[(i-1)%3] + fibo_arr[(i-2)%3]

print(fibo_arr[n%3])

'''
int byLoop(int n){
    int f[3] = {0,1,1};
    for(int i = 3; i <= n; i++){
        f[i%3] = f[(i-1)%3] + f[(i-2)%3];
    }
    return f[n%3];
}
'''