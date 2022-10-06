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

```python
msg1 = "I"
msg2 = "like"
msg3 = "gnuykob"
p1 = 3.141592653589793238462643383

# 자동 넘버링
# 넘버링 안하면 format의 인자값을 순서대로 읽음
# 인자값보다 많은 {} 사용 -> Error
# {} 보다 적은 인자값 -> Error
print("{} {} {}".format(msg1, msg2, msg3))

# 넘버링
# 넘버링을 하다가 중간에 안 할 경우 -> Error
print("{1} / {0} / {2}".format(msg1, msg2, msg3))

# * 기호로 인수 언패킹
print("{} / {} / {}".format(*msg3))

# 이름 지정 사용
print("{me} & {you}".format(me=msg1, you=msg3))

# dict 자료형 사용
my_dict = {'name': 'f1r3_r41n', \
    'job': 'developer', 'age': '22'}
print("{name} {age} {job}".format(**my_dict))

# list 자료형 사용
my_list = ['f1r3_r41n', 'developer', '22']
print("{1} / {0} / {0} / {2}".format(*my_list))

# dict & list
my_dili = [{'name': 'f1r3_r41n', \
    'job': 'developer', 'age': '22'}, \
        {'name': 'gnuykob', 'job': 'data engineer', \
            'age': '22'}]
print("{0[name]} {0[age]} {0[job]}".format(*my_dili))
print("{1[name]} {1[age]} {1[job]}".format(*my_dili))

# 기본 왼쪽 정렬
# < 왼쪽 정렬, > 오른쪽 정렬, ^ 가운데 정렬
print("{:<11}".format(msg3))
print("{:>11}".format(msg3))
print("{:^11}".format(msg3))
# - 기호로 공백 채움
print("{:-^11}".format(msg3))
# 복합
print("{0:-^{1}}".format(msg3, 22))

# 숫자 부호 표현 (소수점 이하 동일)
print("{:+d} / {:+d}".format(+21, -21)) # + 항상 부호 표시
print("{:-d} / {:-d}".format(+21, -21)) # - 음수일 경우 부호 표시
print("{:=+5d} / {:=+5d}".format(+21, -21)) # 기호만 앞으로 정렬
print("{:+05d} / {:+05d}".format(+21, -21)) # 공백 0으로 채움

# 지수 표기
print("{:e}".format(p1))
print("{:E}".format(p1))

# 백분율 표기
print("{0} / {0:%}".format(45/100))

# 의미없는 소수점 삭제
print("{}".format(3.000000)) # 0 표시
print("{:g}".format(3.000000)) # 0 삭제

# 천단위 구분
print("{:,}".format(30000000))
print("{:,.2f}".format(30000000.1234))

# 진법
print("{0:d} / {0:x} / {0:o} / {0:b}".format(79)) # dec, hex, oct, bin
print("{0:#d} / {0:#x} / {0:#o} / {0:#b}".format(79)) # 접두어 표시
# hex에서 대소문자 구분은 x, X 사용

# 유니코드
print("{0:c}{1:c}{2:c}".format(98, 101, 101))

# 각 중괄호 두 번 사용하여 중괄호 표기
print("{{ {0:c}{1:c}{2:c} }}".format(98, 101, 101))
```

- **Result**
  - ```
    I like gnuykob
    like / I / gnuykob
    g / n / u
    I & gnuykob
    f1r3_r41n 22 developer
    developer / f1r3_r41n / f1r3_r41n / 22
    f1r3_r41n 22 developer
    gnuykob 22 data engineer
    gnuykob    
        gnuykob
    gnuykob  
    --gnuykob--
    -------gnuykob--------
    +21 / -21
    21 / -21
    +  21 / -  21
    +0021 / -0021
    3.141593e+00
    3.141593E+00
    0.45 / 45.000000%
    3.0
    3
    30,000,000
    30,000,000.12
    79 / 4f / 117 / 1001111
    79 / 0x4f / 0o117 / 0b1001111
    bee
    { bee }
    ```

## `F-String`

- Python 3.6 이상 버전에서 지원
- 코드가 길어져도 직관적으로 파악 가능
- 보편적인 **_"파이썬스러운"_** 방식
- `format()` 방식과 다르게 변수명을 직접 할당하는 것이 차이점

```python
msg1 = "I"
msg2 = "like"
msg3 = "gnuykob"
p1 = 3.141592653589793238462643383

print(f"{msg1} {msg2} {msg3}")

# dict 자료형 사용
my_dict = {'name': 'f1r3_r41n', \
    'job': 'developer', 'age': '22'}
print(f"He's name: {my_dict['name']}")

# list 자료형 사용
my_list = ['f1r3_r41n', 'developer', '22']
print(f"He's age: {my_list[2]}")

# dict & list
my_dili = [{'name': 'f1r3_r41n', \
    'job': 'developer', 'age': '22'}, \
        {'name': 'gnuykob', 'job': 'data engineer', \
            'age': '22'}]
print(f"He's name: {my_dili[0]['name']}")
print(f"She's name: {my_dili[1]['name']}")

# 기본 왼쪽 정렬
# < 왼쪽 정렬, > 오른쪽 정렬, ^ 가운데 정렬
print(f"{msg3:<11}")
print(f"{msg3:>11}")
print(f"{msg3:^11}")
# - 기호로 공백 채움
print(f"{msg3:-^11}")
# 복합
print(f"{msg3:-^{22}}")

# 숫자 부호 표현 (소수점 이하 동일)
p_num = 21
m_num = -21
print(f"{p_num:+d} / {m_num:+d}") # + 항상 부호 표시
print(f"{p_num:-d} / {m_num:-d}") # - 음수일 경우 부호 표시
print(f"{p_num:=+5d} / {m_num:=+5d}") # 기호만 앞으로 정렬
print(f"{p_num:+05d} / {m_num:+05d}") # 공백 0으로 채움

# 지수 표기
print(f"{p1:e}")
print(f"{p1:E}")

# 백분율 표기
print(f"{45/100:%}")

# 의미없는 소수점 삭제
num3 = 3.000000
print(f"{num3}") # 0 표시
print(f"{num3:g}") # 0 삭제

# 천단위 구분
num4 = 3000000000
num5 = 3000000000.1234
print(f"{num4:,}")
print(f"{num5:,.2f}")

# 진법
num6 = 79
print(f"{num6:d} / {num6:x} / {num6:o} / {num6:b}") # dec, hex, oct, bin
print(f"{num6:#d} / {num6:#x} / {num6:#o} / {num6:#b}") # 접두어 표시
# hex에서 대소문자 구분은 x, X 사용

# 유니코드
print(f"{98:c}{101:c}{101:c}")

# 각 중괄호 두 번 사용하여 중괄호 표기
print(f"{{ {98:c}{101:c}{101:c} }}")
```

- **Result**
  - ```
    I like gnuykob
    He's name: f1r3_r41n
    He's age: 22
    He's name: f1r3_r41n
    She's name: gnuykob
    gnuykob    
        gnuykob
    gnuykob  
    --gnuykob--
    -------gnuykob--------
    +21 / -21
    21 / -21
    +  21 / -  21
    +0021 / -0021
    3.141593e+00
    3.141593E+00
    45.000000%
    3.0
    3
    3,000,000,000
    3,000,000,000.12
    79 / 4f / 117 / 1001111
    79 / 0x4f / 0o117 / 0b1001111
    bee
    { bee }
    ```
