* HDL program
  * Xor(exclusive or) gate
  * if x<>y out=1 else out=0
```
CHIP Xor {
    IN x, y;
    OUT out;
    PARTS:
        Not(in=x, out=notx);
        Not(in=y, out=noty);
        And(x=x, y=noty, out=w1);
        And(x=notx, y=y, out=w2);
        Or(x=w1, y=w2, out=out);
}
```

* Text script

```
load Xor.hdl
output-list x, y, out;
set x 0, set y 0,
eval, output;
set x 0, set y 1,
eval, output;
set x 1, set y 0,
eval, output;
set x 1, set y 1,
eval, output;
```

* Output file

```
x | y | out
-----------
0 | 0 | 0
0 | 1 | 1
1 | 0 | 1
1 | 1 | 0 
```

- HDL의 헤더 부분에서는 칩 인터페이스 정의
- 칩의 이름과 입력 및 출력 이름을 명시
- 파트는 해당 칩의 내부 구현 담당
- 헤더에서 명시된 입력을 각종 불 함수를 사용해 원하는 출력값을 만들어 냄

- 테스트 스크립트는 설계한 칩을 실행시킴
- 실행하고자 하는 칩을 불러와서 입력 값을 설정하고 실행함
- 실행이 끝나고 나면 입력 값의 출력 데이터를 기록한 .out 파일을 생성