GO := go

sequential: GOsequential.go
	$(GO) build -o GOsequential GOsequential.go

concurrent: GOconcurrent.go
	$(GO) build -o GOconcurrent GOconcurrent.go

parallel: GOparallel.go
	$(GO) build -o GOparallel GOparallel.go

clean: 
	rm -rf *.out
