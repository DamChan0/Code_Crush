# test

## 01.03.2026

```bash
=== Search Core 테스트 ===

📄 [1] 단일 파일 검색 테스트
--------------------------------------------------
  패턴 'hello' 검색 결과: 3 개 발견

  [1] test_data/sample1.txt:1:1
      내용: hello world

  [2] test_data/sample1.txt:3:1
      내용: hello again, hello rust!

  [3] test_data/sample1.txt:3:14
      내용: hello again, hello rust!


📁 [2] 디렉토리 전체 검색 테스트
--------------------------------------------------
  패턴 'rust' 검색 결과: 7 개 발견

  [1] test_data\sample1.txt:3:20
      내용: hello again, hello rust!

  [2] test_data\sample1.txt:4:10
      내용: learning rust programming

  [3] test_data\sample1.txt:5:7
      내용: 안녕하세요 rust입니다

  [4] test_data\sample2.txt:3:1
      내용: rust rust rust - triple match test test test

  [5] test_data\sample2.txt:3:6
      내용: rust rust rust - triple match test test test

  [6] test_data\sample2.txt:3:11
      내용: rust rust rust - triple match test test test

  [7] test_data\subdir\nested.txt:4:1
      내용: rust works great here too


🔤 [3] 한글 검색 테스트
--------------------------------------------------
  패턴 '안녕' 검색 결과: 2 개 발견

  [1] test_data\sample1.txt:5:1
      내용: 안녕하세요 rust입니다

  [2] test_data\subdir\nested.txt:5:1
      내용: 안녕 from nested!


🔍 [4] 한 줄에 다중 매치 테스트
--------------------------------------------------
  패턴 'test' 검색 결과: 4 개 발견

  [1] test_data\sample1.txt:2:11
      내용: this is a test file

  [2] test_data\sample2.txt:3:31
      내용: rust rust rust - triple match test test test

  [3] test_data\sample2.txt:3:36
      내용: rust rust rust - triple match test test test

  [4] test_data\sample2.txt:3:41
      내용: rust rust rust - triple match test test test


=== 테스트 완료 ===
```