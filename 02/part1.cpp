#include <charconv>
#include <iostream>
#include <string>

using namespace std;

int main() {
	string	input;

	int pos = 0;
	int depth = 0;

	while (getline(cin, input)) {
		auto value_start = input.find(' ') + 1;

		int value = 0;
		from_chars(input.c_str() + value_start, input.c_str() + input.size(), value);

		if (input.starts_with("forward")) {
			pos += value;
		} else if (input.starts_with("down")) {
			depth += value;
		} else if (input.starts_with("up")) {
			depth -= value;
		}
	}

	cout << "Position = " << pos << " depth = " << depth << "\n";
	cout << "Result = " << pos * depth << "\n";

	return 0;
}
