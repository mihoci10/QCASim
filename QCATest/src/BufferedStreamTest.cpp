#include <gtest/gtest.h>

#include <QCACore/Utilities/Stream/FileStream.hpp>
#include <QCACore/Utilities/Stream/BufferedStream.hpp>
#include <fstream>

TEST(BufferedStream, Create){
	std::shared_ptr<QCAC::FileStream<char>> stream = 
		std::make_shared<QCAC::FileStream<char>>("test.txt");
	QCAC::BufferedStream<char> bufStream(stream);
}

TEST(BufferedStream, SimpleWriteRead) {
	std::shared_ptr<QCAC::FileStream<char>> stream =
		std::make_shared<QCAC::FileStream<char>>("test.txt");
	QCAC::BufferedStream<char> bufStream(stream, 4);

	char buf[4];

	ASSERT_EQ(bufStream.Write("asdf", 4), 4);
	bufStream.Commit();
	ASSERT_EQ(bufStream.Seek(0, QCAC::SeekOrigin::Begining), 0);
	ASSERT_EQ(bufStream.Read(buf, 4), 4);

	ASSERT_EQ(buf[0], 'a');
	ASSERT_EQ(buf[1], 's');
	ASSERT_EQ(buf[2], 'd');
	ASSERT_EQ(buf[3], 'f');
}

TEST(BufferedStream, ContinuousWriteRead) {
	QCAC::FileStream<char> stream("test.txt");

	char buf[8];

	ASSERT_EQ(stream.Write("asdf", 4), 4);
	ASSERT_EQ(stream.Write("1234", 4), 8);
	ASSERT_EQ(stream.Seek(0, QCAC::SeekOrigin::Begining), 0);
	ASSERT_EQ(stream.Read(buf, 8), 8);

	ASSERT_EQ(buf[0], 'a');
	ASSERT_EQ(buf[1], 's');
	ASSERT_EQ(buf[2], 'd');
	ASSERT_EQ(buf[3], 'f');
	ASSERT_EQ(buf[4], '1');
	ASSERT_EQ(buf[5], '2');
	ASSERT_EQ(buf[6], '3');
	ASSERT_EQ(buf[7], '4');
}