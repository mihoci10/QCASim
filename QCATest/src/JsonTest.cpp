#include <gtest/gtest.h>

#include <QCACore/Utilities/Json/JsonFile.h>

std::string simpleJson = R"(
	  {
		"int": 3,
		"intMinus": -10,
		"float": 2.12451,
		"floatMinus": -1.05829374,
		"boolFalse": false,
		"boolTrue": true,
		"text": "asdashdjikhoihahjkasjdaij"
	  }
	)";

std::string complexJson = R"(
	  {
		"arrayNum": [1, 2, 3],
		"arrayMixed": [1, "223", true],
		"object": {
			"num": 123,
			"text": "asdsad"
		}
	  }
	)";

TEST(JSON, ParseBasic) {
	QCAC::Util::JsonFile f(simpleJson);
}

TEST(JSON, ParseComplex) {
	QCAC::Util::JsonFile f(complexJson);
}

TEST(JSON, ReadBasic) {
	QCAC::Util::JsonFile f(simpleJson);

	auto root = f.GetRootNode();
	int i;
	double d;
	bool b;
	std::string s;

	f.ReadValue(root, "int", i, 0);
	ASSERT_EQ(3, i);

	f.ReadValue(root, "intMinus", i, 0);
	ASSERT_EQ(-10, i);

	f.ReadValue(root, "missing", i, 123);
	ASSERT_EQ(123, i);

	f.ReadValue(root, "float", d, 0.0);
	ASSERT_EQ(2.12451, d);

	f.ReadValue(root, "floatMinus", d, 0.0);
	ASSERT_EQ(-1.05829374, d);

	f.ReadValue(root, "missing", d, 1.0);
	ASSERT_EQ(1.0, d);

	f.ReadValue(root, "boolFalse", b, true);
	ASSERT_FALSE(b);

	f.ReadValue(root, "boolTrue", b, false);
	ASSERT_TRUE(b);

	f.ReadValue(root, "missing", b, true);
	ASSERT_TRUE(b);

	f.ReadValue(root, "text", s, std::string("asdf"));
	ASSERT_STREQ("asdashdjikhoihahjkasjdaij", s.c_str());
}

TEST(JSON, ReadComplex) {
	QCAC::Util::JsonFile f(complexJson);

	auto root = f.GetRootNode();
	int i;
	double d;
	bool b;
	std::string s;

	auto arrayNode = f.GetChildNode(root, "arrayNum");
	ASSERT_EQ(f.GetChildCount(arrayNode), 3);
	f.ReadValue(arrayNode, 0, i, 0);
	ASSERT_EQ(i, 1);
	f.ReadValue(arrayNode, 1, i, 0);
	ASSERT_EQ(i, 2);
	f.ReadValue(arrayNode, 2, i, 0);
	ASSERT_EQ(i, 3);
	
	auto objectNode = f.GetChildNode(root, "object");
	f.ReadValue(objectNode, "num", i, 0);
	ASSERT_EQ(i, 123);
}