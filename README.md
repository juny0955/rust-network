# rust-network

Rust로 네트워크의 동작을 설명하고, 직접 구현하며, 장애를 진단하는 능력을 기르기 위한 실습형 커리큘럼입니다.

> 학습 흐름: **개념 이해 → 직접 구현 → 패킷·시스템 관찰 → 장애 실험 → 문서화**

## 목표

이 과정을 마치면 Socket, TCP/UDP, IP, DNS, HTTP, TLS의 관계를 설명하고, Proxy·Load Balancer를 구현하며, `tcpdump`, Wireshark, `ss`, `dig`, `ip route`로 문제를 진단할 수 있습니다.

## 진행 방식

각 단계는 구현만으로 끝내지 않습니다.

- [ ] 핵심 개념과 패킷 흐름을 설명한다.
- [ ] 결과물을 직접 실행한다.
- [ ] 정상과 실패 상황을 모두 관찰한다.
- [ ] 실행 방법, 캡처 결과, 배운 점을 해당 실습의 README에 기록한다.

## 학습 과정

| 단계 | 주제 | 대표 결과물 |
| --- | --- | --- |
| 0 | 데이터 표현과 주소 | CIDR 계산기, checksum |
| 1 | Socket과 OS 인터페이스 | TCP·UDP Echo |
| 2 | Ethernet, ARP, IP, ICMP | 패킷 파서 |
| 3~4 | UDP와 Reliable UDP | 채팅, Stop-and-Wait, Sliding Window |
| 5~7 | TCP와 Async I/O | 프레이밍 프로토콜, Tokio 서버 |
| 8~10 | DNS, HTTP, TLS | DNS Client, HTTP Server, HTTPS |
| 11~14 | Proxy와 Load Balancing | Forward/Reverse Proxy, L4/L7 Gateway |
| 15 | Linux 네트워크 | namespace, veth, bridge, NAT |
| 캡스톤 | Gateway 통합 | `rust-network-gateway` |

## 0단계. 데이터 표현과 네트워크 주소

### 학습

* bit, byte, binary, hexadecimal
* big-endian / little-endian
* 네트워크 바이트 순서
* IP 주소와 Port
* IPv4 주소 구조
* subnet mask와 CIDR
* network address, broadcast address
* public/private IP
* localhost와 loopback

### 구현

* `u16`, `u32`를 big-endian byte로 변환
* IPv4 문자열을 4바이트로 변환
* CIDR을 입력받아 네트워크 범위 계산
* 간단한 checksum 계산

```text
192.168.1.10/24
→ network: 192.168.1.0
→ broadcast: 192.168.1.255
```

### 완료 기준

* `/24`, `/16`, `/30`이 의미하는 것을 설명할 수 있다.
* 포트와 IP가 각각 무엇을 식별하는지 설명할 수 있다.
* 패킷 헤더의 바이트를 직접 읽을 수 있다.

---

## 1단계. Socket과 OS 네트워크 인터페이스

Socket은 TCP보다 아래 계층은 아니고, **애플리케이션이 커널의 TCP/UDP를 사용하는 인터페이스**다.

### 학습

* Socket이 무엇인지
* File Descriptor
* `socket`
* `bind`
* `listen`
* `accept`
* `connect`
* `read`, `write`
* `send`, `recv`
* `send_to`, `recv_from`
* socket address
* client socket과 listening socket 차이
* 커널 송수신 버퍼

### 구현

Tokio를 사용해 다음을 만든다.

* UDP Echo Server/Client
* TCP Echo Server/Client

Linux에서는 시스템 콜을 관찰한다.

```bash
strace -e trace=network ./target/debug/tcp-echo
```

### 완료 기준

다음 흐름을 설명할 수 있어야 한다.

```text
Server:
socket → bind → listen → accept → read/write → close

Client:
socket → connect → read/write → close
```

---

## 2단계. Ethernet, ARP, IP, ICMP

직접 네트워크 스택을 구현하기보다는 먼저 **패킷을 파싱하고 관찰**한다.

### 학습

* Ethernet Frame
* MAC Address
* ARP
* IPv4 Header
* Source/Destination IP
* TTL
* Protocol Number
* MTU
* IP Fragmentation
* Routing Table
* Default Gateway
* ICMP
* Ping과 Traceroute 원리

### 구현

`packet-parser`를 만든다.

```text
packet-parser/
├── ethernet.rs
├── arp.rs
├── ipv4.rs
└── icmp.rs
```

PCAP 파일 또는 캡처한 바이트에서 다음을 출력한다.

```text
Ethernet
- source MAC
- destination MAC
- ether type

IPv4
- source IP
- destination IP
- TTL
- protocol
- total length
```

### 실험

```bash
ping 8.8.8.8
traceroute 8.8.8.8
ip route
ip neigh
tcpdump -n -i any icmp
```

### 완료 기준

* 같은 네트워크와 다른 네트워크로 패킷을 보낼 때 차이를 설명한다.
* ARP와 Gateway의 역할을 설명한다.
* TTL이 Traceroute에서 어떻게 사용되는지 설명한다.

---

## 3단계. UDP

### 학습

* connectionless
* datagram
* 메시지 경계
* 패킷 유실
* 중복
* 순서 변경
* UDP checksum
* broadcast
* multicast 기초
* MTU보다 큰 데이터그램의 문제

### 구현

1. UDP Echo
2. UDP 채팅
3. Broadcast 서버 탐색
4. 패킷 유실 시뮬레이터

```text
Client --broadcast--> Server discovery
Client <--response--- Server
```

### 장애 실험

* 패킷 10% 버리기
* 패킷 순서 바꾸기
* 패킷 중복 전송
* 잘못된 패킷 크기 전송

### 완료 기준

* TCP 대신 UDP를 선택하는 상황을 설명한다.
* UDP가 빠르다는 표현이 항상 정확하지 않은 이유를 설명한다.
* UDP에서 애플리케이션이 책임져야 할 부분을 설명한다.

---

## 4단계. Reliable UDP

UDP 위에 신뢰성을 직접 추가한다.

### 1차 구현

Stop-and-Wait 방식:

```text
Sender                    Receiver
   | ---- DATA seq=1 ----> |
   | <----- ACK seq=1 ---- |
```

### 기능

* Sequence Number
* ACK
* Timeout
* Retransmission
* 중복 제거
* 재전송 횟수 제한

### 2차 구현

* Sliding Window
* 패킷 순서 재조립
* 여러 패킷 동시 전송
* RTT 측정
* timeout 동적 조정

### 완료 기준

* TCP가 sequence number와 ACK를 사용하는 이유를 설명한다.
* 손실 네트워크에서 재전송이 어떻게 동작하는지 설명한다.
* Stop-and-Wait의 성능이 낮은 이유를 설명한다.

---

## 5단계. TCP 내부 동작

### 학습

* 3-way handshake
* 4-way termination
* Sequence Number
* ACK
* TCP Flag
* SYN, ACK, FIN, RST
* Receive Window
* Flow Control
* Congestion Control 기초
* Retransmission
* RTT
* MSS와 MTU
* TIME_WAIT
* CLOSE_WAIT
* Half-close
* listen backlog
* Nagle Algorithm
* TCP keepalive

### 구현

1. TCP Echo
2. 다중 클라이언트 Echo
3. TCP 채팅
4. 파일 전송
5. 연결 상태 출력

### 필수 패킷 분석

Wireshark로 다음을 확인한다.

* SYN → SYN/ACK → ACK
* 데이터의 sequence/acknowledgement
* FIN 종료
* RST 종료
* 재전송
* Zero Window 가능 여부

### 장애 실험

* 클라이언트 프로세스 강제 종료
* 서버 프로세스 강제 종료
* 연결 중 네트워크 차단
* 데이터를 읽지 않는 클라이언트
* 느린 서버
* 동시에 수백 개 연결

### 완료 기준

* TCP 연결 생성부터 종료까지 설명한다.
* `TIME_WAIT`과 `CLOSE_WAIT`의 차이를 설명한다.
* TCP가 메시지가 아니라 스트림이라는 의미를 설명한다.
* `read()`가 요청한 크기보다 적게 반환할 수 있는 이유를 설명한다.

---

## 6단계. TCP 메시지 프레이밍

TCP 위에서 애플리케이션 메시지를 구분한다.

### 구현

Length Prefix 프로토콜:

```text
[Length: 4 bytes][Payload: N bytes]
```

이후 명령을 추가한다.

```text
[Length][Version][Command][Payload]
```

예시 명령:

```text
PING
ECHO
SEND
CLOSE
```

### 학습

* Partial Read
* Partial Write
* 메시지 합쳐짐
* 메시지 분할
* Buffer
* State Machine
* 최대 Frame 크기
* Protocol Error
* Versioning

### 필수 구현 조건

* 헤더가 나뉘어 들어와도 처리
* 여러 Frame이 한 번에 들어와도 처리
* 비정상 길이 거부
* 최대 메시지 크기 제한
* UTF-8이 아닌 바이너리 데이터 처리

### 완료 기준

* 왜 TCP에서 `read()` 한 번이 메시지 하나를 의미하지 않는지 설명한다.
* 안전한 프레임 파서를 직접 구현한다.

---

## 7단계. Async I/O와 Tokio

여기서는 Tokio 사용법뿐 아니라 내부 원리를 학습한다.

### 학습

* blocking / non-blocking
* `WouldBlock`
* readiness
* epoll
* event loop
* Future
* task
* cooperative scheduling
* cancellation
* backpressure
* timeout
* graceful shutdown

### 구현

기존 TCP 서버를 다음 구조로 개선한다.

```text
accept loop
→ connection task 생성
→ read/write
→ cancellation
→ graceful shutdown
```

### 필수 기능

* 연결 timeout
* read timeout
* idle timeout
* 동시 연결 제한
* Ctrl+C graceful shutdown
* 연결 Task 종료 대기
* 느린 클라이언트 처리

### 완료 기준

* Tokio가 스레드 하나당 연결 하나를 만들지 않고 처리할 수 있는 이유를 설명한다.
* async와 병렬 실행의 차이를 설명한다.
* backpressure가 필요한 이유를 설명한다.

---

## 8단계. DNS

### 학습

* Domain Name
* Resolver
* Recursive Query
* Iterative Query
* Root DNS
* TLD DNS
* Authoritative DNS
* A, AAAA, CNAME, NS, MX, TXT
* TTL과 Cache
* UDP 53 / TCP 53

### 구현

간단한 DNS Client를 직접 만든다.

```text
domain 입력
→ DNS Query Packet 생성
→ UDP 전송
→ DNS Response 파싱
→ IP 출력
```

구현 범위:

* Header
* Question
* A Record
* Transaction ID
* Name Compression 해석

### 비교

```bash
dig example.com
nslookup example.com
```

### 완료 기준

* 브라우저에서 도메인을 입력한 뒤 DNS가 어떻게 동작하는지 설명한다.
* DNS 캐시와 TTL을 설명한다.
* DNS가 TCP를 사용하는 경우를 설명한다.

---

## 9단계. HTTP/1.1

### 학습

* Request Line
* Status Line
* Header
* Body
* Host
* Content-Length
* Chunked Transfer Encoding
* Keep-Alive
* Pipelining 개념
* Stateless
* Cookie와 Session 기초

### 구현

1. 최소 HTTP Server
2. 최소 HTTP Client
3. HTTP Parser
4. Keep-Alive 지원
5. 정적 파일 응답

```http
GET /hello HTTP/1.1
Host: localhost
Connection: keep-alive
```

### 안전성 처리

* Header 최대 크기
* Body 최대 크기
* malformed request
* timeout
* 느린 요청
* 경로 검증

### 완료 기준

* TCP 연결과 HTTP 요청의 관계를 설명한다.
* HTTP Keep-Alive가 로드밸런싱에 주는 영향을 설명한다.
* Content-Length와 Chunked Encoding 차이를 설명한다.

---

## 10단계. TLS 기초

TLS 자체 암호화를 직접 구현하지는 않는다.

### 학습

* HTTP와 HTTPS 차이
* 대칭키와 비대칭키
* 인증서
* CA
* 공개키와 개인키
* TLS Handshake
* SNI
* ALPN
* TLS Termination

### 실습

```bash
openssl s_client -connect example.com:443
```

Rust에서는 `rustls`를 사용해 다음을 구현한다.

* TLS TCP Client
* HTTPS 요청
* 로컬 HTTPS Server

### 완료 기준

* HTTPS가 단순히 HTTP에 암호화만 추가한 것인지 정확히 설명한다.
* 인증서가 서버 신원을 어떻게 검증하는지 설명한다.
* Reverse Proxy에서 TLS termination이 무엇인지 설명한다.

---

## 11단계. Forward Proxy

```text
Client → Forward Proxy → Target Server
```

### 구현

* 목적지 주소 입력
* 대상 서버 연결
* 양방향 데이터 전달
* 접속 로그
* 차단 목록
* timeout
* 최대 연결 수

### 심화

* HTTP CONNECT
* HTTPS Tunnel
* SOCKS5 일부 구현

### 완료 기준

* Proxy와 VPN 차이를 설명한다.
* Forward Proxy를 누가 사용하는지 설명한다.
* HTTPS 트래픽을 Proxy가 볼 수 있는 경우와 없는 경우를 설명한다.

---

## 12단계. Reverse Proxy

```text
Client → Reverse Proxy → Backend
```

### 구현

* 단일 Backend 전달
* 연결 timeout
* idle timeout
* backend 실패 처리
* graceful shutdown
* access log
* active connection 집계

### 완료 기준

* Forward Proxy와 Reverse Proxy 차이를 설명한다.
* Reverse Proxy가 필요한 이유를 설명한다.
* Client 연결과 Backend 연결 생명주기를 설명한다.

---

## 13단계. L4 Load Balancer

### 구현 순서

1. Round Robin
2. Weighted Round Robin
3. Smooth Weighted Round Robin
4. Least Connections
5. Consistent Hashing

### 운영 기능

* Active Health Check
* Passive Health Check
* 장애 Backend 제외
* Backend 복구
* retry
* 최대 연결 수
* connection draining
* backend별 active connection

### 필수 실험

* 백엔드 한 대 종료
* 느린 백엔드 추가
* Keep-Alive 연결
* 긴 연결과 짧은 연결 혼합
* Round Robin 분산 결과 분석

### 완료 기준

* L4 Load Balancer가 무엇을 기준으로 분산하는지 설명한다.
* Round Robin인데 요청 수가 균등하지 않을 수 있는 이유를 설명한다.
* Least Connections 구현 시 연결 수 증가·감소 시점을 설명한다.

---

## 14단계. L7 Reverse Proxy와 Gateway

### 구현

* Host 기반 라우팅
* Path 기반 라우팅
* HTTP Load Balancing
* Header 추가 및 제거
* Rate Limiting
* Retry
* Circuit Breaker
* 요청 크기 제한
* Access Log

```text
api.local        → API Backend
static.local     → Static Backend
/admin           → Admin Backend
```

### 완료 기준

* L4와 L7 로드밸런싱 차이를 설명한다.
* HTTP retry가 위험할 수 있는 경우를 설명한다.
* Rate Limit과 Circuit Breaker 목적을 설명한다.

---

## 15단계. 실제 Linux 네트워크 구성

애플리케이션만 구현하면 네트워크 기본기가 부족할 수 있다. 반드시 포함하는 것이 좋다.

### 학습

* network interface
* routing table
* ARP table
* DNS configuration
* bridge
* veth
* network namespace
* NAT
* firewall
* port forwarding
* container networking 기초

### 실습

Network Namespace 두 개를 만든다.

```text
namespace-client
       |
      veth
       |
namespace-server
```

실습 항목:

* IP 직접 할당
* route 설정
* ping 연결
* TCP 서버 접속
* bridge 구성
* NAT 구성
* 특정 Port 차단

### 완료 기준

* Docker 컨테이너가 외부 네트워크와 통신하는 기본 구조를 설명한다.
* Routing과 NAT 차이를 설명한다.
* `localhost`가 컨테이너마다 다른 이유를 설명한다.

---

## 최종 캡스톤

### `rust-network-gateway`

```text
Client
  ↓
L4/L7 Gateway
  ├── DNS Resolution
  ├── TLS Termination
  ├── HTTP Routing
  ├── Load Balancing
  ├── Health Check
  ├── Rate Limiting
  └── Metrics
        ↓
     Backends
```

필수 기능:

* TCP Reverse Proxy
* Round Robin / Least Connections
* Health Check
* HTTP Host/Path Routing
* Timeout
* Graceful Shutdown
* Access Log
* 활성 연결 수
* 실패율
* 전송 바이트 수

---

## 권장 저장소 구성

```text
rust-network/
├── Cargo.toml
├── crates/
│   ├── byte-lab/
│   ├── socket-lab/
│   ├── packet-parser/
│   ├── udp-echo/
│   ├── reliable-udp/
│   ├── tcp-echo/
│   ├── tcp-protocol/
│   ├── dns-client/
│   ├── http-server/
│   ├── forward-proxy/
│   ├── reverse-proxy/
│   ├── l4-load-balancer/
│   └── l7-gateway/
├── captures/
├── experiments/
└── docs/
```

각 단계에는 반드시 README를 작성한다.

```text
- 이 기능이 해결하는 문제
- 핵심 개념
- 패킷 흐름
- 구현 방법
- 실패 상황
- 실행 방법
- Wireshark 캡처 결과
- 배운 내용
```

## 최종 완료 기준

이 커리큘럼을 제대로 완료하면 최소한 다음 질문에는 답할 수 있어야 한다.

* 브라우저에 URL을 입력하면 서버까지 어떤 일이 발생하는가?
* 같은 네트워크와 다른 네트워크의 패킷 전달은 어떻게 다른가?
* Socket, TCP, UDP, IP의 관계는 무엇인가?
* TCP 연결과 종료는 어떻게 이루어지는가?
* DNS는 도메인을 어떻게 IP로 변환하는가?
* HTTP Keep-Alive는 무엇인가?
* Proxy와 Reverse Proxy의 차이는 무엇인가?
* L4와 L7 Load Balancer의 차이는 무엇인가?
* TIME_WAIT, CLOSE_WAIT은 왜 발생하는가?
* 패킷이 유실되거나 서버가 느릴 때 어떤 일이 발생하는가?
* `tcpdump`, Wireshark, `ss`, `dig`, `ip route`로 문제를 어떻게 확인하는가?

여기까지 직접 구현하고 실험했다면 **“네트워크 기본은 한다” 수준을 충분히 넘는다.** 중요한 것은 기능 개수가 아니라, 각 단계에서 패킷을 직접 확인하고 장애 원인을 설명할 수 있는지다.
