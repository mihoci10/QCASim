#include <gtest/gtest.h>

#include <QCACore/Utils/Json/JsonFile.h>

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

TEST(BasicTests, ParseBasic) {
	QCAC::Util::JsonFile f(simpleJson);
}

TEST(BasicTests, ParseComplex) {
	QCAC::Util::JsonFile f(complexJson);
}

TEST(BasicTests, ReadBasic) {
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