import hashlib
import binascii
import os
import sys # 프로그램 종료를 위해 사용
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
from typing import Optional # 타입 힌트

# --- 1. 암호화 파라미터 정의 ---
SALT_HEX = "c8570ac98cc615aa6a6b97b3f20f1b4"
ITERATION_COUNT = 1000
KEY_LENGTH_BITS = 128  # AES-128-CTR 키 길이 (128비트)
AES_KEY_BYTES = KEY_LENGTH_BITS // 8 # 16 바이트 (PBKDF2 유도 키 길이)
IV_LENGTH = 16 # AES CTR 모드에서 사용하는 IV 길이 (일반적으로 AES 블록 크기와 동일)

# PNG 파일 시그니처 (매직 넘버)
PNG_MAGIC_NUMBER_HEX = "89504E470D0A1A0A" # PNG 파일의 첫 8바이트
PNG_MAGIC_NUMBER_BYTES = binascii.unhexlify(PNG_MAGIC_NUMBER_HEX)

# --- 2. PBKDF2 키 파생 함수 ---
def derive_key(password: str, salt_bytes: bytes, iterations: int, key_len_bits: int) -> bytes:
    """
    PBKDF2-HMAC-SHA1을 사용하여 패스워드로부터 암호화 키를 파생합니다.
    Args:
        password: 사용자가 입력한 비밀번호 문자열.
        salt_bytes: 솔트 바이트.
        iterations: 반복 횟수.
        key_len_bits: 파생할 키의 비트 길이.
    Returns:
        파생된 암호화 키 (바이트).
    """
    password_bytes = password.encode('utf-8') # 패스워드를 바이트로 인코딩
    derived_key = hashlib.pbkdf2_hmac(
        'sha1',                      # 문제에서 주어진 해시 함수 (SHA1)
        password_bytes,              # 패스워드
        salt_bytes,                  # 솔트
        iterations,                  # 반복 횟수
        dklen=key_len_bits // 8      # 유도 키 길이 (바이트 단위)
    )
    return derived_key

# --- 3. AES-128-CTR 복호화 함수 ---
def decrypt_aes_ctr(ciphertext_with_iv: bytes, key: bytes) -> Optional[bytes]:
    """
    AES-128-CTR 알고리즘으로 데이터를 복호화합니다.
    IV (16바이트)가 암호문의 앞에 붙어 있다고 가정합니다.
    Args:
        ciphertext_with_iv: IV + 암호문 형태의 바이트 데이터.
        key: 16바이트 AES-128 암호화 키.
    Returns:
        복호화된 평문 바이트 또는 오류 시 None.
    """
    if len(ciphertext_with_iv) < IV_LENGTH:
        # IV 길이보다 짧으면 유효한 암호문이 아님
        return None

    iv = ciphertext_with_iv[:IV_LENGTH]         # 앞 16바이트를 IV로 사용
    ciphertext = ciphertext_with_iv[IV_LENGTH:]  # IV 이후가 실제 암호문

    try:
        cipher = Cipher(algorithms.AES(key), modes.CTR(iv), backend=default_backend())
        decryptor = cipher.decryptor()
        plaintext = decryptor.update(ciphertext) + decryptor.finalize()
        return plaintext
    except Exception as e:
        # 키나 IV가 올바르지 않으면 복호화 자체가 실패할 수 있음
        # 여기서는 단순히 올바른 키/IV 조합이 아니라고 간주하고 None 반환
        return None

# --- 4. 메인 로직: 비밀번호 추측 및 파일 복호화 시도 ---

# 실제 암호화된 이미지 파일 이름 목록 (예시)
# 이 파일들은 암호화된 이미지 데이터가 담겨 있어야 합니다.
encrypted_files = [
    "encrypted_photo_1.enc", # 실제 암호화된 파일명으로 변경
    "encrypted_photo_2.enc",
    "encrypted_photo_3.enc"
]

# 가상 패스워드 목록 또는 무차별 대입 생성 (이 부분이 비밀번호 유추의 핵심)
# 실제 시나리오에서는 00000부터 99999까지의 모든 숫자 조합일 수도 있습니다.
potential_passwords = [
    "password123", "secret", "qwerasdf", "userpass", "12345", 
    "photos1", "summer2024", "mypic", "hanseonguni", "sangeosong", # 임의의 비밀번호
    # --- 만약 5자리 숫자 패스워드만 시도한다면, 아래 주석을 풀고 위에 potential_passwords는 지워도 됩니다. ---
    # str(i).zfill(5) for i in range(100000) # 00000 부터 99999 까지 모든 5자리 숫자
]

found_correct_password = None
salt_bytes = binascii.unhexlify(SALT_HEX) # 16진수 솔트 문자열을 바이트로 변환

print("--- 암호화된 파일 복호화를 통한 사용자 비밀번호 추측 시작 ---")
print(f"설정: iter={ITERATION_COUNT}, salt={SALT_HEX}, key_len={KEY_LENGTH_BITS}bit (SHA1)")
print(f"검증: PNG Magic Number ({PNG_MAGIC_NUMBER_HEX})")

# 각 암호화된 파일에 대해 테스트를 반복 (어떤 파일이든 올바른 비밀번호로 복호화되면 찾음)
for filename in encrypted_files:
    if not os.path.exists(filename):
        print(f"❌ 오류: 파일 '{filename}'을(를) 찾을 수 없습니다. 다음 파일로 건너뜜.")
        continue

    print(f"\n--- 파일 '{filename}'로 비밀번호 추측 시도 중 ---")
    try:
        with open(filename, 'rb') as f:
            encrypted_data = f.read()
    except Exception as e:
        print(f"❌ 오류: 파일 '{filename}' 읽기 실패: {e}. 다음 파일로 건너뜜.")
        continue

    # 파일이 IV 길이보다 짧으면 유효하지 않음
    if len(encrypted_data) < IV_LENGTH:
        print(f"경고: 파일 '{filename}'의 크기가 IV 길이({IV_LENGTH}바이트)보다 작습니다. 건너뜜.")
        continue

    # 모든 패스워드 후보에 대해 시도
    for password_candidate in potential_passwords:
        # 1. PBKDF2로 암호 키 파생
        derived_key = derive_key(password_candidate, salt_bytes, ITERATION_COUNT, KEY_LENGTH_BITS)
        
        # 2. 파생된 키로 AES-128-CTR 복호화 시도
        decrypted_content = decrypt_aes_ctr(encrypted_data, derived_key)
        
        # 3. 복호화 결과 검증 (PNG 매직 넘버 확인)
        if decrypted_content and decrypted_content.startswith(PNG_MAGIC_NUMBER_BYTES):
            found_correct_password = password_candidate
            print(f"\n🎉 발견! 비밀번호: '{found_correct_password}'")
            print(f"   (파일: '{filename}')")
            print(f"   생성된 암호 키 (Hex): {derived_key.hex()}")
            
            # 복호화된 이미지 파일 저장 (증거 확보)
            decrypted_filename = f"decrypted_{os.path.basename(filename).replace('.enc', '')}_{found_correct_password}.png"
            try:
                with open(decrypted_filename, 'wb') as f_out:
                    f_out.write(decrypted_content)
                print(f"✅ 복호화된 이미지 파일이 '{decrypted_filename}'으로 저장되었습니다.")
            except IOError as e:
                print(f"❌ 복호화된 파일 저장 실패: {e}")
            
            # 정답 비밀번호를 찾았으니 더 이상 루프를 돌 필요 없음 (보통 모든 파일에 같은 비밀번호 사용)
            break # password_candidate 루프 종료
    
    if found_correct_password:
        break # filename 루프 종료 (다른 파일 테스트 불필요)

if not found_correct_password:
    print("\n⚠️ 제공된 패스워드 후보 목록에서 올바른 비밀번호를 찾지 못했습니다.")
    print("후보 목록을 확장하거나 다른 방식으로 비밀번호를 추측해야 합니다.")

print("\n--- 비밀번호 추측 시도 종료 ---")