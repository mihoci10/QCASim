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

		void FillBuffer();
		void FlushBuffer();
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
	inline size_t BufferedStream<T>::Read(T* buffer, size_t count)
	{
		
	}

	template<class T>
	inline size_t BufferedStream<T>::Write(T* buffer, size_t count)
	{
		return size_t();
	}

	template<class T>
	inline void BufferedStream<T>::FillBuffer()
	{
		m_Stream->Seek(m_BufferPos, SeekOrigin::Begining);
		m_Buffer.Clear();

		T element;
		for (int i = 0; i < m_Buffer.Capacity(); i++)
		{
			m_Stream->Read(&element, 1);
			m_Buffer.Add(element);
		}
	}

	template<class T>
	inline void BufferedStream<T>::FlushBuffer()
	{
		m_Stream->Seek(m_BufferPos, SeekOrigin::Begining);

		for (int i = 0; i < m_Buffer.Capacity(); i++)
		{
			T element = m_Buffer.PopFront();
			m_Stream->Write(&element, 1);
		}
	}

}