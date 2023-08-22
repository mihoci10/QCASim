#pragma once

#include <QCACore/Utilities/Stream/IStream.hpp>
#include <QCACore/Utilities/Container/RingBuffer.hpp>

namespace QCAC {

	template <class T>
	class BufferedStream : public IStream<T> {
	public:
		BufferedStream(size_t bufferCapacity = 1000);

		size_t Seek(int64_t offset, SeekOrigin origin) override;

		size_t Read(T* buffer, size_t count) override;
		size_t Write(T* buffer, size_t count) override;

	private:
		RingBuffer<T> m_Buffer;

	};

	template<class T>
	BufferedStream<T>::BufferedStream(size_t bufferCapacity) :
		m_Buffer(RingBuffer<T>(bufferCapacity))
	{

	}

}