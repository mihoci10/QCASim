#pragma once

#include <QCACore/Utilities/Stream/IStream.hpp>

#include <fstream>

namespace QCAC{

	template <class T>
	class FileStream : public IStream<T> {

	public:
		FileStream(std::unique_ptr<std::fstream> fileStream);
		FileStream(std::string filename);
		virtual ~FileStream() {};

		size_t Seek(int64_t offset, SeekOrigin origin) override;

		size_t Read(T* buffer, size_t count) override;
		size_t Write(T* buffer, size_t count) override;

	private:
		std::unique_ptr<std::fstream> m_Stream;
	};


	template<class T>
	FileStream<T>::FileStream(std::unique_ptr<std::fstream> fileStream)
		: m_Stream(fileStream)
	{
	}
	template<class T>
	FileStream<T>::FileStream(std::string filename)
		: m_Stream(std::make_unique<std::fstream>(filename.c_str(), std::fstream::in | std::fstream::out | std::fstream::trunc))
	{
	}

	template<class T>
	inline size_t FileStream<T>::Seek(int64_t offset, SeekOrigin origin)
	{
		std::ios_base::seekdir seekDir;

		switch (origin)
		{
		case QCAC::SeekOrigin::Begining:
			seekDir = std::ios_base::beg;
			break;
		case QCAC::SeekOrigin::Current:
			seekDir = std::ios_base::cur;
			break;
		case QCAC::SeekOrigin::End:
			seekDir = std::ios_base::end;
			break;
		}

		m_Stream->seekp(offset, seekDir);

		return m_Stream->tellp();
	}
	template<class T>
	inline size_t FileStream<T>::Read(T* buffer, size_t count)
	{
		m_Stream->read(reinterpret_cast<char*>(buffer), sizeof(T) * count);
		return m_Stream->tellg();
	}
	template<class T>
	inline size_t FileStream<T>::Write(T* buffer, size_t count)
	{
		m_Stream->write(reinterpret_cast<char*>(buffer), sizeof(T) * count);
		m_Stream->flush();
		return m_Stream->tellp();
	}
}