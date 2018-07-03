all: convert saty

saty:
	./convert-satyh.out
	satysfi saty/report.saty -o output/StudentNum-Name-00.pdf

convert:
	rustc convert-satyh.rs -o convert-satyh.out