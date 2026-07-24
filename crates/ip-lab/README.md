# IP Lab

IPv4 CIDR 입력을 받아 subnet mask, network address, broadcast address를 계산하는 실습이다.

## 학습 목표

- CIDR prefix length가 network bit 수를 뜻한다는 것을 이해한다.
- IPv4 주소를 네 개의 octet으로 파싱한다.
- bitwise AND와 OR로 network address와 broadcast address를 계산한다.

## 실행 방법

저장소 루트에서 실행한다.

```bash
cargo run -p ip-lab
```

CIDR 형식의 IPv4 주소를 한 줄로 입력한다.

```text
192.168.1.10/24
```

## 실행 예시

입력:

```text
192.168.1.10/24
```

출력:

```text
input: 192.168.1.10/24
subnet_mask: 255.255.255.0
network_address: 192.168.1.0
broadcast_address: 192.168.1.255
```

`/24`는 앞의 24비트가 network part임을 의미한다.

```text
IP address:    192.168.1.10
subnet mask:   255.255.255.0
network:       192.168.1.0
broadcast:     192.168.1.255
```

`network address`는 IP와 subnet mask를 bitwise AND하여 계산한다.

```text
network = ip & subnet_mask
```

`broadcast address`는 host bit를 모두 1로 만들어 계산한다.

```text
broadcast = ip | !subnet_mask
```

## 실패 입력 예시

IPv4 octet은 `0..=255` 범위를 벗어날 수 없다.

입력:

```text
300.0.0.1/24
```

출력:

```text
InvalidIpAddress
```

prefix length는 `0..=32` 범위여야 한다.

입력:

```text
10.0.0.1/33
```

출력:

```text
InvalidPrefixLength(33)
```

## 검증

```bash
cargo test -p ip-lab
```

현재 테스트는 CIDR 파싱과 prefix 경계값, subnet mask, network address, broadcast address 계산을 검증한다.

## 현재 범위

- IPv4 CIDR만 지원한다.
- host 수나 usable host 범위는 아직 계산하지 않는다.
