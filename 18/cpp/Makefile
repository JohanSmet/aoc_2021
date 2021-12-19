all: solution

solution: main.o
	$(CXX) $(LDFLAGS) -o $@ $<

clean:
	$(RM) *.o solution

@phony: clean all
