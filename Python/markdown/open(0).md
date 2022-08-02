# 💻 open(0)과 sys.stdin

우리는 알고리즘 문제를 풀 때 빠른 입력을 받기 위해서 `sys.stdin.readline()`을 사용한다고 했었다.

참조: https://it-neicebee.tistory.com/115

하지만 굳이 sys 모듈을 import하여 쓸 필요가 없었다.

파일 입출력 함수인 open() 함수를 사용하면 같은 코딩을 할 수 있다.

- `open(0).read()` == `sys.stdin.read()`

## read(), readline(), readlines()

- `read()`: 파일 내용 전체를 가져와 문자열로 반환
- `readline()`: 파일의 한 줄을 가져와 문자열로 반환. 파일 포인터는 다음 줄로 이동
- `readlines()`: 파일 전체 내용을 가져와 리스트로 반환. 각 줄은 문자열 형태로 리스트 요소에 저장

## Standard Stream

- 표준 스트림: 운영체제에서 기본적으로 제공하는 추상화된 입출력 장치
  - `std`: standard 약자
  - `stdin`(표준 입력): Id는 0, 일반적으로 키보드
  - `stdout`(표준 출력): Id는 1, 일반적으로 현재 쉘을 실행한 콘솔이나 터미널
  - `stderr`(표준 에러): Id는 2, 일반적으로 표준 출력과 동일

> 즉 open()에게 0이라는 인자를 주면 stdin과 동일한 역할을 한다.