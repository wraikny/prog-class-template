output/StudentNum-Name-00.pdf: saty/* saty/source.satyh
	satysfi saty/report.saty -o output/StudentNum-Name-00.pdf

convert-satyh.out: ../template/convert-satyh.rs
	rustc ../template/convert-satyh.rs -o ../template/convert-satyh.out

saty/source.satyh: source ../template/convert-satyh.out
	../template/convert-satyh.out