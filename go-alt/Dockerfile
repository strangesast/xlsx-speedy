arg tag=latest
from golang:${tag} as builder

workdir /build
copy go.* ./
run go mod download
copy main.go .
run go build -o main .


from scratch

copy --from=builder /build/main /main
cmd ["/main"]
