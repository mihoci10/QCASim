#pragma once

#include <QCACore/Utilities/Stream/IStream.hpp>
#include <QCACore/Utilities/Container/RingBuffer.hpp>

namespace QCAC {

	template <class T>
	class BufferedStream : public IStream<T> {
	public:
		BufferedStream(std::shared_ptr<IStream<T>> stream, size_t bufferCapacity = 1000);

		size_t Seek(int64_t offset, SeekOrigin origin) override;

		size_t Read(T* buffer, size_t count) override;
		size_t Write(T* buffer, size_t count) override;

	private:
		size_t m_StreamPos = 0;
		size_t m_BufferPos = 0;

		RingBuffer<T> m_Buffer;
		bool m_BufferModified = false;

		std::shared_ptr<IStream<T>> m_Stream;

		void FillBuffer(size_t bufferPosition);
		void FlushBuffer();
		bool GetPositionInBuffer(size_t position);
	};

	template<class T>
	BufferedStream<T>::BufferedStream(std::shared_ptr<IStream<T>> stream, size_t bufferCapacity) :
		m_Stream(stream),
		m_Buffer(RingBuffer<T>(bufferCapacity))
	{
	}

	template<class T>
	size_t BufferedStream<T>::Seek(int64_t offset, SeekOrigin origin)
	{
		m_StreamPos = m_Stream->Seek(offset, origin);
		return m_StreamPos;
	}

	template<class T>
	size_t BufferedStream<T>::Read(T* buffer, size_t count)
	{
		while (count > 0) {
			if (!GetPositionInBuffer(m_StreamPos))
				FillBuffer(m_StreamPos);

			size_t countIter = std::min(m_Buffer.Capacity() - m_StreamPos - m_BufferPos, count);

			m_Buffer.GetRange(buffer, m_StreamPos - m_BufferPos, countIter);

			m_StreamPos += countIter;
			buffer += countIter;
			count -= countIter;
		}

		return m_StreamPos;
	}

	template<class T>
	size_t BufferedStream<T>::Write(T* buffer, size_t count)
	{
		while (count > 0) {
			if (! GetPositionInBuffer(m_StreamPos))
				FillBuffer(m_StreamPos);
			
			size_t countIter = std::min(m_Buffer.Capacity() - m_StreamPos - m_BufferPos, count);

			m_Buffer.AddRange(buffer, m_StreamPos - m_BufferPos, countIter);
			m_BufferModified = true;

			m_StreamPos += countIter;
			buffer += countIter;
			count -= countIter;
		}

		return m_StreamPos;
	}

	template<class T>
	void BufferedStream<T>::FillBuffer(size_t bufferPosition)
	{
		if (m_BufferModified)
			FlushBuffer();

		m_BufferPos = bufferPosition;

		m_Stream->Seek(m_BufferPos, SeekOrigin::Begining);
		m_Buffer.Clear();

		size_t N = m_Buffer.Capacity();

		std::unique_ptr<T[]> elements = std::make_unique<T[]>(N);
		m_Stream->Read(elements.get(), N);

		m_Buffer.AddRange(elements.get(), 0, N);
		
		m_BufferModified = false;
	}

	template<class T>
	void BufferedStream<T>::FlushBuffer()
	{
		m_Stream->Seek(m_BufferPos, SeekOrigin::Begining);

		for (int i = 0; i < m_Buffer.Capacity(); i++)
		{
			T element = m_Buffer.PopFront();
			m_Stream->Write(&element, 1);
		}

		m_BufferModified = false;
	}

	template<class T>
	inline bool BufferedStream<T>::GetPositionInBuffer(size_t position)
	{
		return (position >= m_BufferPos) && (position <= m_BufferPos + m_Buffer.Size());
	}

}