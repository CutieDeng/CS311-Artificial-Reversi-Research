librand.a: rand.o 
	ar -crv librand.a rand.o 

rand.o: rand.cpp 
	g++ -std=c++20 -static -c -Wno-return-type-c-linkage rand.cpp 

.Phony: clean run

clean: 
	-rm -rf rand.o librand.a

run: 
	cargo r --features multi-thread --release --bin main > log.txt