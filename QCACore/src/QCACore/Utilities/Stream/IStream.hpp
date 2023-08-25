#pragma once

namespace QCAC{

	enum class SeekOrigin {
		Begining, Current, End
	};

	class StreamBOFException : public std::exception {
	};
	class StreamEOFException : public std::exception {
	};

	template <class T>
	class IStream {
		virtual ~IStream() {};

	public:
		virtual size_t Seek(int64_t offset, SeekOrigin origin) = 0;
		
		virtual size_t Read(T* buffer, size_t count) = 0;
		virtual size_t Write(T* buffer, size_t count) = 0;
	};

}