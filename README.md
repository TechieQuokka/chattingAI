# Gemma4 Chat

로컬 AI 채팅 앱 — Ollama + Gemma4 기반, 완전 오프라인 동작

## 아키텍처

```
[Vue3 + Tauri WebView]
      ↕ HTTP / SSE
[Rust Axum 서버 (chat-server)]
      ↕ HTTP
[Ollama]
      ↕
[gemma4:e2b / e4b]
```

## 요구사항

- **Rust** 1.77+ (`rustup` 설치)
- **Node.js** 18+ + npm
- **Ollama** 실행 중 (`ollama serve`)
- **Tauri v2** 시스템 의존성 (Ubuntu)

## Ubuntu 의존성 설치

```bash
# Tauri 시스템 의존성
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Rust 설치 (없으면)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Tauri CLI
cargo install tauri-cli --version "^2"

# Node 의존성
npm install
```

## 개발 실행

```bash
# 터미널 1: Ollama 실행
ollama serve

# 터미널 2: chat-server 직접 실행 (개발용)
cd chat-server
cargo run

# 터미널 3: Tauri 개발 모드
npm run tauri dev
```

## 빌드 (Ubuntu)

```bash
# chat-server를 Tauri sidecar로 빌드
cargo build --release -p chat-server

# sidecar 바이너리 배치
mkdir -p src-tauri/binaries
cp target/release/chat-server \
   src-tauri/binaries/chat-server-x86_64-unknown-linux-gnu

# Tauri 앱 빌드
npm run tauri build
```

빌드 결과물: `src-tauri/target/release/bundle/`

## API 테스트 (curl)

```bash
# 서버 상태
curl http://localhost:3456/health

# 모델 목록
curl http://localhost:3456/models

# 모델 로드
curl -X POST http://localhost:3456/model/load \
  -H "Content-Type: application/json" \
  -d '{"model": "gemma4:e2b"}'

# 채팅 (SSE 스트리밍)
curl -N -X POST http://localhost:3456/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "안녕하세요!", "thinking": true}'

# 히스토리 초기화
curl -X DELETE http://localhost:3456/chat/history

# 모델 언로드
curl -X POST http://localhost:3456/model/unload
```

## 환경변수

| 변수 | 기본값 | 설명 |
|------|--------|------|
| `OLLAMA_URL` | `http://localhost:11434` | Ollama 서버 주소 |
| `PORT` | `3456` | chat-server 포트 |

## Windows 빌드 (추후)

크로스컴파일 또는 Windows에서 직접 빌드:
```bash
# Windows에서
cargo build --release -p chat-server
cp target/release/chat-server.exe \
   src-tauri/binaries/chat-server-x86_64-pc-windows-msvc.exe
npm run tauri build
```
