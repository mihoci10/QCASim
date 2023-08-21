#include <gtest/gtest.h>

#include <QCACore/Utilities/Container/RingBuffer.hpp>

TEST(RingBuffer, Create) {
	QCAC::RingBuffer<int> buf;
}

TEST(RingBuffer, CreateCustom) {
	QCAC::RingBuffer<int> buf(10);
	ASSERT_EQ(buf.Capacity(), 10);
}

TEST(RingBuffer, SimpleAdd) {
	QCAC::RingBuffer<int> buf;

	buf.Add(1);
	buf.Add(2);
	buf.Add(3);

	ASSERT_EQ(buf.Size(), 3);
	ASSERT_EQ(buf[0], 1);
	ASSERT_EQ(buf[1], 2);
	ASSERT_EQ(buf[2], 3);
}

TEST(RingBuffer, SimplePop) {
	QCAC::RingBuffer<int> buf;

	buf.Add(1);
	buf.Add(2);
	buf.Add(3);

	ASSERT_EQ(buf.Size(), 3);
	ASSERT_EQ(buf.PopBack(), 3);
	ASSERT_EQ(buf.PopFront(), 1);
	ASSERT_EQ(buf.Front(), 2);
}

TEST(RingBuffer, RangeException) {
	QCAC::RingBuffer<int> buf;

	ASSERT_THROW(buf.Front(), QCAC::RingBufferRangeException);
	ASSERT_THROW(buf.Back(), QCAC::RingBufferRangeException);
}

TEST(RingBuffer, Ring) {
	QCAC::RingBuffer<int> buf(3);

	buf.Add(0);
	buf.PopFront();

	buf.Add(1);
	buf.Add(2);
	buf.Add(3);

	ASSERT_EQ(buf.Size(), 3);
	ASSERT_EQ(buf[0], 1);
	ASSERT_EQ(buf[1], 2);
	ASSERT_EQ(buf[2], 3);
}

TEST(RingBuffer, Resize) {
	QCAC::RingBuffer<int> buf;

	buf.Add(1);
	buf.Add(2);
	buf.Add(3);

	ASSERT_THROW(buf.Resize(1), QCAC::RingBufferResizeException);

	buf.Resize(5);

	ASSERT_EQ(buf.Size(), 3);
	ASSERT_EQ(buf[0], 1);
	ASSERT_EQ(buf[1], 2);
	ASSERT_EQ(buf[2], 3);
}

TEST(RingBuffer, ResizeRing) {
	QCAC::RingBuffer<int> buf(3);

	buf.Add(0);
	buf.PopFront();

	buf.Add(1);
	buf.Add(2);
	buf.Add(3);

	buf.Resize(5);

	ASSERT_EQ(buf.Size(), 3);
	ASSERT_EQ(buf[0], 1);
	ASSERT_EQ(buf[1], 2);
	ASSERT_EQ(buf[2], 3);
}