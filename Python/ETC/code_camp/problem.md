# 💻 Hansung Code Camp Problems

## Problem 1 - 디지털 포렌식 문제

### 문제 정보
- 메신저 실행 시 **사용자가 입력하는 숫자(0~9)로 이루어진 5자리 패스워드**
- **PBKDF2-HMAC-SHA256 방식을 통해 16바이트의 암호 키 생성**
  - 키 생성 알고리즘: PBKDF2-HMAC-SHA256(Password, Salt=0x00*16, IterationCount=10,000, DerivedKeyLength=16)
- 평문을 암호 키와 **16바이트 단위의 XOR 연산**을 통해 암/복호화 설계
- 올바른 암호 키로 첫 16바이트 복호화 시 다음의 평문 정보 식별
  - `53 51 4C 69 74 65 20 66 6F 72 6D 61 74 20 33 00 : SQLite format 3.`

<br>

### 문제 풀이 포인트
- 0~9 숫자로 이루어진 5자리 패스워드이기 때문에 Brute-Force Attack이 가능함
- 파이썬에서의 PBKDF2-HMAC-SHA256 방식의 메서드는 `hashlib` 모듈의 `pbkdf2_hmac` 메서드 사용
- 생성 알고리즘의 정보들은 상수로 설정
- 생성된 암호 키로 평문을 XOR 연산하는 메서드 따로 구현
- 파일 전체가 아닌 앞 부분의 시그니처 복호화 후 sqlite3 파일의 시그니처와 일치하는 지 확인
- 일치하면 비밀번호와 복호화된 파일 생성

<br>

### 결과

![code_result](code_result.png)

![db_result](db_result.png)

## Problem 2 - 디지털 포렌식 문제
- apk 파일 분석 후 암호화된 사진을 복호화하여 증거 확보
- 사용자 비밀번호 입력 후, 해당 암호 기반으로 키를 파생하여 파일 AES 암호화
  - 별도 난독화 존재 X
- AES 키 파생 알고리즘: PBDKF2-HMAC-SHA1
    - 파생 키로 AES-128-CTR 알고리즘으로 암호화 수행

<br>

### 문제 풀이 포인트

![crypto_aes](crypto_aes.png)
- iter: 1,000
- salt: `c8570ac98cc615aa6a6b97b3f20f1b4`
- png 파일의 시그니처는 `89504E47 0D0A1A0A`
- 고정된 salt 값과 인코딩된 파일의 IV가 계속 반복되는 키 재사용성이라는 취약점 때문에 XOR 연산으로 사용자의 비밀번호를 알아낼 수 있음

<br>

## Problem 3 - 머신러닝 기반 암호화 트래픽 분석
- 암호화 트래픽을 인공지능 모델의 입력으로 사용하기 적절하도록 전처리 수행
  - 전처리 방법론에 따라 제시된 pcap 파일을 전처리(flow 추출, 특징 구성, 정규화)를 수행하여 머신러닝 모델에 학습시킬 수 있도록 데이터셋 구성
- 전처리 방법론
  - 네트워크 트래픽 데이터는 일반적으로 .pcap 파일 포맷으로 저장되어 있으며, 여러 flow가 혼재되어 있음
  - Flow-level 특징을 추출하기 위해 트래픽 데이터를 Flow 별로 분할
    - 5-Tuple(Source IP, Destination IP, Source Port, Destination Port, Protocol)을 기준으로 분할
    - 모델의 입력으로 사용하기 위해서는 데이터셋을 고정된 길이, 동일한 형식으로 정규화 필요
      - Standard Scaler
      - Min-Max Scaler
- 머신러닝 모델의 입력으로 사용하기 위해 다음과 같은 단계로 암호화 트래픽 데이터 전처리를 수행했습니다.
- pcap 파일 로드: 제공된 '/content/benign.pcap' 및 '/content/malware.pcap' 파일을 Scapy 라이브러리를 사용하여 로드했습니다. 로드된 총 패킷 수는 **{len(packets)}**개 입니다. (정상: {len(benign_packets)}개, 악성: {len(malware_packets)}개)
- 흐름 추출 및 특징 구성: 로드된 패킷에서 데이터 흐름(flow)을 식별하고 각 흐름에 대한 특징을 추출했습니다. 총 **{len(flows)}**개의 흐름이 식별되었으며, 각 흐름에 대해 흐름 특징과 레이블(flow_label)을 할당했습니다.
- 데이터 구조화: 추출된 흐름 특징들은 pandas DataFrame (flows_df) 형태로 구조화되었습니다. 이 DataFrame은 총 **{flows_df.shape[0]}**개의 행(흐름)과 **{flows_df.shape[1]}**개의 열(특징 및 레이블)으로 구성됩니다.
- 데이터 정규화: 숫자형 특징들에 대해 MinMaxScaler를 사용하여 정규화를 수행했습니다.
- 데이터셋 분할: 정규화된 데이터셋은 학습 세트와 테스트 세트로 분할되었습니다. 학습 세트는 {X_train.shape}, 테스트 세트는 **{X_test.shape}**의 형태를 가집니다. 레이블(y_train, y_test)도 각각 {y_train.shape}, {y_test.shape} 형태입니다.

<br>

## Problem 4 - 머신러닝 기반 암호화 트래픽 분석
- 전처리된 암호화 트래픽 데이터를 머신러닝 모델을 사용하여 악성 트래픽인지 또는 정상 트래픽인지를 분류
  - Random Forest, Logistic Regression 모델을 사용하여 전처리한 데이터셋을 학습시켜 성능을 확인
- 전처리된 데이터셋을 사용하여 Random Forest 및 Logistic Regression 모델을 학습시키고 악성 트래픽 분류 성능을 평가했습니다.
- Random Forest 모델: 학습 결과, 테스트 데이터에 대한 성능은 **정확도 1.0000과 F1 점수 1.0000**을 기록했습니다.
- Logistic Regression 모델: 학습 결과, 테스트 데이터에 대한 성능은 **정확도:1.0000과 F1 점수 1.0000**을 기록했습니다.


<br>

## Problem 5 - 암호화
- AES로 암호화된 데이터를 복구하라
- 적절한 암호화 방식을 선택하라
```python
# cipher = AES.new(key, AES.MODE_CFB, iv=iv)
# cipher = AES.new(key, AES.MODE_OFB, iv=iv)
# cipher = AES.new(key, AES.MODE_CTR, nonce=nonce)  # ← 여기!
```
  - 패딩이 필요 없고, 스트림 방식(키스트림과 XOR, "counter increases")을 사용한다는 점에서 CTR (Counter) 모드가 정답입니다.
- 암호화 키값을 유추하시오.
  - 최종 키 문자열 codingcryptogra#

## Problem 6 - 암호화
- Low Public Exponent Attack을 RSA에 적용하여 해킹하시오.
  - 적절한 e 값 대입하여 복호화
  
![1](KakaoTalk_Photo_2025-09-03-17-28-38.png)
![2](KakaoTalk_Photo_2025-09-03-17-28-44.png)
  - 발견된 키: e = 17
  - 복호화된 메시지: "camp2025"