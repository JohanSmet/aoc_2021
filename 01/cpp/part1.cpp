#include <iostream>

using namespace std;

int main() {

	int prev_input, input;
	int result = 0;

	cin >> prev_input;

	while (cin >> input) {
		result += (input > prev_input);
		prev_input = input;
	}

	cout << result << "\n";

	return 0;
}
