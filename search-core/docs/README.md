# Search-Core 프로젝트 개요

## 빠른 시작

### 빌드 및 실행
```bash
# 개발 모드로 실행
cargo run

# 릴리즈 모드로 빌드
cargo build --release

# 릴리즈 모드로 실행
cargo run --release
```

### 사용법
```
Code_Crush v0.1.0
사용법: <pattern> [path]
명령어: help, quit

> rust .              # 현재 디렉토리에서 'rust' 검색
> hello test_data     # test_data 디렉토리에서 'hello' 검색
> help                # 도움말 표시
> quit                # 종료
```

## 프로젝트 목적

고성능 파일 검색 엔진으로, 다음 특징을 제공:
- **빠른 검색**: SIMD 가속 및 메모리 매핑
- **병렬 처리**: CPU 코어를 최대한 활용
- **실시간 결과**: 스트리밍으로 즉시 출력
- **메모리 효율**: Zero-copy I/O 및 Arc 공유

## 핵심 모듈

| 모듈 | 역할 | 주요 함수 |
|------|------|----------|
| `types.rs` | 타입 정의 | `MatchInfo`, `SearchOptions` |
| `matcher.rs` | 패턴 매칭 | `find_matches()`, `extract_line_context()` |
| `searcher.rs` | 파일 검색 | `searcher()` |
| `search_dir.rs` | 디렉토리 검색 | `search_stream()` |
| `command.rs` | 명령어 파싱 | `Command::parse()` |
| `repls.rs` | REPL 인터페이스 | `run_repl()` |

## 기술 스택

- **언어**: Rust (Edition 2024)
- **비동기**: Tokio
- **파일 순회**: ignore (ripgrep 기반)
- **메모리 매핑**: memmap2
- **SIMD 검색**: memchr/memmem
- **에러 처리**: anyhow

## 문서 구조

1. **[프로젝트 구조](./프로젝트-구조.md)**: 전체 디렉토리 및 모듈 구조
2. **[모듈 상세 분석](./모듈-상세-분석.md)**: 각 모듈의 구현 세부사항
3. **[코드 흐름 및 데이터 구조](./코드-흐름-및-데이터-구조.md)**: 실행 흐름 및 데이터 변환
4. **[의존성 및 빌드 설정](./의존성-및-빌드-설정.md)**: Cargo.toml 분석 및 빌드 전략

## 주요 기능

### ✅ 구현됨
- 단일 파일 검색
- 디렉토리 전체 검색
- 실시간 결과 출력
- UTF-8 지원 (한글 등)
- 숨김 파일 필터링
- 병렬 처리

### 🚧 미구현
- 대소문자 무시 옵션 (`case_insensitive` 플래그는 있으나 로직 미구현)
- 정규식 패턴 지원
- 파일 타입 필터링
- 결과 정렬 옵션
- 컨텍스트 라인 표시

## 성능 특징

1. **SIMD 가속**: x86-64의 SSE/AVX 명령어 활용
2. **Zero-copy I/O**: 메모리 매핑으로 복사 제거
3. **병렬 처리**: CPU 코어 수만큼 워커 스레드
4. **스트리밍**: 결과 즉시 출력으로 지연 최소화

## 테스트

```bash
# 단위 테스트
cargo test

# 통합 테스트 (예제 실행)
cd example
cargo run
```

## 코드 품질

```bash
# 포맷팅
cargo fmt

# 린팅
cargo clippy

# 경고를 에러로 처리
cargo clippy -- -D warnings
```

## 아키텍처 원칙

1. **계층 분리**: REPL → Command → Search → Matcher
2. **스트리밍**: 채널 기반 비동기 통신
3. **Zero-copy**: 메모리 매핑 활용
4. **타입 안전성**: Rust 타입 시스템 활용
5. **에러 처리**: anyhow로 명확한 에러 전파

## 관련 문서

- [프로젝트 구조](./프로젝트-구조.md): 전체 구조 및 모듈 설명
- [모듈 상세 분석](./모듈-상세-분석.md): 각 모듈의 구현 세부사항
- [코드 흐름 및 데이터 구조](./코드-흐름-및-데이터-구조.md): 실행 흐름 다이어그램
- [의존성 및 빌드 설정](./의존성-및-빌드-설정.md): Cargo.toml 분석

## 라이선스

(프로젝트 라이선스 정보)

## 기여

(기여 가이드라인)
