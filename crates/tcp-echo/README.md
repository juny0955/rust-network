# TCP Echo
TCP 연결에서 `\n` 단위의 메세지를 프레이밍해 돌려주는 Echo 통신 구현

## 핵심 개념
- TCP는 연결형 프로토콜이다
- 서버는 `bind -> listen -> accept` 순서로 연결을 수락한다
- `TcpStream`은 한 클라이언트와의 연결을 나타낸다
- TCP는 `read()`와 `write()`의 메세지 경계를 보존하지않아 프레이밍이 필요하다

