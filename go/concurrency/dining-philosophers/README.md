# 식사하는 철학자들 문제

## 실행

```bash
$ go run main.go
```

## 테스트

```bash
$ go test -v -race .
```

또는

```bash
$ make build
$ make test
```

## 테스트 결과

```bash
$ go test -v -race -cover .
식사하는 철학자들 문제
==================================================
라이프니츠가 자리에 앉음
아리스토텔레스가 자리에 앉음
플라톤가 자리에 앉음
헤겔가 자리에 앉음
칸트가 자리에 앉음
플라톤(이)가 생각 중
칸트(이)가 생각 중
라이프니츠(이)가 생각 중
헤겔(이)가 생각 중
아리스토텔레스(이)가 생각 중
헤겔(이)가 왼쪽 포크를 들었음
헤겔(이)가 오른쪽 포크를 들었음
헤겔(이)가 식사 중
칸트(이)가 왼쪽 포크를 들었음
라이프니츠(이)가 왼쪽 포크를 들었음
아리스토텔레스(이)가 왼쪽 포크를 들었음
라이프니츠(이)가 오른쪽 포크를 들었음
라이프니츠(이)가 식사 중
헤겔(이)가 오른쪽 포크를 내려놓음
헤겔(이)가 왼쪽 포크를 내려놓음
칸트(이)가 오른쪽 포크를 들었음
칸트(이)가 식사 중
헤겔(이)가 생각 중
헤겔(이)가 왼쪽 포크를 들었음
칸트(이)가 오른쪽 포크를 내려놓음
라이프니츠(이)가 오른쪽 포크를 내려놓음
칸트(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 왼쪽 포크를 내려놓음
아리스토텔레스(이)가 오른쪽 포크를 들었음
아리스토텔레스(이)가 식사 중
헤겔(이)가 오른쪽 포크를 들었음
헤겔(이)가 식사 중
플라톤(이)가 왼쪽 포크를 들었음
라이프니츠(이)가 생각 중
칸트(이)가 생각 중
헤겔(이)가 오른쪽 포크를 내려놓음
헤겔(이)가 왼쪽 포크를 내려놓음
칸트(이)가 왼쪽 포크를 들었음
칸트(이)가 오른쪽 포크를 들었음
아리스토텔레스(이)가 오른쪽 포크를 내려놓음
칸트(이)가 식사 중
아리스토텔레스(이)가 왼쪽 포크를 내려놓음
플라톤(이)가 오른쪽 포크를 들었음
플라톤(이)가 식사 중
헤겔(이)가 생각 중
아리스토텔레스(이)가 생각 중
칸트(이)가 오른쪽 포크를 내려놓음
칸트(이)가 왼쪽 포크를 내려놓음
헤겔(이)가 왼쪽 포크를 들었음
헤겔(이)가 오른쪽 포크를 들었음
플라톤(이)가 오른쪽 포크를 내려놓음
아리스토텔레스(이)가 왼쪽 포크를 들었음
아리스토텔레스(이)가 오른쪽 포크를 들었음
아리스토텔레스(이)가 식사 중
헤겔(이)가 식사 중
플라톤(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 왼쪽 포크를 들었음
칸트(이)가 생각 중
플라톤(이)가 생각 중
헤겔(이)가 오른쪽 포크를 내려놓음
아리스토텔레스(이)가 오른쪽 포크를 내려놓음
헤겔(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 오른쪽 포크를 들었음
아리스토텔레스(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 식사 중
칸트(이)가 왼쪽 포크를 들었음
칸트(이)가 오른쪽 포크를 들었음
칸트(이)가 식사 중
아리스토텔레스(이)가 생각 중
헤겔 식사 완료!
헤겔(이)가 자리에서 일어남
칸트(이)가 오른쪽 포크를 내려놓음
라이프니츠(이)가 오른쪽 포크를 내려놓음
칸트(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 왼쪽 포크를 내려놓음
플라톤(이)가 왼쪽 포크를 들었음
아리스토텔레스(이)가 왼쪽 포크를 들었음
아리스토텔레스(이)가 오른쪽 포크를 들었음
아리스토텔레스(이)가 식사 중
라이프니츠(이)가 생각 중
칸트 식사 완료!
아리스토텔레스(이)가 오른쪽 포크를 내려놓음
아리스토텔레스(이)가 왼쪽 포크를 내려놓음
칸트(이)가 자리에서 일어남
플라톤(이)가 오른쪽 포크를 들었음
플라톤(이)가 식사 중
아리스토텔레스 식사 완료!
아리스토텔레스(이)가 자리에서 일어남
플라톤(이)가 오른쪽 포크를 내려놓음
플라톤(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 왼쪽 포크를 들었음
라이프니츠(이)가 오른쪽 포크를 들었음
라이프니츠(이)가 식사 중
플라톤(이)가 생각 중
라이프니츠(이)가 오른쪽 포크를 내려놓음
라이프니츠(이)가 왼쪽 포크를 내려놓음
플라톤(이)가 왼쪽 포크를 들었음
플라톤(이)가 오른쪽 포크를 들었음
플라톤(이)가 식사 중
라이프니츠 식사 완료!
플라톤(이)가 오른쪽 포크를 내려놓음
플라톤(이)가 왼쪽 포크를 내려놓음
라이프니츠(이)가 자리에서 일어남
플라톤 식사 완료!
플라톤(이)가 자리에서 일어남
테이블 위의 모든 철학자들이 식사를 마쳤습니다.
==================================================
실행 시간: 24.010749958s
=== RUN   TestDiningPhilosophers
--- PASS: TestDiningPhilosophers (1.86s)
PASS
coverage: 100.0% of statements
ok      dining-philosophers     (cached)        coverage: 100.0% of statements
```