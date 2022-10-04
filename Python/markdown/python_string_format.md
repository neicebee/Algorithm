# 💻 파이썬 문자열 포메팅 Ver.2

https://it-neicebee.tistory.com/102

내용이 부실하여 공부할 겸 추가로 더 작성함

## `%`

- 구 방식
- 길어지면 가독성 떨어짐
- 파이썬 공식 문서에서 권장하지 않음
- C 언어의 변수 타입 지정과 비슷

```python
msg = "I like gnuykob"
p1 = 3.141592653589793238462643383

print("%25s" % msg)
print("%-25s" % msg)
print("%5s" % msg) # 공백보다 문자열이 크면 오버플로우
print("%.5s" % msg)
print("%10.5s" % msg)

print("%f" % p1)
print("%10.5f" % p1)
print("%-10.5f" % p1)
```

- 공백이 표현하고자 하는 데이터보다 클 때 기본적으로 오른쪽 정렬
- 왼쪽 정렬은 `-` 기호를 사용
- 소수점의 기본 출력은 7번째 수를 반올림하여 6번째 자리까지 출력함
- **Result**
  - ```
            I like gnuykob
    I like gnuykob           
    I like gnuykob
    I lik
        I lik
    3.141593
    3.14159
    3.14159   
    ```

## `format()`

- Python 3 포멧 방식
- 길어지면 가독성이 떨어짐

