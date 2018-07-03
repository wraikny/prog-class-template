output/StudentNum-Name-00.pdf: saty saty/source.satyh convert-satyh.out
	satysfi saty/report.saty -o output/StudentNum-Name-00.pdf

convert-satyh.out: convert-satyh.rs
	rustc convert-satyh.rs -o convert-satyh.out

saty/source.satyh: source
	./convert-satyh.out