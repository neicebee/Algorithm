import hashlib
import binascii
import sys

# 상수 정의
PASSWORD_LENGTH = 5
SALT = b'\x00' * 16  # 16바이트 모두 0으로 채워진 솔트
ITERATION_COUNT = 10000
DERIVED_KEY_LENGTH = 16 # 16바이트 (128비트)
MAGIC_STRING_HEX = "53514c69746520666f726d6174203300" # SQLite format 3.
MAGIC_STRING_BYTES = binascii.unhexlify(MAGIC_STRING_HEX) # 바이트로 변환

# XOR 연산 메서드 정의
def xor_bytes(data: bytes, key: bytes) -> bytes:
    result = bytearray(len(data))
    for i in range(len(data)):
        result[i] = data[i] ^ key[i % len(key)] # 키 길이를 초과하면 처음부터 다시 반복
    return bytes(result)

# 기존 파일 데이터 가져오기
db_file_name = 'messenger_encrypted.db' # 암호화된 DB 파일 이름
encrypted_db_content = b''
try:
    with open(db_file_name, 'rb') as f:
        encrypted_db_content = f.read()
        print(f"'{db_file_name}' 파일 ({len(encrypted_db_content)} 바이트)을 성공적으로 읽었습니다.")
except FileNotFoundError:
    print(f"'{db_file_name}' 파일을 찾을 수 없습니다.")
    sys.exit(1) # 파일이 없으면 프로그램 종료
except Exception as e:
    print(f"파일 읽기 중 오류 발생: {e}")
    sys.exit(1)
    
# 암호화된 DB 파일의 시그니처 추출
first_16_bytes_encrypted = encrypted_db_content[:DERIVED_KEY_LENGTH]

# Brute-Force Attack
print(f"\nbrute-force attack 시작: {PASSWORD_LENGTH}자리 숫자 패스워드 (00000 ~ 99999)")
found_password = None
decrypted_db_content = b''
for i in range(10**PASSWORD_LENGTH):
    password_str = str(i).zfill(PASSWORD_LENGTH)
    password_bytes = password_str.encode('utf-8') # PBKDF2는 바이트 형식의 패스워드를 받음
    # PBKDF2를 사용하여 16바이트 암호 키 생성
    derived_key = hashlib.pbkdf2_hmac(
        'sha256',                    # 해시 함수
        password_bytes,              # 패스워드 바이트
        SALT,                        # 솔트 바이트
        ITERATION_COUNT,             # 반복 횟수
        dklen=DERIVED_KEY_LENGTH     # 유도 키 길이
    )
    # 생성된 키로 암호화된 첫 16바이트를 XOR 복호화
    decrypted_first_16_bytes = xor_bytes(first_16_bytes_encrypted, derived_key)

    # 복호화된 첫 16바이트가 매직 문자열과 일치하는지 확인
    if decrypted_first_16_bytes == MAGIC_STRING_BYTES:
        found_password = password_str
        print(f"\n패스워드를 찾았습니다! 올바른 패스워드: '{found_password}'")
        print(f"생성된 암호 키 (Hex): {derived_key.hex()}")
        
        # 올바른 패스워드를 찾았으니 전체 DB 파일 복호화
        decrypted_db_content = xor_bytes(encrypted_db_content, derived_key)
        
        # 복호화된 DB 파일을 새 이름으로 저장
        decrypted_file_name = f"messenger_decrypted_{found_password}.db"
        try:
            with open(decrypted_file_name, 'wb') as f_out: # 'wb' 모드로 이진 파일 쓰기
                f_out.write(decrypted_db_content)
            print(f"복호화된 DB 파일이 '{decrypted_file_name}'으로 저장되었습니다.")
        except IOError as e:
            print(f"복호화된 파일 저장 중 오류 발생: {e}")
        break # 패스워드를 찾았으니 루프 종료
    # 진행 상황 출력
    if i % 1000 == 0:
        print(f"  시도 중: {password_str} ({i+1}/{10**PASSWORD_LENGTH})", end='\r')
if not found_password:
    print(f"\n{PASSWORD_LENGTH}자리 숫자 패스워드 범위 내에서 올바른 패스워드를 찾지 못했습니다.")
