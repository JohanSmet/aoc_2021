#include <iostream>

using namespace std;

int main() {

	int window[3];
	bool valid = (cin >> window[0]).good();
	valid = valid && (cin >> window[1]).good();
	valid = valid && (cin >> window[2]).good();

	if (!valid) {
		cerr << "Not enough input data\n";
		return -1;
	}

	int prev_sum = window[0] + window[1] + window[2];
	int result = 0, first = 0, input;

	while (cin >> input) {
		int sum = prev_sum - window[first] + input;
		result += (sum > prev_sum);
		window[first] = input;
		first = (first + 1) % 3;
		prev_sum = sum;
	}

	cout << result << "\n";

	return 0;
}
