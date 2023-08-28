#include <gtest/gtest.h>

#include <QCACore/Utilities/Stream/FileStream.hpp>
#include <fstream>

TEST(FileStream, CreateFromName){
	QCAC::FileStream<char> stream("test.txt");
}

TEST(FileStream, SimpleWrite) {
	QCAC::FileStream<char> stream("test.txt");

	ASSERT_EQ(stream.Write("asdf", 4), 4);
}

TEST(FileStream, SimpleWriteRead) {
	QCAC::FileStream<char> stream("test.txt");

	char buf[4];

	ASSERT_EQ(stream.Write("asdf", 4), 4);
	ASSERT_EQ(stream.Seek(0, QCAC::SeekOrigin::Begining), 0);
	ASSERT_EQ(stream.Read(buf, 4), 4);

	ASSERT_EQ(buf[0], 'a');
	ASSERT_EQ(buf[1], 's');
	ASSERT_EQ(buf[2], 'd');
	ASSERT_EQ(buf[3], 'f');
}