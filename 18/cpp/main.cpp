#include <cctype>
#include <iostream>
#include <memory>
#include <vector>

using namespace std;

using TreeNodeRef = shared_ptr<struct TreeNode>;

struct TreeNode {
	bool					is_leaf;
	uint32_t				value;
	shared_ptr<TreeNode>	left;
	shared_ptr<TreeNode>	right;

	// construction
	TreeNode(TreeNodeRef l, TreeNodeRef r) : is_leaf(false), value(0), left(l), right(r) {
	}

	TreeNode(uint32_t v) : is_leaf(true), value(v) {
	}

	bool is_regular_pair() const {
		return !is_leaf && left->is_leaf && right->is_leaf;
	}

	uint64_t magnitude() const {
		if (is_leaf) {
			return value;
		} else {
			return 3 * left->magnitude() + 2 * right->magnitude();
		}
	}

	TreeNodeRef clone() const {
		if (is_leaf) {
			return make_shared<TreeNode>(value);
		} else {
			return make_shared<TreeNode>(left->clone(), right->clone());
		}
	}
};

TreeNodeRef parse_snailfish_number(string::iterator &current) {

	if (isdigit(*current)) {
		return make_shared<TreeNode>(*current++ - '0');
	}

	current++;		// skip '['
	auto left = parse_snailfish_number(current);
	current++;		// skip ','
	auto right = parse_snailfish_number(current);
	current++;		// skip ']'

	return make_shared<TreeNode>(left, right);
}

TreeNodeRef parse_snailfish_number(string &input) {
	string::iterator chars = input.begin();
	return parse_snailfish_number(chars);
}

bool snailfish_number_explode(TreeNodeRef node, TreeNodeRef *last_regular, uint32_t *carry, int depth = 1, bool has_exploded = false) {

	if (!has_exploded && depth > 4 && node->is_regular_pair()) {

		if (*last_regular) {
			(*last_regular)->value += node->left->value;
		}
		*carry = node->right->value;

		node->is_leaf = true;
		node->value = 0;
		node->left = nullptr;
		node->right = nullptr;
		return true;
	}

	if (node->is_leaf) {
		*last_regular = node;
		node->value += *carry;
		*carry = 0;
	} else {
		has_exploded = snailfish_number_explode(node->left, last_regular, carry, depth + 1, has_exploded);
		has_exploded = snailfish_number_explode(node->right, last_regular, carry, depth + 1, has_exploded);
	}

	return has_exploded;
}

bool snailfish_number_split(TreeNodeRef node, bool has_split = false) {

	if (has_split) {
		return has_split;
	}

	if (node->is_leaf) {
		if (node->value >= 10) {
			uint32_t h = node->value / 2;
			node->left = make_shared<TreeNode>(h);
			node->right = make_shared<TreeNode>(node->value - h);
			node->is_leaf = false;
			node->value = 0;
			return true;
		}
	} else {
		has_split = has_split || snailfish_number_split(node->left, has_split);
		has_split = has_split || snailfish_number_split(node->right, has_split);
	}

	return has_split;
}

void snailfish_number_reduce(TreeNodeRef node) {

	bool keep_reducing = true;

	while (keep_reducing) {
		keep_reducing = false;

		// explode
		TreeNodeRef last_regular = nullptr;
		uint32_t carry = 0;
		keep_reducing = keep_reducing || snailfish_number_explode(node, &last_regular, &carry);

		// split
		keep_reducing = keep_reducing || snailfish_number_split(node);
	}
}

TreeNodeRef snailfish_number_add(TreeNodeRef a, TreeNodeRef b) {
	return make_shared<TreeNode>(a, b);
}

void part1(const vector<TreeNodeRef> &data) {

	auto total = data[0]->clone();

	for (size_t idx = 1; idx < data.size(); ++idx) {
		total = snailfish_number_add(total, data[idx]->clone());
		snailfish_number_reduce(total);
	}

	cout << "Part 1: magnitude = " << total->magnitude() << endl;
}

void part2(const vector<TreeNodeRef> &data) {

	uint64_t max_magnitude = 0;

	for (size_t i = 0; i < data.size(); ++i) {
		for (size_t j = 0; j < data.size(); ++j) {
			if (i == j) {
				continue;
			}

			auto n1 = snailfish_number_add(data[i]->clone(), data[j]->clone());
			snailfish_number_reduce(n1);
			max_magnitude = max(max_magnitude, n1->magnitude());

			auto n2 = snailfish_number_add(data[j]->clone(), data[i]->clone());
			snailfish_number_reduce(n2);
			max_magnitude = max(max_magnitude, n2->magnitude());
		}
	}

	cout << "Part 2: magnitude = " << max_magnitude << endl;
}

int main() {

	// parse input
	string input;
	vector<TreeNodeRef> data;

	while (getline(cin, input)) {
		data.push_back(parse_snailfish_number(input));
	}

	// solve problems
	part1(data);
	part2(data);

	return 0;
}
