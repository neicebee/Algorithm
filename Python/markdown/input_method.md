# 💻 input()과 sys.stdin.readline() 비교

## 🍎 input()

파이썬 코딩 시 보편적으로 사용하는 입력 함수인 `input()`
- `input()`은 다음과 같은 방식으로 Prompt msg를 인자로 줄 수 있음
```python
msg = input('메시지를 입력하세요...\n>>>')
```
- `input()`의 반환값은 `rstrip()`이 기본으로 적용된 상태
- `input()`은 사용자가 입력하는 값 하나하나마다 버퍼에 저장
- 입력의 종료가 되는 기준은 개행 문자(\n)

## 🍏 sys.stdin.readline()

`input()`보다 빠르게 입력을 받을 수 있는 함수
- `sys`: 파이썬 인터프리터가 제공하는 변수와 함수를 직접 제어할 수 있게 해주는 모듈
- `sys.stdin.readline()`은 Prompt msg 인자를 줄 수 없음
- `sys.stdin.readline()`은 개행 문자를 포함한 하나의 줄로 한 번에 읽어서 버퍼에 저장
- `sys.stdin.readline()`의 기본 반환값은 `str`
- `rstrip()`이 적용되지 않은 함수이므로 개행 문자가 포함된 문자열이 반환됨

## 🍓 Conclusion

**반복되지 않는 입력에는 성능 차이가 크게 없기 때문에 둘 중 아무거나 사용해도 괜찮아보임. 하지만 입력을 반복해서 받는 상황이면 `sys.stdin.readline()`을 사용하자**

**속도가 중요한 알고리즘 문제를 풀 때는 `sys.stdin.readline()`을 애용하는 것이 좋아보인다.**