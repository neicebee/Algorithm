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



